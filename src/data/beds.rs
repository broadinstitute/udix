use std::fmt::{Display, Formatter};
use std::mem;
use crate::conf::Conf;
use crate::data::chromosome::Chromosome;
use crate::dx;
use crate::error::Error;

enum FileType {
    Bed,
    Bim,
    Fam,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub(crate) struct BedBundle {
    prefix: String,
    chromosome: Chromosome,
    i_block: usize,
}

struct BedBundlesOfChr {
    chromosome: Chromosome,
    bed_bundles: Vec<BedBundle>,
}

impl BedBundle {
    fn cannot_parse(basename: &str) -> Error {
        Error::from(
            format!("Cannot parse {} as <prefix>_c<chromosome_b<i_block>", basename)
        )
    }
    fn parse(basename: &str) -> Result<BedBundle, Error> {
        let mut parts = basename.split('_');
        let prefix =
            parts.next().ok_or_else(|| { BedBundle::cannot_parse(basename) })?.to_string();
        let chromosome =
            Chromosome::parse(
                parts.next().ok_or_else(|| { BedBundle::cannot_parse(basename) })?
            )?;
        let i_block =
            parts.next().ok_or_else(|| { BedBundle::cannot_parse(basename) })?
                .strip_prefix('b').ok_or_else(|| { BedBundle::cannot_parse(basename) })?
                .parse::<usize>()?;
        if parts.next().is_some() {
            Err(BedBundle::cannot_parse(basename))?
        }
        Ok(BedBundle { prefix, chromosome, i_block })
    }
    fn basename(&self) -> String {
        format!("{}_c{}_b{}", self.prefix, self.chromosome, self.i_block)
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Bed => { write!(f, "bed") }
            FileType::Bim => { write!(f, "bim") }
            FileType::Fam => { write!(f, "fam") }
        }
    }
}

impl Display for BedBundle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{{bed,bim,fam}}", self.basename())
    }
}

struct FileMatchBuffer {
    basename: String,
    got_bed: bool,
    got_bim: bool,
    got_fam: bool,
}

impl FileMatchBuffer {
    fn new() -> FileMatchBuffer {
        let basename = "random placeholder".to_string();
        let got_bed = false;
        let got_bim = false;
        let got_fam = false;
        FileMatchBuffer { basename, got_bed, got_bim, got_fam }
    }
    fn got_file_type(&mut self, file_type: &FileType) -> &mut bool {
        match file_type {
            FileType::Bed => { &mut self.got_bed }
            FileType::Bim => { &mut self.got_bim }
            FileType::Fam => { &mut self.got_fam }
        }
    }
    fn push(&mut self, file_type: &FileType, basename: &str) -> Result<Option<BedBundle>, Error> {
        if *self.got_file_type(file_type) {
            Err(self.unmatched_error())
        } else {
            self.basename = basename.to_string();
            *self.got_file_type(file_type) = true;
            if self.got_bed && self.got_bim && self.got_fam {
                self.got_bed = false;
                self.got_bim = false;
                self.got_fam = false;
                Ok(Some(BedBundle::parse(basename)?))
            } else {
                Ok(None)
            }
        }
    }
    fn file_name(&self, file_type: &FileType) -> String {
        format!("{}.{}", self.basename, file_type)
    }
    fn unmatched_error(&self) -> Error {
        fn quantifier(got_one: bool) -> &'static str {
            if got_one { "a" } else { "no" }
        }
        let message =
            format!("Got {} {}, {} {}, and {} {}.",
                    quantifier(self.got_bed), self.file_name(&FileType::Bed),
                    quantifier(self.got_bim), self.file_name(&FileType::Bim),
                    quantifier(self.got_fam), self.file_name(&FileType::Fam));
        Error::from(message)
    }
    fn is_empty(&self) -> bool { (!self.got_bed) && (!self.got_bim) && (!self.got_fam) }
}

fn get_bed_bundles(conf: &Conf) -> Result<Vec<BedBundle>, Error> {
    let stdout = dx::capture_stdout(&["ls", conf.data.beds_dir.as_str()])?;
    let mut lines: Vec<&str> = stdout.lines().collect();
    lines.sort();
    let mut bed_bundles: Vec<BedBundle> = Vec::new();
    let mut file_match_buffer = FileMatchBuffer::new();
    for line in lines {
        let bed_bundle_opt_res =
            if let Some(bed_basename) = line.strip_suffix(".bed") {
                file_match_buffer.push(&FileType::Bed, bed_basename)
            } else if let Some(bim_basename) = line.strip_suffix(".bim") {
                file_match_buffer.push(&FileType::Bim, bim_basename)
            } else if let Some(fam_basename) = line.strip_suffix(".fam") {
                file_match_buffer.push(&FileType::Fam, fam_basename)
            } else {
                Ok(None)
            };
        if let Some(file_bundle) = bed_bundle_opt_res? {
            bed_bundles.push(file_bundle)
        }
    }
    if file_match_buffer.is_empty() {
        bed_bundles.sort();
        Ok(bed_bundles)
    } else {
        Err(file_match_buffer.unmatched_error())
    }
}

fn get_bed_bundles_by_chrom(conf: &Conf) -> Result<Vec<BedBundlesOfChr>, Error> {
    let mut bed_bundles_of_chrs: Vec<BedBundlesOfChr> = Vec::new();
    let mut bed_bundle_iter = get_bed_bundles(conf)?.into_iter();
    if let Some(bed_bundle) = bed_bundle_iter.next() {
        let mut chromosome = bed_bundle.chromosome;
        let mut bed_bundles_new: Vec<BedBundle> = vec!(bed_bundle);
        for bed_bundle_new in bed_bundle_iter {
            if bed_bundle_new.chromosome == chromosome {
                bed_bundles_new.push(bed_bundle_new)
            } else {
                let chromosome_new = bed_bundle_new.chromosome;
                let bed_bundles =
                    mem::replace(&mut bed_bundles_new, vec!(bed_bundle_new));
                bed_bundles_of_chrs.push(BedBundlesOfChr { chromosome, bed_bundles });
                chromosome = chromosome_new;
            }
        }
        bed_bundles_of_chrs.push(BedBundlesOfChr { chromosome, bed_bundles: bed_bundles_new });
    }
    Ok(bed_bundles_of_chrs)
}

pub(crate) fn list_beds(conf: &Conf) -> Result<(), Error> {
    for bed_bundle in get_bed_bundles(conf)? {
        println!("{}", bed_bundle)
    }
    Ok(())
}

pub(crate) fn survey_beds(conf: &Conf) -> Result<(), Error> {
    for bed_bundle_of_chr in get_bed_bundles_by_chrom(conf)? {
        println!("For chromosome {}, we have {} BED bundles.", bed_bundle_of_chr.chromosome,
                 bed_bundle_of_chr.bed_bundles.len())
    }
    Ok(())
}