pub enum Selection {
    Vcfs(Vcfs),
    Vcfs2Bed(Vcfs2Bed)
}

pub enum Vcfs {
    List, Survey
}

pub enum Vcfs2Bed {
    Prepare
}

