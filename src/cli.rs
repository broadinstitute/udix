use clap::{Arg, ArgMatches, command, Command};
use udix::error::Error;
use udix::selection::{Choice, Config, Params, RunChoice, Selection, DataChoice, AppChoice, DataSet};

mod top_cmd {
    pub(crate) const VCFS: &str = "vcfs";
    pub(crate) const BEDS: &str = "beds";
    pub(crate) const VCFS2BED: &str = "vcfs2bed";
    pub(crate) const BED_MERGE: &str = "bed_merge";
    pub(crate) const CONFIG: &str = "config";
    pub(crate) const CMDS: [&str; 5] = [VCFS, BEDS, VCFS2BED, BED_MERGE, CONFIG];
}

mod data_sub_cmd {
    pub(crate) const LIST: &str = "list";
    pub(crate) const SURVEY: &str = "survey";
    pub(crate) const CMDS: [&str; 2] = [LIST, SURVEY];
}

mod app_sub_cmd {
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

fn new_data_command(name: &'static str) -> Command {
    Command::new(name)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            new_command(data_sub_cmd::LIST)
        )
        .subcommand(
            new_command(data_sub_cmd::SURVEY)
        )
}

fn new_run_command() -> Command {
    new_command(app_sub_cmd::RUN)
        .arg(Arg::new(params::NUM).short('n').long(params::NUM))
        .arg(Arg::new(params::DRY).short('d').long(params::DRY)
            .num_args(0).action(clap::ArgAction::SetTrue))
        .arg(Arg::new(params::PAT).short('p').long(params::PAT))
}

fn get_params_and_data_choice(top_matches: &ArgMatches) -> Result<(DataChoice, Params), Error> {
    match top_matches.subcommand() {
        Some((data_sub_cmd::LIST, sub_matches)) => {
            let data_choice = DataChoice::List;
            let params = get_params(sub_matches);
            Ok((data_choice, params))
        }
        Some((data_sub_cmd::SURVEY, sub_matches)) => {
            let data_choice = DataChoice::Survey;
            let params = get_params(sub_matches);
            Ok((data_choice, params))
        }
        Some((unknown_cmd, _)) => {
            Err(unknown_cmd_error(unknown_cmd, &data_sub_cmd::CMDS))
        }
        None => { Err(missing_cmd_error(&data_sub_cmd::CMDS)) }
    }
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
        .subcommand(new_data_command(top_cmd::VCFS))
        .subcommand(new_data_command(top_cmd::BEDS))
        .subcommand(
            Command::new(top_cmd::VCFS2BED)
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(new_run_command())
                .subcommand(new_command(app_sub_cmd::MONITOR))
        ).subcommand(
        Command::new(top_cmd::BED_MERGE)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(new_run_command())
            .subcommand(new_command(app_sub_cmd::MONITOR))
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
            let data_set = DataSet::Vcfs;
            let (data_choice, params) =
                get_params_and_data_choice(vcfs_matches)?;
            let choice = Choice::Data { data_set, data_choice };
            Ok(Selection { choice, params })
        }
        Some((top_cmd::BEDS, beds_matches)) => {
            let data_set = DataSet::Beds;
            let (data_choice, params) =
                get_params_and_data_choice(beds_matches)?;
            let choice = Choice::Data { data_set, data_choice };
            Ok(Selection { choice, params })
        }
        Some((top_cmd::VCFS2BED, vcfs2bed_matches)) => {
            match vcfs2bed_matches.subcommand() {
                Some((app_sub_cmd::RUN, matches)) => {
                    let run = get_run_choice(matches)?;
                    let choice = Choice::Vcfs2Bed(AppChoice::Run(run));
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
                Some((app_sub_cmd::MONITOR, matches)) => {
                    let choice = Choice::Vcfs2Bed(AppChoice::Monitor);
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
                Some((unknown_cmd, _)) => {
                    Err(unknown_cmd_error(unknown_cmd, &app_sub_cmd::CMDS))
                }
                None => {
                    Err(missing_cmd_error(&app_sub_cmd::CMDS))
                }
            }
        }
        Some((top_cmd::BED_MERGE, bed_merge_matches)) => {
            match bed_merge_matches.subcommand() {
                Some((app_sub_cmd::RUN, matches)) => {
                    let run = get_run_choice(matches)?;
                    let choice = Choice::BedMerge(AppChoice::Run(run));
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
                Some((app_sub_cmd::MONITOR, matches)) => {
                    let choice = Choice::BedMerge(AppChoice::Monitor);
                    let params = get_params(matches);
                    Ok(Selection { choice, params })
                }
                Some((unknown_cmd, _)) => {
                    Err(unknown_cmd_error(unknown_cmd, &app_sub_cmd::CMDS))
                }
                None => {
                    Err(missing_cmd_error(&app_sub_cmd::CMDS))
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