use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct AnytimeUser {
    #[serde(default)]
    pub files: Files,
    #[serde(default)]
    pub api: Api,
    #[serde(default)]
    pub heartbeats: Heartbeats,
    #[serde(default)]
    pub defaults: Defaults,
}

#[derive(Debug, Deserialize, Default)]
pub struct Files {
    #[serde(default)]
    pub use_relative_filenames: Option<bool>,
    #[serde(default)]
    pub obfuscate_file_names: Option<bool>,
    #[serde(default)]
    pub hide_branch_names: Option<bool>,
    #[serde(default)]
    pub obfuscate_project_names: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Api {
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Heartbeats {
    #[serde(default)]
    pub rate_limit_seconds: Option<u64>,
    #[serde(default)]
    pub offline: Option<bool>,
    #[serde(default)]
    pub obfuscate_machine: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Defaults {
    #[serde(default)]
    pub exclude_binary_files: Option<bool>,
}
