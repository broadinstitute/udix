use std::fmt::{Display, Formatter};
use crate::conf::Conf;
use crate::dx;
use crate::error::Error;

enum FileType {
    Bed,
    Bim,
    Fam,
}

pub(crate) struct BedBundle {
    basename: String,
}

impl Display for BedBundle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{{bed,bim,fam}}", self.basename)
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
        todo!()
    }
    fn unmatched_error(&self) -> Error {
        todo!()
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
        println!("{}", line);
    }
    if file_match_buffer.is_empty() {
        Ok(bed_bundles)
    } else {
        Err(file_match_buffer.unmatched_error())
    }
}

pub(crate) fn list_beds(conf: &Conf) -> Result<(), Error> {
    let stdout = dx::capture_stdout(&["ls", conf.data.beds_dir.as_str()])?;
    let mut lines: Vec<&str> = stdout.lines().collect();
    lines.sort();
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

pub(crate) fn survey_beds(conf: &Conf) -> Result<(), Error> {
    todo!()
}