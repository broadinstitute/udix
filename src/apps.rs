use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Serialize;
use crate::conf::Conf;
use crate::error::Error;
use crate::{dx, monitor};
use crate::monitor::JobInfo;
use crate::selection::RunChoice;

pub(crate) mod vcfs2bed;
pub(crate) mod bed_merge;

pub(crate) trait JobStaged {
    const PREFIX: &'static str;
    fn name(&self) -> String;
    fn is_name(name: &str) -> bool { name.starts_with(Self::PREFIX) }
}

pub(crate) trait App {
    type Job: JobStaged;
    type Inputs: Serialize;
    const INSTANCE_TYPE: &'static str;
    const APP_PATH: &'static str;
    const OUT_DIR_PATH: &'static str;
    fn create_job_list_unfiltered(conf: &Conf) -> Result<Vec<Self::Job>, Error>;
    fn create_inputs_definition(job: &Self::Job, conf: &Conf) -> Result<Self::Inputs, Error>;
}

fn should_be_run(name: &str, jobs: &HashMap<String, JobInfo>) -> bool {
    match jobs.get(name) {
        None => { true }
        Some(job) => { job.state.needs_to_be_submitted() }
    }
}

fn filter_job_list<J: JobStaged>(jobs_unfiltered: Vec<J>, conf: &Conf, pat: &Option<String>)
                                 -> Result<Vec<J>, Error> {
    let mut jobs: Vec<J> = Vec::new();
    let submitted_jobs = monitor::jobs_by_name(conf)?;
    for job in jobs_unfiltered {
        let passes_pat =
            match pat {
                None => { true }
                Some(pat) => { job.name().contains(pat) }
            };
        if passes_pat && should_be_run(&job.name(), &submitted_jobs) {
            jobs.push(job)
        }
    }
    Ok(jobs)
}

pub(crate) fn run_jobs<A: App>(conf: &Conf, run: &RunChoice) -> Result<(), Error> {
    let pat = &run.pat;
    let jobs_unfiltered = A::create_job_list_unfiltered(conf)?;
    let mut jobs = filter_job_list(jobs_unfiltered, conf, pat)?;
    if let Some(num) = run.num {
        jobs.truncate(num)
    }
    for job in jobs {
        if run.dry {
            println!("This would run {}", job.name())
        } else {
            run_job::<A>(&job, conf)?;
        }
    }
    Ok(())
}

fn write_inputs_definition<I: Serialize>(file: &Path, inputs: &I) -> Result<(), Error> {
    let string = serde_json::to_string_pretty(inputs)?;
    fs::write(file, string)?;
    Ok(())
}

fn inputs_file_name<J: JobStaged>(job: &J) -> String {
    format!("inputs_{}", job.name())
}

pub(crate) fn monitor_jobs<J: JobStaged>(conf: &Conf) -> Result<(), Error> {
    let jobs = monitor::find_jobs(conf)?;
    for job in jobs {
        if J::is_name(&job.name) {
            println!("Job {} is {}", job.name, job.state);
        }
    }
    Ok(())
}

fn run_job<A: App>(job: &A::Job, conf: &Conf) -> Result<(), Error> {
    let inputs = A::create_inputs_definition(job, conf)?;
    let work_dir_string = conf.workspace.work_dir_fixed()?;
    let work_dir = Path::new(&work_dir_string);
    fs::create_dir_all(work_dir)?;
    let name = job.name();
    let inputs_file = work_dir.join(inputs_file_name(job));
    write_inputs_definition(&inputs_file, &inputs)?;
    println!("Next job to run is {}", name);
    let inputs_file_arg =
        inputs_file.to_str().ok_or_else(|| {
            Error::from(format!("Could not convert file path '{}' to string.",
                                inputs_file.to_string_lossy()))
        })?;
    let folder_arg = format!("{}:{}", dx::get_project()?, A::OUT_DIR_PATH);
    dx::run(&["run", "--name", name.as_str(), "--input-json-file", inputs_file_arg,
        "--folder", folder_arg.as_str(), "--instance-type", A::INSTANCE_TYPE,
        A::APP_PATH])?;
    println!("Launched job {} with input file {}.", name, inputs_file_arg);
    Ok(())
}



