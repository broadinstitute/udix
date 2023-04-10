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

    fn create_job_list_unfiltered(conf: &Conf) -> Result<Vec<JobBedMerge>, Error> {
        todo!()
    }

    fn run_job(job: &Self::Job, conf: &Conf) -> Result<(), Error> {
        todo!()
    }
}
