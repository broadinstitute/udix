use std::fmt::{Display, Formatter};
use crate::apps::App::Vcfs2bed;

pub(crate) enum App {
    Vcfs2bed
}

const APPS: [App; 1] = [Vcfs2bed];
const VCFS2BED: &str = "vcfs2bed";

impl App {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Vcfs2bed => { VCFS2BED }
        }
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}