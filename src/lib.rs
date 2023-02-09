use crate::selection::Selection;
use crate::error::Error;

pub mod error;
pub mod selection;
mod env;
mod conf;

pub fn run(selection: Selection) -> Result<(), Error>{
  match selection {
    Selection::ListVcfs => {
      let home = env::get_home()?;
      println!("Hello, world! Welcome at home: {}", home);
      Ok(())
    }
  }
}