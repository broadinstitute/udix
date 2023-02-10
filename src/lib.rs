use crate::selection::Selection;
use crate::error::Error;

pub mod error;
pub mod selection;
mod env;
mod conf;
mod vcfs;

pub fn run(selection: Selection) -> Result<(), Error>{
  match selection {
    Selection::ListVcfs => {
      let conf = conf::read_conf()?;
      vcfs::list_vcfs(conf)?;
      Ok(())
    }
  }
}