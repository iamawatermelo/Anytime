use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct WakatimeIni {
    #[serde(default)]
    pub settings: Settings,
}

#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    #[serde(default)]
    pub api_url: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub hide_project_folder: Option<bool>,
    #[serde(default)]
    pub heartbeat_rate_limit_seconds: Option<u64>,
}
