use crate::conf::Conf;
use crate::error::Error;
use crate::vcfs::{Chromosome, group_vcf_files, VcfFileBlock};

pub(crate) fn run_jobs(conf: &Conf) -> Result<(), Error> {
    for vcf_files_of_chr in group_vcf_files(conf)? {
        let chromosome = vcf_files_of_chr.chromosome;
        for block in vcf_files_of_chr.blocks {
            run_job(chromosome, block)?;
        }
    }
    Ok(())
}

fn run_job(chromosome: Chromosome, block: VcfFileBlock) -> Result<(), Error> {
    println!("Once this is implemented, we'll run vcfs2bed for block {} of chromosome {}",
             block.i_block, chromosome);
    Ok(())
}


