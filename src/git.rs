//! Utilities for handling Git

use std::{path::PathBuf, process::Command};

use anyhow::{Context, Result, bail};
use log::{debug, trace};

pub struct GitCli {
    pub version_tuple: (usize, usize, usize)
}

impl GitCli {
    pub fn new() -> Result<Self> {
        trace!("running git -v");
        let git_version_cmd = Command::new("git")
            .args(["-v"])
            .output()?;
        let git_version_stdout = String::from_utf8_lossy(&git_version_cmd.stdout[..]);
        trace!("git version: {:?}", git_version_stdout);
        
        let version_string = git_version_stdout
            .strip_prefix("git version ").context("result of git -v had wrong output")?
            .split(".")
            .filter_map(|x| x.parse::<usize>().ok())
            .take(3)
            .collect::<Vec<usize>>();
        
        let version_tuple = match version_string[..] {
            [] | [_] => bail!("result of git -v had wrong output"),
            [major, minor] => (major, minor, 0),
            [major, minor, patch, ..] => (major, minor, patch)
        };
        trace!("parsed git version {version_tuple:?}");
        
        Ok(Self {
            version_tuple
        })
    }
    
    pub fn get_project_root(&self) -> Result<PathBuf> {
        // first try git rev-parse --absolute-git-dir because it handles
        // edge cases like submodules
        if let Ok(git_absolute_dir_output) = Command::new("git")
            .args(["rev-parse", "--absolute-git-dir"])
            .output()
        {
            let mut path = PathBuf::from(String::from_utf8(git_absolute_dir_output.stdout)?);
            trace!("git rev-parse --absolute-git-dir: {path:#?}");
            path.pop();
            return Ok(path)
        }
        
        // otherwise use git rev-parse --show-toplevel (for older Git)
        let git_toplevel_output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()?;
        let path = PathBuf::from(String::from_utf8(git_toplevel_output.stdout)?);
        trace!("git rev-parse --absolute-git-dir: {path:#?}");
        
        Ok(path)
    }
}