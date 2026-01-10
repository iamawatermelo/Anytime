//! watcherwrapper impl

use std::path::Path;

use log::trace;
use notify_debouncer_full::{Debouncer, RecommendedCache, notify::{self, PollWatcher, RecommendedWatcher, RecursiveMode}};

// this is stupid
#[derive(Debug)]
pub enum WatcherWrapper {
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