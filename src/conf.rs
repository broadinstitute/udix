use std::fs;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use crate::{dx, env};
use crate::error::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Conf {
    pub(crate) data: DataConf,
    pub(crate) workspace: WorkspaceConf,
    pub(crate) misc: Misc
}

#[derive(Deserialize)]
pub(crate) struct DataConf {
    pub(crate) vcfs_dir: String,
    pub(crate) beds_dir: String,
}

#[derive(Deserialize)]
pub(crate) struct WorkspaceConf {
    pub(crate) work_dir: String,
}

#[derive(Deserialize)]
pub(crate) struct Misc {
    pub(crate) start_date: String
}

fn fix_home_dir(file: &str) -> Result<String, Error> {
    if let Some(file_in_home) = file.strip_prefix("~/") {
        Ok(format!("{}/{}", env::get_home()?, file_in_home))
    } else {
        Ok(file.to_string())
    }
}

impl WorkspaceConf {
    pub(crate) fn work_dir_fixed(&self) -> Result<String, Error> {
        fix_home_dir(&self.work_dir)
    }
}

fn get_local_conf_file() -> Result<PathBuf, Error> {
    Ok(PathBuf::from(format!("{}/.config/udix/udix.toml", env::get_home()?)))
}

const REMOTE_CONF_FILE: &str = "/udix/udix.toml";

fn fresh_conf_file_exists(file: &Path) -> Result<bool, Error> {
    if Path::new(file).exists() {
        let duration = Duration::from_secs(3600);
        let modification_time = fs::metadata(file)?.modified()?;
        Ok(SystemTime::now().duration_since(modification_time)? < duration)
    } else {
        Ok(false)
    }
}

fn download_conf_file(file: &Path) -> Result<(), Error> {
    let file_str =
        file.to_str().ok_or_else(|| {
            Error::from(format!("Cannot convert '{}' to string", file.to_string_lossy()))
        })?;
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent)?;
    }
    dx::run(&["download", REMOTE_CONF_FILE, "--overwrite", "--output", file_str])?;
    Ok(())
}

pub(crate) fn read_conf() -> Result<Conf, Error> {
    let conf_file = get_local_conf_file()?;
    if !fresh_conf_file_exists(&conf_file)? {
        download_conf_file(&conf_file)?
    }
    let conf_string = read_to_string(conf_file)?;
    let conf = toml::from_str::<Conf>(&conf_string)?;
    Ok(conf)
}