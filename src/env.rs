use std::env;
use crate::error::Error;

mod key {
    pub(crate) const HOME: &str = "HOME";
}

pub(crate) fn get_home() -> Result<String, Error> {
    Ok(env::var(key::HOME)?)
}