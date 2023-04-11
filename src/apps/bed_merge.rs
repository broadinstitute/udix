use crate::apps::{App, JobStaged};
use crate::conf::Conf;
use crate::error::Error;

pub(crate) struct JobBedMerge {}

pub(crate) struct AppBedMerge {}

impl JobStaged for JobBedMerge {
    const PREFIX: &'static str = "";

    fn name(&self) -> String {
        todo!()
    }

    fn is_name(name: &str) -> bool {
        todo!()
    }
}

impl App for AppBedMerge {
    type Job = JobBedMerge;
    type Inputs = ();
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
