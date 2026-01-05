use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct AnytimeProj {
    #[serde(default)]
    pub project: Project,
    #[serde(default)]
    pub files: Files,
}

#[derive(Debug, Deserialize, Default)]
pub struct Project {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Files {
    #[serde(default)]
    pub exclude_binary_files: Option<bool>,
}
