//! Anytime user-specific config

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct UserConfig {
    files: UserConfigFiles,
    api: UserConfigApi,
    heartbeats: UserConfigHeartbeats,
    defaults: UserConfigDefaults
}

#[derive(Deserialize, Serialize, Debug)]
struct UserConfigFiles {
    hide_filenames: bool,
    hide_branch_names: bool,
    hide_project_names: bool
}

#[derive(Deserialize, Serialize, Debug)]
struct UserConfigApi {
    endpoint: String,
    api_key: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct UserConfigHeartbeats {
    rate_limit_seconds: usize,
    offline: bool,
    accurate_machine: bool
}

#[derive(Deserialize, Serialize, Debug)]
struct UserConfigDefaults {
    use_accurate_reporting: bool,
    exclude_binary_files: bool
}

impl UserConfig {
    fn default_from_wakatime_cfg() -> UserConfig {
        UserConfig {
            files: UserConfigFiles {
                hide_filenames: true,
                hide_branch_names: true,
                hide_project_names: true
            },
            api: UserConfigApi {
                endpoint: "https://api.wakatime.com/api/v1".to_string(),
                api_key: None
            },
            heartbeats: UserConfigHeartbeats {
                rate_limit_seconds: 30,
                offline: true,
                accurate_machine: false
            },
            defaults: UserConfigDefaults {
                use_accurate_reporting: false,
                exclude_binary_files: true
            }
        }
    }
}