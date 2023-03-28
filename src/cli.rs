use clap::{Arg, ArgMatches, command, Command};
use udix::error::Error;
use udix::selection::{Choice, Config, Params, Run, Selection, Vcfs, Vcfs2Bed};

mod top_cmd {
    pub(crate) const VCFS: &str = "vcfs";
    pub(crate) const VCFS2BED: &str = "vcfs2bed";
    pub(crate) const CONFIG: &str = "config";
    pub(crate) const CMDS: [&str; 3] = [VCFS, VCFS2BED, CONFIG];
}

mod vcfs_sub_cmd {
    pub(crate) const LIST: &str = "list";
    pub(crate) const SURVEY: &str = "survey";
    pub(crate) const CMDS: [&str; 2] = [LIST, SURVEY];
}

mod vcfs2bed_sub_cmd {
    pub(crate) const RUN: &str = "run";
    pub(crate) const MONITOR: &str = "monitor";
    pub(crate) const CMDS: [&str; 2] = [RUN, MONITOR];
}

mod config_sub_cmd {
    pub(crate) const DOWNLOAD: &str = "download";
    pub(crate) const CMDS: [&str; 1] = [DOWNLOAD];
}

mod params {
    pub(crate) const CONF_FILE: &str = "conf-file";
    pub(crate) const NUM: &str = "num";
    pub(crate) const DRY: &str = "dry";
    pub(crate) const PAT: &str = "pat";
}

mod defaults {
    pub(crate) const CONF_FILE: &str = "/udix/conf.toml";
}

fn new_command(name: &'static str) -> Command {
    Command::new(name).arg(Arg::new(params::CONF_FILE).long(params::CONF_FILE))
}

fn get_params(matches: &ArgMatches) -> Params {
    let conf_file =
        matches.get_one::<String>(params::CONF_FILE).cloned()
            .unwrap_or(defaults::CONF_FILE.to_string());
    Params { conf_file }
}

fn known_cmds_are(cmds: &[&str]) -> String {
    if cmds.len() == 1 {
        format!("Known command is {}", cmds.join(", "))
    } else {
        format!("Known commands are {}", cmds.join(", "))
    }
}

fn unknown_cmd_error(cmd: &str, cmds: &[&str]) -> Error {
    Error::from(format!("Unknown command {}. {}", cmd, known_cmds_are(cmds)))
}

fn missing_cmd_error(cmds: &[&str]) -> Error {
    Error::from(format!("Missing command. {}", known_cmds_are(cmds)))
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
                new_command(vcfs2bed_sub_cmd::RUN)
                    .arg(Arg::new(params::NUM).short('n').long(params::NUM))
                    .arg(Arg::new(params::DRY).short('d').long(params::DRY)
                        .num_args(0).action(clap::ArgAction::SetTrue))
                    .arg(Arg::new(params::PAT).short('p').long(params::PAT))
            ).subcommand(
            new_command(vcfs2bed_sub_cmd::MONITOR)
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
                    Err(unknown_cmd_error(unknown_cmd, &vcfs_sub_cmd::CMDS))
                }
                None => { Err(missing_cmd_error(&vcfs_sub_cmd::CMDS)) }
            }
        }
        Some((top_cmd::VCFS2BED, vcfs2bed_matches)) => {
            match vcfs2bed_matches.subcommand() {
                Some((vcfs2bed_sub_cmd::RUN, matches)) => {
                    let num =
                        matches.get_one::<String>(params::NUM)
                            .map(|s| s.parse::<usize>()).transpose()?;
                    let dry = matches.get_flag(params::DRY);
                    let pat = matches.get_one::<String>(params::PAT).cloned();
                    let run = Run { num, dry, pat };
                    let choice = Choice::Vcfs2Bed(Vcfs2Bed::Run(run));
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
                Some((vcfs2bed_sub_cmd::MONITOR, matches)) => {
                    let choice = Choice::Vcfs2Bed(Vcfs2Bed::Monitor);
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
                Some((unknown_cmd, _)) => {
                    Err(unknown_cmd_error(unknown_cmd, &vcfs2bed_sub_cmd::CMDS))
                }
                None => {
                    Err(missing_cmd_error(&vcfs2bed_sub_cmd::CMDS))
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
                    Err(unknown_cmd_error(unknown_cmd, &config_sub_cmd::CMDS))
                }
                None => {
                    Err(missing_cmd_error(&config_sub_cmd::CMDS))
                }
            }
        }
        Some((unknown_cmd, _)) => {
            Err(unknown_cmd_error(unknown_cmd, &top_cmd::CMDS))
        }
        None => {
            Err(missing_cmd_error(&top_cmd::CMDS))
        }
    }
}