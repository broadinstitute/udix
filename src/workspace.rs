use std::path::PathBuf;
use crate::conf::WorkspaceConf;
use crate::error::Error;
use crate::job::Job;

pub(crate) struct Workspace {
    work_dir: PathBuf,
}

impl Workspace {
    fn new(conf: &WorkspaceConf) -> Result<Workspace, Error> {
        let work_dir = PathBuf::from(&conf.work_dir);
        Ok(Workspace { work_dir })
    }
    fn job_dir(&self, job: &Job) -> PathBuf {
        self.work_dir.join(job.app.as_str()).join(job.name.as_str())
    }
}