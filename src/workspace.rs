use std::path::PathBuf;
use crate::conf::WorkspaceConf;
use crate::env;
use crate::error::Error;
use crate::job::Job;

pub(crate) struct Workspace {
    work_dir: PathBuf,
}

fn fix_home_dir(file: &str) -> Result<String, Error> {
    if let Some(file_in_home) = file.strip_prefix("~/") {
        Ok(format!("{}/{}", env::get_home()?, file_in_home))
    } else {
        Ok(file.to_string())
    }
}

impl Workspace {
    fn new(conf: &WorkspaceConf) -> Result<Workspace, Error> {
        let work_dir = PathBuf::from(fix_home_dir(&conf.work_dir)?);
        Ok(Workspace { work_dir })
    }
    fn job_dir(&self, job: &Job) -> PathBuf {
        self.work_dir.join(job.app.as_str()).join(job.name.as_str())
    }
}