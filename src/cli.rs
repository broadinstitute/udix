use clap::{Arg, ArgMatches, command, Command};
use udix::error::Error;
use udix::selection::{Choice, Config, Params, Selection, Vcfs, Vcfs2Bed};

mod top_cmd {
    pub(crate) const VCFS: &str = "vcfs";
    pub(crate) const VCFS2BED: &str = "vcfs2bed";
    pub(crate) const CONFIG: &str = "config";
}

mod vcfs_sub_cmd {
    pub(crate) const LIST: &str = "list";
    pub(crate) const SURVEY: &str = "survey";
}

mod vcfs2bed_sub_cmd {
    pub(crate) const PREPARE: &str = "prepare";
}

mod config_sub_cmd {
    pub(crate) const DOWNLOAD: &str = "download";
}

mod params {
    pub(crate) const CONF_FILE: &str = "conf-file";
}

mod defaults {
    pub(crate) const CONF_FILE: &str = "/udix/conf.toml";
}

fn new_command(name: &'static str) -> Command {
    Command::new(name).arg(Arg::new(params::CONF_FILE))
}

fn get_params(matches: &ArgMatches) -> Params {
    let conf_file =
        matches.get_one::<String>(params::CONF_FILE).cloned()
            .unwrap_or(defaults::CONF_FILE.to_string());
    Params { conf_file }
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
                    new_command(vcfs_sub_cmd::LIST)
                )
                .subcommand(
                    new_command(vcfs_sub_cmd::SURVEY)
                )
        ).subcommand(
        Command::new(top_cmd::VCFS2BED)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                new_command(vcfs2bed_sub_cmd::PREPARE)
            )
    ).subcommand(
        Command::new(top_cmd::CONFIG)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                new_command(config_sub_cmd::DOWNLOAD)
            )
    ).get_matches();
    match matches.subcommand() {
        Some((top_cmd::VCFS, vcfs_matches)) => {
            match vcfs_matches.subcommand() {
                Some((vcfs_sub_cmd::LIST, matches)) => {
                    let choice = Choice::Vcfs(Vcfs::List);
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
                Some((vcfs_sub_cmd::SURVEY, matches)) => {
                    let choice = Choice::Vcfs(Vcfs::Survey);
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
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
                Some((vcfs2bed_sub_cmd::PREPARE, matches)) => {
                    let choice = Choice::Vcfs2Bed(Vcfs2Bed::Prepare);
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
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
        Some((top_cmd::CONFIG, config_matches)) => {
            match config_matches.subcommand() {
                Some((config_sub_cmd::DOWNLOAD, matches)) => {
                    let choice = Choice::Config(Config::Download);
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
                Some((unknown_cmd, _)) => {
                    Err(Error::from(format!(
                        "Unknown command {unknown_cmd}. Known command is {}",
                        config_sub_cmd::DOWNLOAD
                    )))
                }
                None => {
                    Err(Error::from(format!(
                        "Missing command. Known command is {}",
                        config_sub_cmd::DOWNLOAD
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