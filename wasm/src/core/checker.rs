use super::{errors::CheckerError, graph::CompositionTable};

// pub fn check_nothing(comp_table: &CompositionTable) -> Result<(), CheckerError>{
//     Err(CheckerError::DummyError)
// }

// TODO: check on the client side that the input is square
pub fn check_size(
    comp_table: &CompositionTable,
) -> Result<(&CompositionTable, usize), CheckerError> {
    let n = comp_table.table.len();

    for row in comp_table.table.iter() {
        if row.len() != n {
            return Err(CheckerError::TableSizeError);
        }
    }

    Ok((comp_table, n))
}

pub fn check_ids(
    (comp_table, size): (&CompositionTable, usize),
) -> Result<(&CompositionTable, usize, Vec<usize>), CheckerError> {
    // Assume identities are 0, 1, 2
    Ok((comp_table, size, vec![0, 1, 2])) // TODO: implement
}

// pub fn check_source_target(comp_table: &CompositionTable, identities: Vec<usize>) -> Result<(), CheckerError> {
//     Ok(())
// }

// pub fn check_composition(comp_table: &CompositionTable) -> Result<(), CheckerError> {
//     Ok(())
// }

pub fn check_all(comp_table: &CompositionTable) -> Result<(), CheckerError> {
    check_size(comp_table).and_then(check_ids)?;
    // check_composition(comp_table)?;
    Ok(())
}
