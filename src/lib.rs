use crate::apps::bed_merge::{AppBedMerge, JobBedMerge};
use crate::apps::vcfs2bed::{AppVcfs2Bed, JobVcfs2Bed};
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
                    apps::run_jobs::<AppVcfs2Bed>(&conf, &run_choice)?;
                }
                AppChoice::Monitor => { apps::monitor_jobs::<JobVcfs2Bed>(&conf)?; }
            }
        }
        Choice::BedMerge(bed_merge_selection) => {
            match bed_merge_selection {
                AppChoice::Run(run_choice) => {
                    apps::run_jobs::<AppBedMerge>(&conf, &run_choice)?;
                }
                AppChoice::Monitor => { apps::monitor_jobs::<JobBedMerge>(&conf)?; }
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