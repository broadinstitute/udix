use std::process::{Command, Output};
use crate::error::Error;
use std::str;

const DX: &str = "dx";

pub(crate) fn capture_stdout(args: &[&str]) -> Result<String, Error> {
    let output = run(args)?;
    Ok(String::from_utf8(output.stdout)?)
}

pub(crate) fn run(args: &[&str]) -> Result<Output, Error> {
    let output = Command::new(DX).args(args).output()?;
    if output.status.success() {
        Ok(output)
    } else {
        let message =
            format!("dx failed ({}): {}", output.status,
                    String::from_utf8_lossy(&output.stderr));
        Err(Error::from(message))
    }
}

pub(crate) fn pwd() -> Result<String, Error> {
    capture_stdout(&["pwd"])
}

pub(crate) fn get_project() -> Result<String, Error> {
    let pwd = pwd()?;
    pwd.split(':').next().map(|s| s.to_string())
        .ok_or_else(||{ Error::from(format!("Could not parse project from '{}'.", pwd))})
}