use crate::job::JobState::{Failed, Staged, Submitted, Succeeded};

pub(crate) enum JobState {
    Staged,
    Submitted,
    Failed,
    Succeeded,
}

const STATES: [JobState; 4] = [Staged, Submitted, Failed, Succeeded];
const STAGED: &str = "staged";
const SUBMITTED: &str = "submitted";
const FAILED: &str = "failed";
const SUCCEEDED: &str = "succeeded";



