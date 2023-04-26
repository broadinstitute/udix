use std::path::Path;
use crate::apps::{App, JobStaged};
use crate::conf::Conf;
use crate::error::Error;
use serde::Serialize;
use crate::data::beds::{FileType, BedBundle, BedBundlesOfChr, get_bed_bundles_by_chrom};
use crate::dx;
use crate::dx::WrappedDnaNexusLink;

pub(crate) struct JobBedMerge {
    bed_bundles_of_chr: BedBundlesOfChr,
}

#[derive(Serialize)]
pub(crate) struct Inputs {
    beds: Vec<WrappedDnaNexusLink>,
    bims: Vec<WrappedDnaNexusLink>,
    fams: Vec<WrappedDnaNexusLink>,
    out_prefix: String,
}

pub(crate) struct AppBedMerge {}

impl JobStaged for JobBedMerge {
    const PREFIX: &'static str = "bedmerge";

    fn name(&self) -> String {
        format!("{}_c{}", Self::PREFIX, self.bed_bundles_of_chr.chromosome)
    }
}

fn in_file_link(conf: &Conf, bed_bundle: &BedBundle, file_type: &FileType)
                -> Result<WrappedDnaNexusLink, Error> {
    let path_string = format!("{}{}", conf.data.beds_dir, bed_bundle.file_name(file_type));
    let path = Path::new(path_string.as_str());
    dx::get_wrapped_dna_nexus_link(path)
}

impl App for AppBedMerge {
    type Job = JobBedMerge;
    type Inputs = Inputs;
    const INSTANCE_TYPE: &'static str = "mem3_ssd3_x8";
    const APP_PATH: &'static str = "/apps/bedmerge/bedmerge";
    const OUT_DIR_PATH: &'static str = "/apps/bedmerge/out/udix/";

    fn create_job_list_unfiltered(conf: &Conf) -> Result<Vec<JobBedMerge>, Error> {
        let jobs =
            get_bed_bundles_by_chrom(conf)?.into_iter().map(|bed_bundles_of_chr| {
                JobBedMerge { bed_bundles_of_chr }
            }).collect();
        Ok(jobs)
    }

    fn create_inputs_definition(job: &Self::Job, conf: &Conf) -> Result<Self::Inputs, Error> {
        let mut beds: Vec<WrappedDnaNexusLink> = Vec::new();
        let mut bims: Vec<WrappedDnaNexusLink> = Vec::new();
        let mut fams: Vec<WrappedDnaNexusLink> = Vec::new();
        for bed_bundle in &job.bed_bundles_of_chr.bed_bundles {
            beds.push(in_file_link(conf, bed_bundle, &FileType::Bed)?);
            bims.push(in_file_link(conf, bed_bundle, &FileType::Bim)?);
            fams.push(in_file_link(conf, bed_bundle, &FileType::Fam)?)
        }
        let out_prefix = job.name();
        Ok(Inputs {beds, bims, fams, out_prefix})
    }
}
