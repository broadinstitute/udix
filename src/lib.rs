use crate::apps::bed_merge::{AppBedMerge, JobBedMerge};
use crate::apps::vcfs2bed::{AppVcfs2Bed, JobVcfs2Bed};
use crate::selection::{Choice, Config, Selection, DataChoice, AppChoice, DataSet};
use crate::error::Error;

pub mod error;
pub mod selection;
mod env;
mod conf;
mod dx;
mod apps;
mod job;
mod monitor;
mod data;

pub fn run(selection: Selection) -> Result<(), Error> {
    let conf = conf::read_conf()?;
    match selection.choice {
        Choice::Data {data_set, data_choice } => {
            match data_set {
                DataSet::Vcfs => {
                    match data_choice {
                        DataChoice::List => { data::vcfs::list_vcfs(&conf)?; }
                        DataChoice::Survey => { data::vcfs::survey_vcfs(&conf)? }
                    }
                }
                DataSet::Beds => {
                    match data_choice {
                        DataChoice::List => { data::beds::list_beds(&conf)?; }
                        DataChoice::Survey => { data::beds::survey_beds(&conf)? }
                    }
                }
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