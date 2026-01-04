//! Anytime project config

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct ProjectConfig {
    project: ProjectConfigDescription,
    files: ProjectConfigFiles
}

#[derive(Deserialize, Serialize, Debug)]
struct ProjectConfigDescription {
    project_name: String
}

fn default_use_accurate_reporting() -> bool { false }
fn default_exclude_binary_files() -> bool { true }

#[derive(Deserialize, Serialize, Debug)]
struct ProjectConfigFiles {
    #[serde(default = "default_use_accurate_reporting")]
    use_accurate_reporting: bool,
    
    #[serde(default = "default_exclude_binary_files")]
    exclude_binary_files: bool
}

impl ProjectConfig {
    fn default_from_current_project() {
        
    }
}