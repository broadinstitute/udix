use std::path::Path;
use std::process::{Command, Output};
use crate::error::Error;
use std::str;
use serde::Serialize;
use serde_json::Value;

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
        .ok_or_else(|| { Error::from(format!("Could not parse project from '{}'.", pwd)) })
}

pub(crate) fn get_id_from_path(path: &Path) -> Result<String, Error> {
    let path_str =
        path.to_str().ok_or_else(|| {
            Error::from(format!("Could not convert path '{}' to string.", path.to_string_lossy()))
        })?;
    let json_string = capture_stdout(&["describe", "--json", path_str])?;
    let json_value: Value = serde_json::from_str(json_string.as_str())?;
    let id = json_value["id"].as_str().ok_or_else(|| {
        Error::from(format!("Could not get id for '{}'", path.to_string_lossy()))
    })?.to_string();
    Ok(id)
}

#[derive(Serialize)]
pub(crate) struct DnaNexusLink {
    id: String,
    project: String,
}

#[derive(Serialize)]
pub(crate) struct WrappedDnaNexusLink {
    #[serde(rename = "$dnanexus_link")]
    dnanexus_link: DnaNexusLink,
}

pub(crate) fn get_dna_nexus_link(path: &Path) -> Result<DnaNexusLink, Error> {
    let path_str =
        path.to_str().ok_or_else(|| {
            Error::from(format!("Could not convert path '{}' to string.",
                                path.to_string_lossy()))
        })?;
    let json_string = capture_stdout(&["describe", "--json", path_str])?;
    let json_value: Value = serde_json::from_str(json_string.as_str())?;
    let id =
        json_value["id"].as_str().ok_or_else(|| {
            Error::from(format!("Could not get id for '{}'", path.to_string_lossy()))
        })?.to_string();
    let project =
        json_value["project"].as_str().ok_or_else(|| {
            Error::from(format!("Could not get project for '{}'", path.to_string_lossy()))
        })?.to_string();
    Ok(DnaNexusLink { id, project })
}

pub(crate) fn get_wrapped_dna_nexus_link(path: &Path) -> Result<WrappedDnaNexusLink, Error> {
    let dnanexus_link = get_dna_nexus_link(path)?;
    Ok(WrappedDnaNexusLink { dnanexus_link })
}
