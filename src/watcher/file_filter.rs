//! Filter for files

use std::{collections::HashSet, path::{Path, PathBuf}};

use ignore::{WalkBuilder, overrides::OverrideBuilder};
use log::{trace, warn};

use crate::config::ProjectConfig;

/// Compute set of directories to watch
pub(super) fn directories_to_watch(
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
                Err(err) => { warn!("Error while traversing directory: {err}"); None }
            }
        })
        .collect::<HashSet<PathBuf>>()
}

pub(super) fn build_filters(
    project_directory: &Path,
    project_config: &ProjectConfig
) -> 