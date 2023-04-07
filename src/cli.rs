use clap::{Arg, ArgMatches, command, Command};
use udix::error::Error;
use udix::selection::{Choice, Config, Params, RunChoice, Selection, Vcfs, AppChoice};

mod top_cmd {
    pub(crate) const VCFS: &str = "vcfs";
    pub(crate) const VCFS2BED: &str = "vcfs2bed";
    pub(crate) const BED_MERGE: &str = "bed_merge";
    pub(crate) const CONFIG: &str = "config";
    pub(crate) const CMDS: [&str; 4] = [VCFS, VCFS2BED, BED_MERGE, CONFIG];
}

mod sub_cmd {
    pub(crate) const RUN: &str = "run";
}

mod vcfs_sub_cmd {
    pub(crate) const LIST: &str = "list";
    pub(crate) const SURVEY: &str = "survey";
    pub(crate) const CMDS: [&str; 2] = [LIST, SURVEY];
}

mod vcfs2bed_sub_cmd {
    use crate::cli::sub_cmd;

    pub(crate) const MONITOR: &str = "monitor";
    pub(crate) const CMDS: [&str; 2] = [sub_cmd::RUN, MONITOR];
}

mod bed_merge_sub_cmd {
    use crate::cli::sub_cmd;

    pub(crate) const MONITOR: &str = "monitor";
    pub(crate) const CMDS: [&str; 2] = [sub_cmd::RUN, MONITOR];
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

fn new_run_command() -> Command {
    new_command(sub_cmd::RUN)
        .arg(Arg::new(params::NUM).short('n').long(params::NUM))
        .arg(Arg::new(params::DRY).short('d').long(params::DRY)
            .num_args(0).action(clap::ArgAction::SetTrue))
        .arg(Arg::new(params::PAT).short('p').long(params::PAT))
}

fn get_params(matches: &ArgMatches) -> Params {
    let conf_file =
        matches.get_one::<String>(params::CONF_FILE).cloned()
            .unwrap_or(defaults::CONF_FILE.to_string());
    Params { conf_file }
}

fn get_run_choice(matches: &ArgMatches) -> Result<RunChoice, Error> {
    let num =
        matches.get_one::<String>(params::NUM)
            .map(|s| s.parse::<usize>()).transpose()?;
    let dry = matches.get_flag(params::DRY);
    let pat = matches.get_one::<String>(params::PAT).cloned();
    Ok(RunChoice { num, dry, pat })
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
            .subcommand(new_run_command())
            .subcommand(new_command(vcfs2bed_sub_cmd::MONITOR))
    ).subcommand(
        Command::new(top_cmd::BED_MERGE)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(new_run_command())
            .subcommand(new_command(bed_merge_sub_cmd::MONITOR))
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
                Some((sub_cmd::RUN, matches)) => {
                    let run = get_run_choice(matches)?;
                    let choice = Choice::Vcfs2Bed(AppChoice::Run(run));
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
                Some((vcfs2bed_sub_cmd::MONITOR, matches)) => {
                    let choice = Choice::Vcfs2Bed(AppChoice::Monitor);
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
        Some((top_cmd::BED_MERGE, bed_merge_matches)) => {
            match bed_merge_matches.subcommand() {
                Some((sub_cmd::RUN, matches)) => {
                    let run = get_run_choice(matches)?;
                    let choice = Choice::BedMerge(AppChoice::Run(run));
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
                Some((bed_merge_sub_cmd::MONITOR, matches)) => {
                    let choice = Choice::BedMerge(AppChoice::Monitor);
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
                Some((unknown_cmd, _)) => {
                    Err(unknown_cmd_error(unknown_cmd, &bed_merge_sub_cmd::CMDS))
                }
                None => {
                    Err(missing_cmd_error(&bed_merge_sub_cmd::CMDS))
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