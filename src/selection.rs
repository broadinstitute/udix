pub struct Selection {
    pub choice: Choice,
    pub params: Params,
}

pub struct Params {
    pub conf_file: String,
}

pub enum DataSet {
    Vcfs,
    Beds,
}

pub enum Choice {
    Data { data_set: DataSet, data_choice: DataChoice },
    Vcfs2Bed(AppChoice),
    BedMerge(AppChoice),
    Config(Config),
}

pub enum DataChoice {
    List,
    Survey,
}

pub enum AppChoice {
    Run(RunChoice),
    Monitor,
}

pub struct RunChoice {
    pub num: Option<usize>,
    pub dry: bool,
    pub pat: Option<String>,
}

pub enum Config {
    Download
}

