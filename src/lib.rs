use crate::selection::{Choice, Config, Selection, Vcfs, Vcfs2Bed};
use crate::error::Error;

pub mod error;
pub mod selection;
mod env;
mod conf;
mod vcfs;
mod dx;
mod workspace;
mod apps;
mod job;

pub fn run(selection: Selection) -> Result<(), Error> {
    let conf = conf::read_conf()?;
    match selection.choice {
        Choice::Vcfs(vcfs_selection) => {
            match vcfs_selection {
                Vcfs::List => { vcfs::list_vcfs(&conf)?; }
                Vcfs::Survey => { vcfs::survey_vcfs(&conf)? }
            }
        }
        Choice::Vcfs2Bed(vcfs2bed_selection) => {
            match vcfs2bed_selection {
                Vcfs2Bed::Prepare => { apps::vcfs2bed::prepare(&conf)?; }
            }
        }
        Choice::Config(config_selection) => {
            match config_selection {
                Config::Download => {

                }
            }
        }
    }
    Ok(())
}