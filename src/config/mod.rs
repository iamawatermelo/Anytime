use anyhow::{Context, Result};
use log::{trace, warn};
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub mod anytime_proj;
pub mod anytime_user;
pub mod wakapi_toml;
pub mod wakatime_ini;

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub category: String,
    pub exclude_binary_files: bool,
    pub ignorefiles: Vec<String>,
    pub ignore: Vec<String>,
    pub include: Vec<String>,
}

impl ProjectConfig {
    pub fn load(project_root: &Path, user_config: &UserConfig) -> Result<Self> {
        // File-specific configs
        let anytime_proj_path = project_root.join("anytime.toml");
        let anytime_proj: anytime_proj::AnytimeProj = if anytime_proj_path.exists() {
            trace!("Found project config at {:?}", anytime_proj_path);
            toml::from_str(&fs::read_to_string(&anytime_proj_path)?)?
        } else {
            trace!("No project config found at {:?}", anytime_proj_path);
            Default::default()
        };

        let wak_toml_path = project_root.join("wak.toml");
        let wak_toml: wakapi_toml::WakapiToml = if wak_toml_path.exists() {
            trace!("Found wakapi config at {:?}", wak_toml_path);
            toml::from_str(&fs::read_to_string(&wak_toml_path)?)?
        } else {
            trace!("No wakapi config found at {:?}", wak_toml_path);
            Default::default()
        };

        let wakatime_project_path = project_root.join(".wakatime-project");
        let wakatime_project = if wakatime_project_path.exists() {
            trace!("Found wakatime project file at {:?}", wakatime_project_path);
            Some(
                fs::read_to_string(&wakatime_project_path)?
                    .trim()
                    .to_string(),
            )
        } else {
            trace!(
                "No wakatime project file found at {:?}",
                wakatime_project_path
            );
            None
        };

        // Project config stitching
        let project_config = Self {
            name: anytime_proj
                .project
                .name
                .or(wak_toml.project.name)
                .or(wakatime_project)
                .or_else(|| {
                    project_root
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                })
                .context("Could not determine project name")?,
            category: anytime_proj
                .project
                .category
                .unwrap_or_else(|| "coding".to_string()),
            exclude_binary_files: anytime_proj
                .files
                .exclude_binary_files
                .or(wak_toml.files.exclude_binary_files)
                .or(user_config.default_exclude_binary_files)
                .unwrap_or(true),
            ignorefiles: anytime_proj
                .files
                .ignorefiles
                .or(wak_toml.files.exclude_files)
                .unwrap_or_else(|| user_config.default_ignorefiles.clone()),
            ignore: anytime_proj
                .files
                .ignore
                .or(wak_toml.files.exclude)
                .unwrap_or_else(|| user_config.default_ignore.clone()),
            include: anytime_proj
                .files
                .include
                .or(wak_toml.files.include)
                .unwrap_or_else(|| user_config.default_include.clone()),
        };

        Ok(project_config)
    }
}

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub use_relative_filenames: bool,
    pub obfuscate_file_names: bool,
    pub hide_branch_names: bool,
    pub obfuscate_project_names: bool,
    pub use_polling: bool,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub rate_limit_seconds: u64,
    pub offline: bool,
    pub obfuscate_machine: bool,
    pub default_exclude_binary_files: Option<bool>,
    pub default_ignorefiles: Vec<String>,
    pub default_ignore: Vec<String>,
    pub default_include: Vec<String>,
}

impl UserConfig {
    pub fn load() -> Result<Self> {
        let mut anytime_user: anytime_user::AnytimeUser = Default::default();
        if let Some(xdg_config_home) = env::var("XDG_CONFIG_HOME").ok().map(PathBuf::from) {
            let user_conf_path = xdg_config_home.join("anytime/config.toml");
            if user_conf_path.exists() {
                trace!("Found user config at {:?}", user_conf_path);
                anytime_user = toml::from_str(&fs::read_to_string(&user_conf_path)?)?;
            } else {
                trace!("No user config found at {:?}", user_conf_path);
            }
        } else if let Some(home) = env::var("HOME").ok().map(PathBuf::from) {
            let user_conf_path = home.join("anytime.toml");
            if user_conf_path.exists() {
                trace!("Found user config at {:?}", user_conf_path);
                anytime_user = toml::from_str(&fs::read_to_string(&user_conf_path)?)?;
            } else {
                trace!("No user config found at {:?}", user_conf_path);
            }
        } else {
            trace!("Could not determine home directory to search for user config.");
        };

        let mut wakatime_ini: wakatime_ini::WakatimeIni = Default::default();
        let wakatime_home = env::var("WAKATIME_HOME").ok().map(PathBuf::from);
        if let Some(wakatime_home) = wakatime_home {
            let wakatime_cfg_path = wakatime_home.join(".wakatime.cfg");
            if wakatime_cfg_path.exists() {
                trace!("Found wakatime config at {:?}", wakatime_cfg_path);
                wakatime_ini = serini::from_str(&fs::read_to_string(wakatime_cfg_path)?)?;
            } else {
                trace!("No wakatime config found at {:?}", wakatime_cfg_path);
            }
        } else if let Some(home) = env::var("HOME").ok().map(PathBuf::from) {
            let wakatime_cfg_path = home.join(".wakatime.cfg");
            if wakatime_cfg_path.exists() {
                trace!("Found wakatime config at {:?}", wakatime_cfg_path);
                wakatime_ini = serini::from_str(&fs::read_to_string(wakatime_cfg_path)?)?;
            } else {
                trace!("No wakatime config found at {:?}", wakatime_cfg_path);
            }
        } else {
            trace!("Could not determine home directory to search for wakatime config.");
        };

        // User config stitching
        let rate_limit_seconds = {
            let limit = anytime_user
                .heartbeats
                .rate_limit_seconds
                .or(wakatime_ini.settings.heartbeat_rate_limit_seconds)
                .unwrap_or(30);
            if limit < 3 {
                warn!(
                    "Heartbeat interval of {} seconds is too low. Setting to 3 seconds.",
                    limit
                );
                3
            } else {
                limit
            }
        };
        let user_config = Self {
            use_relative_filenames: anytime_user
                .files
                .use_relative_filenames
                .or(wakatime_ini.settings.hide_project_folder)
                .unwrap_or(true),
            obfuscate_file_names: anytime_user.files.obfuscate_file_names.unwrap_or(false),
            hide_branch_names: anytime_user.files.hide_branch_names.unwrap_or(false),
            obfuscate_project_names: anytime_user.files.obfuscate_project_names.unwrap_or(false),
            use_polling: anytime_user.files.use_polling.unwrap_or(false),
            endpoint: wakatime_ini
                .settings
                .api_url
                .unwrap_or_else(|| "https://api.wakatime.com/api/v1".to_string()),
            api_key: wakatime_ini.settings.api_key.or(anytime_user.api.api_key),
            rate_limit_seconds,
            offline: anytime_user.heartbeats.offline.unwrap_or(true),
            obfuscate_machine: anytime_user.heartbeats.obfuscate_machine.unwrap_or(false),
            default_exclude_binary_files: anytime_user.defaults.exclude_binary_files,
            default_ignorefiles: anytime_user
                .defaults
                .ignorefiles
                .unwrap_or_else(|| vec![".gitignore".to_string(), ".ignore".to_string()]),
            default_ignore: anytime_user
                .defaults
                .ignore
                .unwrap_or_else(|| vec![".*".to_string(), ".git/".to_string()]),
            default_include: anytime_user
                .defaults
                .include
                .unwrap_or_else(|| vec![".gitignore".to_string()]),
        };

        Ok(user_config)
    }
}
