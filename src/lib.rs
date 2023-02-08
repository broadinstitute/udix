use crate::selection::Selection;
use crate::error::Error;

pub mod error;
pub mod selection;

pub fn run(selection: Selection) -> Result<(), Error>{
  match selection {
    Selection::ListVcfs => {
      println!("Hello, world!");
      Ok(())
    }
  }
}