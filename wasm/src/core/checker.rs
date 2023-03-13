use super::{errors::CheckerError, graph::CompositionTable};

// pub fn check_nothing(comp_table: &CompositionTable) -> Result<(), CheckerError>{
//     Err(CheckerError::DummyError)
// }

// TODO: check on the client side that the input is square
pub fn check_size(comp_table: &CompositionTable) -> Result<(), CheckerError> {
    let n = comp_table.table.len();

    // check if each of the rows has the length of n
    Ok(for row in comp_table.table.iter() {
        if row.len() != n {
            return Err(CheckerError::TableSizeError);
        }
    })
}

pub fn check_composition(comp_table: &CompositionTable) -> Result<(), CheckerError> {
    let n = comp_table.table.len();
    Ok(())
}


pub fn check_all(comp_table: &CompositionTable) -> Result<(), CheckerError> {
    check_size(comp_table)?;
    check_composition(comp_table)?;
    Ok(())
}