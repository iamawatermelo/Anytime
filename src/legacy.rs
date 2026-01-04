//! Parse legacy wakapi-anyide files

use std::collections::HashMap;

use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct AnyideV1Config {
    meta: AnyideV1ConfigMeta,
    files: AnyideV1ConfigFiles,
    project: AnyideV1ConfigProject
}

fn default_watchers() -> Vec<String> {
    return vec!["files".to_string()]
}

#[derive(Deserialize, Debug)]
struct AnyideV1ConfigMeta {
    version: isize,
    
    #[serde(default = "default_watchers")]
    watchers: Vec<String>
}

/// this function is stupid and i hate serde
fn default_exclude_binary_files() -> bool { false }

#[derive(Deserialize, Debug)]
struct AnyideV1ConfigFiles {
    include: Vec<String>,
    exclude: Vec<String>,
    exclude_files: Vec<String>,
    
    #[serde(default = "default_exclude_binary_files")]
    exclude_binary_files: bool,
    
    #[serde(default)]
    language_mapping: HashMap<String, String>
}

#[derive(Deserialize, Debug)]
struct AnyideV1ConfigProject {
    name: String
}