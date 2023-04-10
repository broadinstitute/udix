use std::fs;
use std::path::Path;
use serde::Serialize;
use crate::conf::Conf;
use crate::{apps, dx};
use crate::apps::{App, JobStaged};
use crate::dx::WrappedDnaNexusLink;
use crate::error::Error;
use crate::vcfs::{Chromosome, group_vcf_files, VcfFileBlock};

pub(crate) struct JobVcfs2Bed {
    chromosome: Chromosome,
    block: VcfFileBlock,
}

#[derive(Serialize)]
struct Inputs {
    vcfs: Vec<WrappedDnaNexusLink>,
    out_prefix: String,
}

pub(crate) struct AppVcfs2Bed {

}


impl JobStaged for JobVcfs2Bed {
    const PREFIX: &'static str = "vcfs2bed";
    fn name(&self) -> String {
        format!("{}_c{}_b{}", Self::PREFIX, self.chromosome, self.block.i_block)
    }
    fn is_name(name: &str) -> bool { name.starts_with(Self::PREFIX) }
}

impl App for AppVcfs2Bed {
    type Job = JobVcfs2Bed;

    fn create_job_list_unfiltered(conf: &Conf) -> Result<Vec<JobVcfs2Bed>, Error> {
        let mut jobs: Vec<JobVcfs2Bed> = Vec::new();
        for vcf_files_of_chr in group_vcf_files(conf)? {
            let chromosome = vcf_files_of_chr.chromosome;
            for block in vcf_files_of_chr.blocks {
                let job_staged = JobVcfs2Bed { chromosome, block };
                jobs.push(job_staged);
            }
        }
        Ok(jobs)
    }

    fn run_job(job: &Self::Job, conf: &Conf) -> Result<(), Error> {
        let inputs = create_inputs_definition(job, conf)?;
        let work_dir_string = conf.workspace.work_dir_fixed()?;
        let work_dir = Path::new(&work_dir_string);
        fs::create_dir_all(work_dir)?;
        let name = job.name();
        let inputs_file = work_dir.join(apps::inputs_file_name(job));
        apps::write_inputs_definition(&inputs_file, &inputs)?;
        println!("Next job to run is {}", name);
        let inputs_file_arg =
            inputs_file.to_str().ok_or_else(|| {
                Error::from(format!("Could not convert file path '{}' to string.",
                                    inputs_file.to_string_lossy()))
            })?;
        let folder_arg = format!("{}/apps/vcf2bed/out/udix/", dx::get_project()?);
        dx::run(&["run", "--name", name.as_str(), "--input-json-file", inputs_file_arg,
            "--folder", folder_arg.as_str(), "--instance-type", INSTANCE_TYPE,
            "/apps/vcfs2bed/vcfs2bed"])?;
        println!("Launched job {} with input file {}.", name, inputs_file_arg);
        Ok(())
    }
}

fn create_inputs_definition(job: &JobVcfs2Bed, conf: &Conf) -> Result<Inputs, Error> {
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

const INSTANCE_TYPE: &str = "mem2_hdd2_v2_x4";

