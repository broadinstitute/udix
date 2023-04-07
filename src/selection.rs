pub struct Selection {
    pub choice: Choice,
    pub params: Params
}

pub struct Params {
    pub conf_file: String
}

pub enum Choice {
    Vcfs(Vcfs),
    Vcfs2Bed(AppChoice),
    BedMerge(AppChoice),
    Config(Config)
}

pub enum Vcfs {
    List, Survey
}

pub enum AppChoice {
    Run(RunChoice),
    Monitor
}

pub struct RunChoice {
    pub num: Option<usize>,
    pub dry: bool,
    pub pat: Option<String>
}

pub enum Config {
    Download
}

