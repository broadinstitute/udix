use std::collections::HashMap;
use crate::conf::Conf;
use crate::dx;
use crate::error::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct JobInfo {
    pub(crate) name: String,
    pub(crate) state: String
}

mod state {
    pub(crate) const DONE: &str = "done";
    pub(crate) const RUNNING: &str = "running";
}

impl JobInfo {
    pub(crate) fn is_done(&self) -> bool { self.state == state::DONE }
    pub(crate) fn is_running(&self) -> bool { self.state == state::RUNNING }
}

pub(crate) fn find_jobs(conf: &Conf) -> Result<Vec<JobInfo>, Error> {
    let start_date = conf.misc.start_date.as_str();
    let output =
        dx::capture_stdout(&["find", "jobs", "-n", "100000", "--created-after", start_date,
            "--json"])?;
    let jobs: Vec<JobInfo> = serde_json::from_str(&output)?;
    Ok(jobs)
}

pub(crate) fn jobs_by_name(conf: &Conf) -> Result<HashMap<String, JobInfo>, Error> {
    let mut jobs_by_name: HashMap<String, JobInfo> = HashMap::new();
    for job in find_jobs(conf)? {
        if !jobs_by_name.contains_key(&job.name) {
            jobs_by_name.insert(job.name.clone(), job);
        }
    }
    Ok(jobs_by_name)
}