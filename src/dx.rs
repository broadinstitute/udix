use std::process::Command;
use crate::error::Error;
use std::str;

pub(crate) fn capture_stdout(args: &[&str]) -> Result<String, Error> {
    let output = Command::new("dx").args(args).output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        let message =
            format!("dx failed ({}): {}", output.status,
                    String::from_utf8_lossy(&output.stderr));
        Err(Error::from(message))
    }
}