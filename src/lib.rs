use crate::selection::Selection;
use crate::error::Error;

pub mod error;
pub mod selection;
mod env;
mod conf;

pub fn run(selection: Selection) -> Result<(), Error>{
  match selection {
    Selection::ListVcfs => {
      let conf = conf::read_conf()?;
      println!("VCF files are here: {}", conf.data.vcfs_dir);
      Ok(())
    }
  }
}