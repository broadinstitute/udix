use crate::conf::Conf;
use crate::error::Error;
use crate::vcfs::{Chromosome, group_vcf_files, VcfFileBlock};

pub(crate) fn prepare(conf: &Conf) -> Result<(), Error> {
    for vcf_files_of_chr in group_vcf_files(conf)? {
        let chromosome = vcf_files_of_chr.chromosome;
        for block in vcf_files_of_chr.blocks {
            prepare_block(chromosome, block)?;
        }
    }
    Ok(())
}

fn prepare_block(chromosome: Chromosome, block: VcfFileBlock) -> Result<(), Error> {
    println!("Once this is implemented, we'll prepare block {} of chromosome {}",
             block.i_block, chromosome);
    Ok(())
}
