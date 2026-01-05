use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct WakapiToml {
    #[serde(default)]
    pub project: Project,
    #[serde(default)]
    pub files: Files,
}

#[derive(Debug, Deserialize, Default)]
pub struct Project {
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Files {
    #[serde(default)]
    pub exclude_binary_files: Option<bool>,
}
