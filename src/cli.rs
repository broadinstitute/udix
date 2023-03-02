use clap::{ArgMatches, command, Command};
use udix::error::Error;
use udix::selection::{Selection, Vcfs, Vcfs2Bed};

mod top_cmd {
    pub(crate) const VCFS: &str = "vcfs";
    pub(crate) const VCFS2BED: &str = "vcfs2bed";
}

mod vcfs_sub_cmd {
    pub(crate) const LIST: &str = "list";
    pub(crate) const SURVEY: &str = "survey";
}

mod vcfs2bed_sub_cmd {
    pub(crate) const PREPARE: &str = "prepare";
}

pub(crate) fn get_selection() -> Result<Selection, Error> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(top_cmd::VCFS)
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new(vcfs_sub_cmd::LIST)
                )
                .subcommand(
                    Command::new(vcfs_sub_cmd::SURVEY)
                )
        ).subcommand(
        Command::new(top_cmd::VCFS2BED)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new(vcfs2bed_sub_cmd::PREPARE)
            )
    ).get_matches();
    match matches.subcommand() {
        Some((top_cmd::VCFS, vcfs_matches)) => {
            match vcfs_matches.subcommand() {
                Some((vcfs_sub_cmd::LIST, _)) => { Ok(Selection::Vcfs(Vcfs::List)) }
                Some((vcfs_sub_cmd::SURVEY, _)) => { Ok(Selection::Vcfs(Vcfs::Survey)) }
                Some((unknown_cmd, _)) => {
                    Err(Error::from(format!(
                        "Unknown command {unknown_cmd}. Known commands are {} and {}",
                        vcfs_sub_cmd::LIST, vcfs_sub_cmd::SURVEY
                    )))
                }
                None => {
                    Err(Error::from(format!(
                        "Missing command. Known commands are {} and {}",
                        vcfs_sub_cmd::LIST, vcfs_sub_cmd::SURVEY
                    )))
                }
            }
        }
        Some((top_cmd::VCFS2BED, vcfs2bed_matches)) => {
            match vcfs2bed_matches.subcommand() {
                Some((vcfs2bed_sub_cmd::PREPARE, _)) => {
                    Ok(Selection::Vcfs2Bed(Vcfs2Bed::Prepare))
                }
                Some((unknown_cmd, _)) => {
                    Err(Error::from(format!(
                        "Unknown command {unknown_cmd}. Known command is {}",
                        vcfs2bed_sub_cmd::PREPARE
                    )))
                }
                None => {
                    Err(Error::from(format!(
                        "Missing command. Known command is {}",
                        vcfs2bed_sub_cmd::PREPARE
                    )))
                }
            }
        }
        Some((unknown_cmd, _)) => {
            Err(Error::from(format!(
                "Unknown command {unknown_cmd}. Known commands are {} and {}", top_cmd::VCFS,
                top_cmd::VCFS2BED
            )))
        }
        None => {
            Err(Error::from(format!(
                "Missing command. Known commands are {} and {}", top_cmd::VCFS,
                top_cmd::VCFS2BED
            )))
        }
    }
}