use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Serialize;
use crate::conf::Conf;
use crate::error::Error;
use crate::monitor;
use crate::monitor::JobInfo;
use crate::selection::RunChoice;

pub(crate) mod vcfs2bed;
pub(crate) mod bed_merge;

pub(crate) trait JobStaged {
    const PREFIX: &'static str;
    fn name(&self) -> String;
    fn is_name(name: &str) -> bool;
}

pub(crate) trait App {
    type Job: JobStaged;
    fn create_job_list_unfiltered(conf: &Conf) -> Result<Vec<Self::Job>, Error>;
    fn run_job(job: &Self::Job, conf: &Conf) -> Result<(), Error>;
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
            A::run_job(&job, conf)?;
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



