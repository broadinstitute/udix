use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Serialize;
use crate::conf::Conf;
use crate::{dx, monitor};
use crate::dx::WrappedDnaNexusLink;
use crate::error::Error;
use crate::monitor::JobInfo;
use crate::selection::Run;
use crate::vcfs::{Chromosome, group_vcf_files, VcfFileBlock};

struct JobStaged {
    chromosome: Chromosome,
    block: VcfFileBlock,
}

#[derive(Serialize)]
struct Inputs {
    vcfs: Vec<WrappedDnaNexusLink>,
    out_prefix: String,
}

const VCFS2ED: &str = "vcfs2bed";

impl JobStaged {
    fn name(&self) -> String {
        format!("{}_c{}_b{}", VCFS2ED, self.chromosome, self.block.i_block)
    }
    fn is_name(name: &str) -> bool { name.starts_with(VCFS2ED) }
}

fn should_be_run(name: &str, jobs: &HashMap<String, JobInfo>) -> bool {
    match jobs.get(name) {
        None => { true }
        Some(job) => { (!job.is_done()) && (!job.is_running()) }
    }
}

fn create_job_list(conf: &Conf) -> Result<Vec<JobStaged>, Error> {
    let mut jobs: Vec<JobStaged> = Vec::new();
    let submitted_jobs = monitor::jobs_by_name(conf)?;
    for vcf_files_of_chr in group_vcf_files(conf)? {
        let chromosome = vcf_files_of_chr.chromosome;
        for block in vcf_files_of_chr.blocks {
            let job_staged = JobStaged { chromosome, block };
            if should_be_run(&job_staged.name(), &submitted_jobs) {
                jobs.push(job_staged)
            }
        }
    }
    Ok(jobs)
}

pub(crate) fn run_jobs(conf: &Conf, run: &Run) -> Result<(), Error> {
    let mut jobs = create_job_list(conf)?;
    if let Some(num) = run.num {
        jobs.truncate(num)
    }
    for job in jobs {
        if run.dry {
            println!("This would run {}", job.name())
        } else {
            run_job(&job, conf)?;
        }
    }
    Ok(())
}

fn create_inputs_definition(job: &JobStaged, conf: &Conf) -> Result<Inputs, Error> {
    let mut vcfs: Vec<WrappedDnaNexusLink> = Vec::new();
    for vcf_file in &job.block.files {
        let path_string = format!("{}{}", conf.data.vcfs_dir, vcf_file.name);
        let path = Path::new(path_string.as_str());
        let vcf_file_id = dx::get_wrapped_dna_nexus_link(path)?;
        vcfs.push(vcf_file_id)
    }
    let out_prefix = job.name();
    Ok(Inputs { vcfs, out_prefix })
}

fn write_inputs_definition(file: &Path, inputs: &Inputs) -> Result<(), Error> {
    let string = serde_json::to_string_pretty(inputs)?;
    println!("{}", string);
    fs::write(file, string)?;
    Ok(())
}

fn inputs_file_name(job: &JobStaged) -> String {
    format!("inputs_{}", job.name())
}

const INSTANCE_TYPE: &str = "mem2_hdd2_v2_x4";

fn run_job(job: &JobStaged, conf: &Conf) -> Result<(), Error> {
    let inputs = create_inputs_definition(job, conf)?;
    let work_dir = Path::new(&conf.workspace.work_dir);
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
    let out_prefix_arg = format!("out_prefix:string={}", name);
    println!("{}", out_prefix_arg);
    let folder_arg = format!("{}/apps/vcf2bed/out/udix/", dx::get_project()?);
    dx::run(&["run", "--name", name.as_str(), "--input-json-file", inputs_file_arg,
        "--folder", folder_arg.as_str(), "--instance-type", INSTANCE_TYPE,
        "/apps/vcfs2bed/vcfs2bed"])?;
    println!("Launched job {}.", name);
    Ok(())
}

pub(crate) fn monitor_jobs(conf: &Conf) -> Result<(), Error> {
    let jobs = monitor::find_jobs(conf)?;
    for job in jobs {
        if JobStaged::is_name(&job.name) {
            println!("Job {} is {}", job.name, job.state);
        }
    }
    Ok(())
}
