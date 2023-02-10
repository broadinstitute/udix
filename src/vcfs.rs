use crate::conf::Conf;
use crate::error::Error;
use std::process::Command;
use std::str;

pub(crate) fn list_vcfs(conf: Conf) -> Result<(), Error> {
    println!("VCF files are here: {}", conf.data.vcfs_dir);
    let vcfs_list_bytes =
        Command::new("dx").args(["ls", conf.data.vcfs_dir.as_str()]).output()?.stdout;
    let vcfs_list_string = str::from_utf8(&vcfs_list_bytes)?;
    println!("{}", vcfs_list_string);
    Ok(())
}