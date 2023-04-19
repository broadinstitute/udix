use crate::apps::{App, JobStaged};
use crate::conf::Conf;
use crate::error::Error;
use serde::Serialize;
use crate::data::vcfs::Chromosome;

pub(crate) struct JobBedMerge {
    chromosome: Chromosome,
}

#[derive(Serialize)]
pub(crate) struct Inputs {}

pub(crate) struct AppBedMerge {}

impl JobStaged for JobBedMerge {
    const PREFIX: &'static str = "bedmerge";

    fn name(&self) -> String {
        format!("{}_c{}", Self::PREFIX, self.chromosome)
    }
}

impl App for AppBedMerge {
    type Job = JobBedMerge;
    type Inputs = Inputs;
    const INSTANCE_TYPE: &'static str = "mem2_hdd2_v2_x4";
    const APP_PATH: &'static str = "/apps/bedmerge/bedmerge";
    const OUT_DIR_PATH: &'static str = "/apps/bedmerge/out/udix/";

    fn create_job_list_unfiltered(conf: &Conf) -> Result<Vec<JobBedMerge>, Error> {
        todo!()
    }

    fn create_inputs_definition(job: &Self::Job, conf: &Conf) -> Result<Self::Inputs, Error> {
        todo!()
    }
}
