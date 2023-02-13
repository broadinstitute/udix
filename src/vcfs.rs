use std::fmt::{Display, Formatter};
use crate::conf::Conf;
use crate::error::Error;
use crate::dx;

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

struct VcfFile {
    chromosome: Chromosome,
    block: usize,
    name: String,
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
        Ok(VcfFile { chromosome, block, name })
    }
    fn parse_if_vcf(name: &str) -> Result<Option<VcfFile>, Error> {
        if let Some(base_name) = name.strip_suffix(".vcf.gz") {
            Ok(Some(VcfFile::parse_base(name, base_name)?))
        } else {
            Ok(None)
        }
    }
}

pub(crate) fn list_vcfs(conf: Conf) -> Result<(), Error> {
    println!("VCF files are here: {}", conf.data.vcfs_dir);
    let stdout = dx::capture_stdout(&["ls", conf.data.vcfs_dir.as_str()])?;
    let mut vcf_files: Vec<VcfFile> = Vec::new();
    for line in stdout.lines() {
        match VcfFile::parse_if_vcf(line)? {
            None => {}
            Some(vcf_file) => { vcf_files.push(vcf_file) }
        }
    }
    println!("{}", vcf_files.len());
    for vcf in vcf_files {
        println!("Name: {}, chromosome: {}, block: {}", vcf.name, vcf.chromosome, vcf.block);
    }
    Ok(())
}