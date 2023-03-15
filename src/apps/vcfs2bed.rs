use crate::conf::Conf;
use crate::dx;
use crate::error::Error;
use crate::vcfs::{Chromosome, group_vcf_files, VcfFileBlock};

struct JobStaged {
    chromosome: Chromosome,
    block: VcfFileBlock,
}

impl JobStaged {
    fn name(&self) -> String {
        format!("vcfs2bed_c{}_b{}", self.chromosome, self.block.i_block)
    }
}

fn create_job_list(conf: &Conf) -> Result<Vec<JobStaged>, Error> {
    let mut jobs: Vec<JobStaged> = Vec::new();
    for vcf_files_of_chr in group_vcf_files(conf)? {
        let chromosome = vcf_files_of_chr.chromosome;
        for block in vcf_files_of_chr.blocks {
            jobs.push(JobStaged { chromosome, block })
        }
    }
    Ok(jobs)
}

pub(crate) fn run_jobs(conf: &Conf, num: &Option<usize>) -> Result<(), Error> {
    let mut jobs = create_job_list(conf)?;
    if let Some(num) = num {
        jobs.truncate(*num)
    }
    for job in jobs {
        run_job(&job, conf)?;
    }
    Ok(())
}

fn run_job(job: &JobStaged, conf: &Conf) -> Result<(), Error> {
    let name = job.name();
    println!("TODO: run {}", name);
    let vcfs =
        job.block.files.iter()
            .map(|file| { format!("{}{}", conf.data.vcfs_dir, file.name) })
            .collect::<Vec<String>>()
            .join("\",\"");
    let vcfs_arg = format!("vcfs:array:file=[\"{vcfs}\"]");
    println!("{vcfs_arg}");
    let out_prefix_arg = format!("out_prefix:string={}", name);
    println!("{}", out_prefix_arg);
    let folder_arg = format!("{}/apps/vcf2bed/out/udix/", dx::get_project()?);
    dx::run(&["run", "--name", name.as_str(), "-i", vcfs_arg.as_str(), "-i",
        out_prefix_arg.as_str(), "--folder", folder_arg.as_str(), "/apps/vcfs2bed/vcfs2bed"])?;
    Ok(())
}


