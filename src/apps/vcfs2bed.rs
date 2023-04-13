use std::path::Path;
use serde::Serialize;
use crate::conf::Conf;
use crate::dx;
use crate::apps::{App, JobStaged};
use crate::dx::WrappedDnaNexusLink;
use crate::error::Error;
use crate::vcfs::{Chromosome, group_vcf_files, VcfFileBlock};

pub(crate) struct JobVcfs2Bed {
    chromosome: Chromosome,
    block: VcfFileBlock,
}

#[derive(Serialize)]
pub(crate) struct Inputs {
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
}

impl App for AppVcfs2Bed {
    type Job = JobVcfs2Bed;
    type Inputs = Inputs;
    const INSTANCE_TYPE: &'static str = "mem2_hdd2_v2_x4";
    const APP_PATH: &'static str = "/apps/vcfs2bed/vcfs2bed";
    const OUT_DIR_PATH: &'static str = "/apps/vcf2bed/out/udix/";

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

    fn create_inputs_definition(job: &Self::Job, conf: &Conf) -> Result<Self::Inputs, Error> {
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
}
