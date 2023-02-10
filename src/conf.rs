use std::fs::read_to_string;
use crate::env;
use crate::error::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Conf {
    pub(crate) data: DataConf
}

#[derive(Deserialize)]
pub(crate) struct DataConf {
    pub(crate) vcfs_dir: String
}

pub(crate) fn read_conf() -> Result<Conf, Error> {
    let conf_file = format!("{}/.config/udix/udix.toml", env::get_home()?);
    let conf_string = read_to_string(conf_file)?;
    let conf = toml::from_str::<Conf>(&conf_string)?;
    Ok(conf)
}