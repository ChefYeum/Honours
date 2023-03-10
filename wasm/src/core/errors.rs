#[derive(Debug)]
pub enum EndoCheckFail {
    NoEndo(usize),
    ManyEndo(usize), // TODO: this needs to be searched too
}


pub enum IDCheckFail {
    LeftIDFail(usize),
    RightIDFail(usize),
}
