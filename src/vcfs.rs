use std::fmt::{Display, Formatter};
use std::mem;
use std::mem::replace;
use crate::conf::Conf;
use crate::error::Error;
use crate::dx;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub(crate) enum Chromosome {
    Auto(u8),
    Allo(char),
}

impl Chromosome {
    pub(crate) fn parse(string: &str) -> Result<Chromosome, Error> {
        let stripped =
            if let Some(stripped) = string.strip_prefix("chr") {
                stripped
            } else if let Some(stripped) = string.strip_prefix('c') {
                stripped
            } else {
                string
            };
        if stripped == "X" {
            Ok(Chromosome::Allo('X'))
        } else if stripped == "Y" {
            Ok(Chromosome::Allo('Y'))
        } else {
            let number = stripped.parse::<u8>()?;
            Ok(Chromosome::Auto(number))
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct VcfFileKey {
    pub(crate) chromosome: Chromosome,
    pub(crate) i_file: usize,
}

pub(crate) struct VcfFile {
    pub(crate) key: VcfFileKey,
    pub(crate) name: String,
}

pub(crate) struct VcfFileBlock {
    pub(crate) i_block: usize,
    pub(crate) files: Vec<VcfFile>
}

pub(crate) struct VcfFilesOfChr {
    pub(crate) chromosome: Chromosome,
    pub(crate) blocks: Vec<VcfFileBlock>
}

const BLOCK_SIZE: usize = 100;

impl VcfFileKey {
    fn i_block(&self) -> usize {
        self.i_file / BLOCK_SIZE
    }
}

impl Display for Chromosome {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Chromosome::Auto(num) => { write!(f, "{}", num) }
            Chromosome::Allo(sym) => { write!(f, "{}", sym) }
        }
    }
}

impl VcfFile {
    fn parse_base(name: &str, base_name: &str) -> Result<VcfFile, Error> {
        let mut parts = base_name.split('_');
        let parse_failure =
            || { Error::from(format!("Cannot parse VCF file '{}'.", name)) };
        let _ = parts.next().ok_or_else(parse_failure)?;
        let chromosome =
            Chromosome::parse(parts.next().ok_or_else(parse_failure)?)?;
        let i_file =
            parts.next().ok_or_else(parse_failure)?
                .strip_prefix('b').ok_or_else(parse_failure)?
                .parse::<usize>()?;
        let name = name.to_string();
        let key = VcfFileKey { chromosome, i_file };
        Ok(VcfFile { key, name })
    }
    fn parse_if_vcf(name: &str) -> Result<Option<VcfFile>, Error> {
        if let Some(base_name) = name.strip_suffix(".vcf.gz") {
            Ok(Some(VcfFile::parse_base(name, base_name)?))
        } else {
            Ok(None)
        }
    }
}

fn get_vcf_files(conf: &Conf) -> Result<Vec<VcfFile>, Error> {
    let stdout = dx::capture_stdout(&["ls", conf.data.vcfs_dir.as_str()])?;
    let mut vcf_files: Vec<VcfFile> = Vec::new();
    for line in stdout.lines() {
        match VcfFile::parse_if_vcf(line)? {
            None => {}
            Some(vcf_file) => { vcf_files.push(vcf_file) }
        }
    }
    Ok(vcf_files)
}

fn get_vcf_files_sorted(conf: &Conf) -> Result<Vec<VcfFile>, Error> {
    let mut vcf_files = get_vcf_files(conf)?;
    vcf_files.sort_by(|file1, file2| file1.key.cmp(&file2.key));
    Ok(vcf_files)
}

pub(crate) fn group_vcf_files(conf: &Conf) -> Result<Vec<VcfFilesOfChr>, Error> {
    let mut files =  get_vcf_files_sorted(conf)?.into_iter();
    let mut files_by_chr: Vec<VcfFilesOfChr> = Vec::new();
    if let Some(file) = files.next() {
        let mut i_block = file.key.i_block();
        let mut chr = file.key.chromosome;
        let mut files_for_block: Vec<VcfFile> = vec![file];
        let mut blocks: Vec<VcfFileBlock> = Vec::new();
        for file in files {
            let chr_new = file.key.chromosome;
            let i_block_new = file.key.i_block();
            if chr_new != chr {
                let files_for_block_old =
                    replace(&mut files_for_block, vec![file]);
                let block = VcfFileBlock { i_block, files: files_for_block_old };
                blocks.push(block);
                let blocks_old = mem::take(&mut blocks);
                files_by_chr.push(VcfFilesOfChr { chromosome: chr, blocks: blocks_old });
                chr = chr_new;
                i_block = i_block_new;
            } else if i_block_new != i_block {
                let files_for_block_old =
                    replace(&mut files_for_block, vec![file]);
                let block = VcfFileBlock { i_block, files: files_for_block_old };
                blocks.push(block);
                i_block = i_block_new;
            } else {
                files_for_block.push(file);
            }
        }
        let group = VcfFileBlock { i_block, files: files_for_block };
        blocks.push(group);
        files_by_chr.push(VcfFilesOfChr { chromosome: chr, blocks });
    }
    Ok(files_by_chr)
}

pub(crate) fn list_vcfs(conf: &Conf) -> Result<(), Error> {
    let vcf_files = get_vcf_files_sorted(conf)?;
    for vcf in vcf_files {
        println!("{}", vcf.name);
    }
    Ok(())
}

pub(crate) fn survey_vcfs(conf: &Conf) -> Result<(), Error> {
    println!("VCF files are here: {}", conf.data.vcfs_dir);
    let files_by_chr = group_vcf_files(conf)?;
    let mut n_files: usize = 0;
    let mut n_blocks: usize = 0;
    let mut n_chrs: usize = 0;
    for files_of_chr in files_by_chr {
        let mut n_blocks_for_chr: usize = 0;
        let mut n_files_for_chr: usize = 0;
        for block in &files_of_chr.blocks {
            n_blocks_for_chr += 1;
            n_files_for_chr += block.files.len()
        }
        println!("Chromosome {} has {} files in {} blocks", files_of_chr.chromosome,
                 n_files_for_chr, n_blocks_for_chr);
        n_chrs += 1;
        n_blocks += n_blocks_for_chr;
        n_files += n_files_for_chr;
    }
    println!("There are {} files in {} blocks and {} chromosomes.", n_files, n_blocks, n_chrs);
    Ok(())
}

