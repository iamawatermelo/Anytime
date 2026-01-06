//! File watcher impl

use std::collections::HashSet;
use std::result;
use std::time::Duration;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use crossbeam::select;
use crossbeam_channel::{Receiver, Sender};
use ignore::{WalkBuilder, gitignore::Gitignore, overrides::{Override, OverrideBuilder}};
use log::{trace, warn};
use notify_debouncer_full::notify::{PollWatcher, RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, FileIdMap, RecommendedCache, new_debouncer, new_debouncer_opt, notify};

use crate::{config::{ProjectConfig, UserConfig}};

#[derive(Debug)]
pub enum WatchMessage {
    /// Batch of Watcher messages
    Files(Vec<PathBuf>),
    
    /// Time skipped while the Watcher was running
    TimeSkipped(Duration),
    
    /// An error occoured while watching
    WatchError(anyhow::Error)
}

#[derive(Debug)]
pub struct FileWatcher {
    watcher: WatcherWrapper,
    watched_paths: HashSet<PathBuf>,
    shutdown_sender: Sender<()>
}

// this is stupid
#[derive(Debug)]
enum WatcherWrapper {
    PlatformWatcher(Debouncer<RecommendedWatcher, RecommendedCache>),
    PollWatcher(Debouncer<PollWatcher, RecommendedCache>)
}

// this is genuinely so dumb i hate rust
impl WatcherWrapper {
    pub fn watch(
        &mut self,
        path: impl AsRef<Path>,
        recursive_mode: RecursiveMode,
    ) -> notify::Result<()> {
        trace!("watching {:#?} with mode {recursive_mode:#?}", path.as_ref());
        
        match self {
            Self::PlatformWatcher(w) => w.watch(path, recursive_mode),
            Self::PollWatcher(w) => w.watch(path, recursive_mode)
        }
    }
    
    pub fn unwatch(&mut self, path: impl AsRef<Path>) -> notify::Result<()> {
        trace!("unwatching {:#?}", path.as_ref());
        
        match self {
            Self::PlatformWatcher(w) => w.unwatch(path),
            Self::PollWatcher(w) => w.unwatch(path)
        }
    }
    
    pub fn stop(self) {
        trace!("stopping");
        
        match self {
            Self::PlatformWatcher(w) => w.stop(),
            Self::PollWatcher(w) => w.stop()
        }
    }
}

impl FileWatcher {
    pub fn new(
        project_directory: &Path,
        project_config: &ProjectConfig,
        user_config: &UserConfig,
        result_sender: Sender<WatchMessage>
    ) -> Result<Self> {
        let (tx, watcher_receiver) = crossbeam_channel::unbounded();
        let (shutdown_sender, shutdown_receiver) = crossbeam_channel::bounded(1);
        let mut watcher = match user_config.use_polling {
            true => WatcherWrapper::PollWatcher(new_debouncer_opt(
                Duration::new(user_config.rate_limit_seconds, 0),
                Some(Duration::new(2, 0)),
                tx,
                RecommendedCache::new(),
                notify::Config::default()
            )?),
            false => WatcherWrapper::PollWatcher(new_debouncer_opt(
                Duration::new(user_config.rate_limit_seconds, 0),
                Some(Duration::new(2, 0)),
                tx,
                RecommendedCache::new(),
                notify::Config::default()
                    .with_poll_interval(Duration::new(2, 0))
            )?)
        };
        
        let watched_paths = Self::directories_to_watch(project_directory, project_config);
        
        for directory in watched_paths.iter() {
            watcher.watch(directory, RecursiveMode::NonRecursive)?;
        }
        
        let join_handle = std::thread::spawn(|| match Self::worker(result_sender, watcher_receiver, shutdown_receiver) {
            Err(e) => result_sender.send(WatchMessage::WatchError(e)).unwrap(),
            Ok(_) => return
        });
        
        Ok()
    }
    
    fn worker(
        result_sender: Sender<WatchMessage>,
        watcher_receiver: Receiver<DebounceEventResult>,
        shutdown_receiver: Receiver<()>
    ) -> Result<()> {
        loop {
            let event = select! {
                recv(watcher_receiver) -> event => event,
                recv(shutdown_receiver) -> shutdown => {
                    trace!("recv shutdown event {shutdown:#?}");
                    return shutdown.context("failed to recv shutdown event").into()
                }
            }.context("failed to recv watcher event")?;
            
            event.
        }
    }
    
    fn directories_to_watch(
        project_directory: &Path,
        project_config: &ProjectConfig
    ) -> HashSet<PathBuf> {
        let mut builder = WalkBuilder::new(project_directory);
        builder.standard_filters(false);
        
        for file in project_config.ignorefiles.iter() {
            let result = builder.add_ignore(file);
            trace!("adding {file}: {result:#?}");
        }
        
        match {
            let mut override_builder = OverrideBuilder::new(project_directory);
            
            for file in project_config.ignore.iter() {
                let result = override_builder.add(file.as_str());
                trace!("ignore {file}: {result:#?}");
            }
            
            for file in project_config.include.iter() {
                let result = override_builder.add(format!("!{}", file).as_str());
                trace!("ignore {file}: {result:#?}");
            }
            
            override_builder.build()
        } {
            Ok(overrides) => { builder.overrides(overrides); }
            Err(e) => warn!("failed to apply overrides: {e}")
        }
        
        builder.build()
            .filter_map(|r| {
                trace!("watch: {r:#?}");
                match r {
                    Ok(directory) => Some(directory.into_path()),
                    Err(err) => { warn!("err: {err}"); None }
                }
            })
            .collect::<HashSet<PathBuf>>()
    }
}