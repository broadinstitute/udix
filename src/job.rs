use std::fmt::{Display, Formatter};
use serde::Deserialize;
use crate::error::Error;

#[derive(Deserialize, PartialEq)]
#[serde(try_from = "&str")]
pub(crate) enum JobState {
    Running,
    Failed,
    Done,
}

mod names {
    pub(crate) const RUNNING: &str = "running";
    pub(crate) const FAILED: &str = "failed";
    pub(crate) const DONE: &str = "done";
}


impl TryFrom<&str> for JobState {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            names::RUNNING => { Ok(JobState::Running) }
            names::FAILED => { Ok(JobState::Failed) }
            names::DONE => { Ok(JobState::Done) }
            unknown_state => {
                Err(Error::from(
                    format!("Unknown job state {}. Known job states are {}, {} and {}",
                            unknown_state, names::RUNNING, names::FAILED, names::DONE)
                ))
            }
        }
    }
}

impl Display for JobState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JobState::Running => { write!(f, "{}", names::RUNNING) }
            JobState::Failed => { write!(f, "{}", names::FAILED) }
            JobState::Done => { write!(f, "{}", names::DONE) }
        }
    }
}



