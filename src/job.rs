use crate::apps::App;
use crate::job::JobState::{Failed, Staged, Submitted, Succeeded};

pub(crate) enum JobState {
    Staged,
    Submitted,
    Failed,
    Succeeded,
}

pub(crate) struct Job {
    pub(crate) app: App,
    pub(crate) name: String
}

const STATES: [JobState; 4] = [Staged, Submitted, Failed, Succeeded];
const STAGED: &str = "staged";
const SUBMITTED: &str = "submitted";
const FAILED: &str = "failed";
const SUCCEEDED: &str = "succeeded";

impl JobState {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Staged => { STAGED }
            Submitted => { SUBMITTED }
            Failed => { FAILED }
            Succeeded => { SUCCEEDED }
        }
    }
}


