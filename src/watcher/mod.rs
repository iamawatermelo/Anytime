//! File watcher impl

use std::collections::{BinaryHeap, HashSet};
use std::result;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use crossbeam::select;
use crossbeam_channel::{Receiver, Sender};
use ignore::{WalkBuilder, gitignore::Gitignore, overrides::{Override, OverrideBuilder}};
use log::{error, trace, warn};
use notify_debouncer_full::notify::event::{CreateKind, DataChange, ModifyKind};
use notify_debouncer_full::notify::{EventKind, PollWatcher, RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, FileIdMap, RecommendedCache, new_debouncer, new_debouncer_opt, notify};

use crate::watcher::file_filter::directories_to_watch;
use crate::watcher::watcher_wrapper::WatcherWrapper;
use crate::{config::{ProjectConfig, UserConfig}};

mod watcher_wrapper;
mod file_filter;

#[derive(Debug)]
pub enum WatchMessage {
    /// Batch of Watcher messages
    Files(Vec<PathBuf>),
    
    /// Time went forwards
    TimeForward,
    
    /// Time went backwards
    TimeBackward,
    
    /// An error occoured while watching
    WatchError(anyhow::Error)
}

#[derive(Debug)]
pub struct FileWatcher<'a> {
    shutdown_sender: Sender<()>,
    join_handle: Option<JoinHandle<()>>,
    project_config: &'a ProjectConfig,
    user_config: &'a UserConfig
}

impl<'a> Drop for FileWatcher<'a> {
    fn drop(&mut self) {
        // don't care if the channel is disconnected, means that the thread
        // already died
        self.shutdown_sender.send(()).ok();
    }
}

impl<'a> FileWatcher<'a> {
    pub fn new(
        project_directory: PathBuf,
        project_config: &'a ProjectConfig,
        user_config: &'a UserConfig,
        result_sender: Sender<WatchMessage>
    ) -> Result<Self> {
        let (tx, watcher_receiver) = crossbeam_channel::unbounded();
        let (shutdown_sender, shutdown_receiver) = crossbeam_channel::bounded(0);
        let watcher = match user_config.use_polling {
            true => WatcherWrapper::PollWatcher(new_debouncer_opt(
                user_config.rate_limit,
                Some(user_config.poll_frequency),
                tx,
                RecommendedCache::new(),
                notify::Config::default()
                    .with_follow_symlinks(false)
            )?),
            false => WatcherWrapper::PollWatcher(new_debouncer_opt(
                user_config.event_debounce_timeout,
                Some(user_config.poll_frequency),
                tx,
                RecommendedCache::new(),
                notify::Config::default()
                    .with_follow_symlinks(false)
                    .with_poll_interval(user_config.poll_frequency)
            )?)
        };
        
        let (project_config_ref, user_config_ref) = unsafe {
            // SAFETY: We must ensure that the reference outlives this struct.
            // We do this by impl'ing Drop for FileWatcher that first ensures
            // the worker has shut down.
            
            std::mem::transmute((project_config, user_config))
        };
        
        let join_handle = std::thread::spawn(move || match Self::worker(
            project_directory,
            watcher,
            result_sender.clone(),
            watcher_receiver,
            shutdown_receiver,
            project_config_ref,
            user_config_ref
        ) {
            Err(e) => result_sender.send(WatchMessage::WatchError(e)).unwrap(),
            Ok(_) => return
        });
        
        Ok(Self {
            shutdown_sender,
            join_handle: Some(join_handle),
            project_config,
            user_config
        })
    }
    
    fn worker(
        project_directory: PathBuf,
        mut watcher: WatcherWrapper,
        result_sender: Sender<WatchMessage>,
        watcher_receiver: Receiver<DebounceEventResult>,
        shutdown_receiver: Receiver<()>,
        project_config: &'a ProjectConfig,
        user_config: &'a UserConfig
    ) -> Result<()> {
        let mut watched_paths = directories_to_watch(project_directory.as_path(), project_config);
        
        for directory in watched_paths.iter() {
            watcher.watch(directory, RecursiveMode::NonRecursive)?;
        }
        
        loop {
            let start_systime = SystemTime::now();
            
            macro_rules! check_systime {
                () => {
                    match start_systime.duration_since(SystemTime::now()) {
                        Ok(d) => {
                            if d > (user_config.poll_frequency * 2) {
                                warn!("time skipped forward {}s", d.as_secs());
                                result_sender.send(WatchMessage::TimeForward)?;
                            }
                        },
                        Err(e) => {
                            warn!("time skipped backward {}s", e.duration().as_secs());
                            result_sender.send(WatchMessage::TimeBackward)?;
                        }
                    }
                }
            }
            
            let event = select! {
                recv(watcher_receiver) -> event => event,
                recv(shutdown_receiver) -> shutdown => {
                    trace!("recv shutdown event {shutdown:#?}");
                    return shutdown.context("failed to recv shutdown event").into()
                },
                default(user_config.poll_frequency) => {
                    check_systime!();
                    continue
                }
            }?;
            
            check_systime!();
            
            let mut should_rebuild = false;
            let ignore_paths = project_config.ignorefiles
                .iter()
                .filter_map(|f| Path::new(f.as_str()).canonicalize().ok())
                .collect::<HashSet<_>>();
            
            match event {
                Ok(v) => result_sender.send(WatchMessage::Files(
                    v.into_iter()
                        .filter_map(|ev| {
                            // i'm lazy: if anything needs rescanning recompute
                            // directory set
                            if ev.need_rescan() {
                                should_rebuild = true;
                            }
                            
                            // don't care about accesses
                            if let EventKind::Access(..) = ev.kind {
                                return None
                            }
                            
                            // check for ignore paths changed -- should rebuild config
                            for path in ev.paths.iter() {
                                let Ok(canon_path) = path.canonicalize() else {
                                    warn!("cannot canonicalize {} (deleted?)", path.display());
                                    continue
                                };
                                
                                if ignore_paths.contains(&canon_path) {
                                    should_rebuild = true;
                                }
                            }
                            
                            // otherwise
                            match ev.kind {
                                EventKind::Create(CreateKind::File) => Some(ev.paths[0].clone()),
                                EventKind::Modify(ModifyKind::Data(..)) => Some(ev.paths[0].clone()),
                                _ => None
                            }
                        })
                        .collect()
                ))?,
                Err(e) => for error in e {
                    error!("Watch error: {error}")
                }
            }
            
            if should_rebuild {
                let new_watched_paths = directories_to_watch(project_directory.as_path(), project_config);
                
                // things to unwatch
                for path in watched_paths.difference(&new_watched_paths) {
                    watcher.unwatch(path);
                }
                
                // things to watch
                for path in new_watched_paths.difference(&watched_paths) {
                    watcher.watch(path, RecursiveMode::NonRecursive);
                }
                
                watched_paths = new_watched_paths;
            }
        }
    }
}