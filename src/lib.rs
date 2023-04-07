use crate::selection::{Choice, Config, Selection, Vcfs, AppChoice};
use crate::error::Error;

pub mod error;
pub mod selection;
mod env;
mod conf;
mod vcfs;
mod dx;
mod apps;
mod job;
mod monitor;

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
                AppChoice::Run(run_choice) => {
                    apps::vcfs2bed::run_jobs(&conf, &run_choice)?;
                }
                AppChoice::Monitor => { apps::vcfs2bed::monitor_jobs(&conf)?; }
            }
        }
        Choice::BedMerge(bed_merge_selection) => {
            match bed_merge_selection {
                AppChoice::Run(run_choice) => {
                    apps::bed_merge::run_jobs(&conf, &run_choice)?;
                }
                AppChoice::Monitor => { apps::bed_merge::monitor_jobs(&conf)?; }
            }
        }
        Choice::Config(config_selection) => {
            match config_selection {
                Config::Download => {
                    todo!()
                }
            }
        }
    }
    Ok(())
}