use std::fmt::{Display, Formatter};
use std::mem;
use std::mem::replace;
use crate::conf::Conf;
use crate::error::Error;
use crate::dx;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Chromosome {
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
struct VcfFileKey {
    chromosome: Chromosome,
    block: usize,
}

struct VcfFile {
    key: VcfFileKey,
    name: String,
}

struct VcfFileGroup {
    i_group: usize,
    files: Vec<VcfFile>
}

struct VcfFilesOfChr {
    chromosome: Chromosome,
    groups: Vec<VcfFileGroup>
}

const GROUP_SIZE: usize = 100;

impl VcfFileKey {
    fn i_group(&self) -> usize {
        self.block / GROUP_SIZE
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
        let block =
            parts.next().ok_or_else(parse_failure)?
                .strip_prefix('b').ok_or_else(parse_failure)?
                .parse::<usize>()?;
        let name = name.to_string();
        let key = VcfFileKey { chromosome, block };
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

fn group_vcf_files(conf: &Conf) -> Result<Vec<VcfFilesOfChr>, Error> {
    let mut files =  get_vcf_files_sorted(conf)?.into_iter();
    let mut files_by_chr: Vec<VcfFilesOfChr> = Vec::new();
    if let Some(file) = files.next() {
        let mut i_group = file.key.i_group();
        let mut chr = file.key.chromosome;
        let mut files_for_group: Vec<VcfFile> = vec![file];
        let mut groups: Vec<VcfFileGroup> = Vec::new();
        for file in files {
            let chr_new = file.key.chromosome;
            let i_group_new = file.key.i_group();
            if chr_new != chr {
                let files_for_group_old =
                    replace(&mut files_for_group, vec![file]);
                let group = VcfFileGroup { i_group, files: files_for_group_old };
                groups.push(group);
                let groups_old = mem::take(&mut groups);
                files_by_chr.push(VcfFilesOfChr { chromosome: chr, groups: groups_old});
                chr = chr_new;
                i_group = i_group_new;
            } else if i_group_new != i_group {
                let files_for_group_old =
                    replace(&mut files_for_group, vec![file]);
                let group = VcfFileGroup { i_group, files: files_for_group_old };
                groups.push(group);
                i_group = i_group_new;
            } else {
                files_for_group.push(file);
            }
        }
        let group = VcfFileGroup { i_group, files: files_for_group };
        groups.push(group);
        files_by_chr.push(VcfFilesOfChr { chromosome: chr, groups});
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
    let mut n_groups: usize = 0;
    let mut n_chrs: usize = 0;
    for files_of_chr in files_by_chr {
        let mut n_groups_for_chr: usize = 0;
        let mut n_files_for_chr: usize = 0;
        for group in &files_of_chr.groups {
            n_groups_for_chr += 1;
            n_files_for_chr += group.files.len()
        }
        println!("Chromosome {} has {} files in {} groups", files_of_chr.chromosome,
                 n_files_for_chr, n_groups_for_chr);
        n_chrs += 1;
        n_groups += n_groups_for_chr;
        n_files += n_files_for_chr;
    }
    println!("There are {} files in {} groups and {} chromosomes.", n_files, n_groups, n_chrs);
    Ok(())
}

