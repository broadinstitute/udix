pub struct Selection {
    pub choice: Choice,
    pub params: Params
}

pub struct Params {
    pub conf_file: String
}

pub enum Choice {
    Vcfs(Vcfs),
    Vcfs2Bed(Vcfs2Bed)
}

pub enum Vcfs {
    List, Survey
}

pub enum Vcfs2Bed {
    Prepare
}

