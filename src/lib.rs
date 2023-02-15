use crate::selection::Selection;
use crate::error::Error;

pub mod error;
pub mod selection;
mod env;
mod conf;
mod vcfs;
mod dx;

pub fn run(selection: Selection) -> Result<(), Error> {
    let conf = conf::read_conf()?;
    match selection {
        Selection::ListVcfs => { vcfs::list_vcfs(&conf)?; }
        Selection::SurveyVcfs => { vcfs::survey_vcfs(&conf)? }
    }
    Ok(())
}