use clap::{command, Command};
use udix::error::Error;
use udix::selection::Selection;

mod cmd {
    pub(crate) const LIST_VCFS: &str = "list-vcfs";
    pub(crate) const SURVEY_VCFS: &str = "survey-vcfs";
    pub(crate) const LIST: [&str; 2] = [LIST_VCFS, SURVEY_VCFS];
}

fn command_list() -> String {
    cmd::LIST.join(", ")
}

pub(crate) fn get_selection() -> Result<Selection, Error> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(cmd::LIST_VCFS)
        ).subcommand(
        Command::new(cmd::SURVEY_VCFS)
    )
        .get_matches();
    match matches.subcommand() {
        Some((cmd::LIST_VCFS, _)) => { Ok(Selection::ListVcfs) }
        Some((cmd::SURVEY_VCFS, _)) => { Ok(Selection::SurveyVcfs) }
        Some((unknown_cmd, _)) => {
            Err(Error::from(format!(
                "Unknown command {unknown_cmd}. Known command is {}", command_list()
            )))
        }
        None => {
            Err(Error::from(format!(
                "Missing command. Known command is {}", command_list()
            )))
        }
    }
}