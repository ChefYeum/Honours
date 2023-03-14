use std::vec;

use super::{
    errors::CheckerError,
    graph::{CompositionTable, Link},
};

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

// TODO: do they need to be public
pub fn check_ids(
    (comp_table, size): (&CompositionTable, usize),
) -> Result<(&CompositionTable, usize, Vec<usize>), CheckerError> {

    let id_morphs = (0..size)
        // Filter by left identity: for all i, f_i(i) = fi
        .filter(|f_i|comp_table
                    .get_row(*f_i)
                    .iter()
                    .all(|&fioi| fioi == *f_i))

        // Filter by right identity: for all i, i(f_i) = fi
        .filter(|f_i| comp_table
                    .get_col(*f_i)
                    .iter()
                    .all(|&iofi| iofi == *f_i));

    Ok((comp_table, size, id_morphs.collect()))
}

pub fn check_source_target(
    (comp_table, size, ids): (&CompositionTable, usize, Vec<usize>),
) -> Result<(&CompositionTable, usize, Vec<usize>, Vec<Link>), CheckerError> {
    // A vector to map target id to source and target
    let src_target_map: Vec<Link> = vec![Link {
        source: 0,
        target: 0,
    }];

    Ok((comp_table, size, ids, src_target_map))
}

// pub fn check_composition(comp_table: &CompositionTable) -> Result<(), CheckerError> {
//     Ok(())
// }

pub fn check_all(comp_table: &CompositionTable) -> Result<(), CheckerError> {
    check_size(comp_table)
        .and_then(check_ids)
        .and_then(check_source_target)
        .map(|(_, _, _, _src_target_map)| (_src_target_map))
        // check_composition(comp_table)?;
        // Discard the table and return Ok(())
        .map(|_| ())
    // Ok(())
}

// Run check_ids and print the ids
pub fn print_ids(comp_table: &CompositionTable) -> Result<(), CheckerError> {
    check_size(comp_table)
        .and_then(check_ids)
        .map(|(_, _, ids)| {
            println!("ids: {:?}", ids);
        })
        .map(|_| ())
}
