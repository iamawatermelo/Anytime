# Anytime configuration

## Project configuration

Sources:
- Anytime project file: `$PROJECT_ROOT/anytime.toml`
- wakapi-anyide project file: `$PROJECT_ROOT/wak.toml`
- Wakatime project name: `$PROJECT_ROOT/.wakatime-project`

### `[project]`

- `name` _String_  
  The project name sent to the wakapi server.  
  Defaults (in order):
  - `project.name` in `wak.toml`
  - a `.wakatime-project` file
  - the folder name

- `category` _String_  
  The project's category. See [Heartbeats](https://wakatime.com/developers#heartbeats). Default: `coding`.

### `[files]`

- `exclude_binary_files` _Boolean_  
  Whether to exclude non-UTF-8 files from tracking.  
  Defaults (in order):
  - `files.exclude_binary_files` in `wak.toml`
  - `defaults.exclude_binary_files` in your user config
  
- `ignorefiles` _String[]_  
  Set of ignore files to use, in glob format.
  Defaults (in order):
  - `files.exclude_files` in `wak.toml`
  - `defaults.ignorefiles` in your user config


- `ignore` _String[]_  
  Set of files/directories to ignore, in glob format.
  Defaults (in order):
    - `files.exclude` in `wak.toml`
    - `defaults.ignore` in your user config

- `include` _String[]_  
  Set of files/directories to include, in glob format.
  Defaults (in order):
    - `files.include` in `wak.toml`
    - `defaults.include` in your user config

## User configuration

Sources:
- Anytime config file: `$XDG_CONFIG_HOME/anytime/config.toml`, `$HOME/anytime.toml`
- Wakatime config file: `$HOME/.wakatime.cfg`, `$WAKATIME_HOME/.wakatime.cfg`

### `[files]`

- `use_relative_filenames` _Boolean_  
  Whether to hide the project folder from the Wakapi server. When `true`, the file's path will be relative to the project name. For example, this file will be reported as `Anytime/src/config/README.md` instead of `/var/home/sarah/Documents/Anytime/...`  
  Defaults (in order):
  - `settings.hide_project_folder` in `.wakatime.cfg`
  - `true`

- `obfuscate_file_names` _Boolean_  
  Whether to obfuscate file names sent to the Wakapi server. When `true`, files are reported as something like `Anytime/c643e7500ebf84e6a47a99389a84bf67`. Default: `false`.

- `hide_branch_names` _Boolean_  
  Whether to hide branch names sent to the Wakapi server. When `true`, branch names are not sent at all. Default: `false`.

- `obfuscate_project_names` _Boolean_  
  When `true`, Anytime will create a .wakatime-project file if the project name hasn't already been set with a randomly generated project name. Default: `false`.

- `use_polling` _Boolean_  
  Whether to use constant polling instead of platform-specific file watching APIs. Default: `false`.  
  
> [!CAUTION]
> The polling backend has poor performance. Only use it if your platform-specific
> backend doesn't work for you, for whatever reason.

### `[api]`

- `endpoint` _String_  
  The wakapi API URL to use.  
  Defaults (in order):
  - `settings.api_url` in `.wakatime.cfg`
  - `https://api.wakatime.com/api/v1`.

- `api_key` _String_  
  The wakapi API key to use. Defaults (in order):
  - `settings.api_key` in `.wakatime.cfg`

### `[heartbeats]`

- `rate_limit_seconds` _Number_  
  How long to wait in between sending heartbeats. Defaults (in order):
  - `settings.heartbeat_rate_limit_seconds` in `.wakatime.cfg`
  - 30 seconds.

- `offline` _Boolean_  
  Whether to enable saving heartbeats offline. Heartbeats are saved to `$XDG_STATE_HOME/anytime/heartbeats.db` or `$HOME/.anytime-heartbeats.db`. Default: `true`.

- `obfuscate_machine` _Boolean_  
  Whether to obfuscate your machine name. When `true`, machine names are reported as something like `anytime-9f408d2fc479201b`. Default: `false`.

### `[defaults]`

- `exclude_binary_files` _Boolean_  
  Whether to exclude non-UTF-8 files from tracking. Default: `true`.

- `ignorefiles` _String[]_  
  Set of ignore files to use, in glob format. Default: `[".gitignore", ".ignore"]`.

- `ignore` _String[]_  
  Set of files/directories to ignore, in glob format. Default: `[".*", ".git/"]`.

- `include` _String[]_  
  Set of files/directories to include, in glob format. Default: `[".gitignore"]`.