//! File watcher impl

use std::path::Path;

use notify_debouncer_full::{DebounceEventResult, Debouncer, FileIdMap, new_debouncer, new_debouncer_opt, notify::*};

use crate::config::ProjectConfig;

struct FileWatcher<T: Watcher> {
    notifier: Debouncer<T, FileIdMap>
}

impl<T: Watcher> FileWatcher<T> {
    pub fn new(
        project_directory: &Path,
        project_config: ProjectConfig
    ) -> Self {
        todo!();
    }
}