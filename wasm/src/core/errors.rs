// use super::graph::Link;

#[derive(Debug)]
pub enum EndoCheckFail {
    NoEndo(usize),
    ManyEndo(usize), // TODO: this needs to be searched too
}


pub enum IDCheckFail {
    // LeftIDFail(&'a Link),
    // RightIDFail(&'a Link),
}
