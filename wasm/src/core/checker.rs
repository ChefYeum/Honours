use super::{graph::CompositionTable, errors::CheckerError};

pub fn check_nothing(comp_table: CompositionTable) -> Result<(), CheckerError>{
    Err(CheckerError::DummyError)
}

pub fn check_composition(comp_table: &CompositionTable) -> Result<(), CheckerError> {
    
    Err(CheckerError::DummyError)
}