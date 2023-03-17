use crate::checker::graph::MorphID;

use super::{
    errors::CheckerError,
    graph::{CompositionTable},
};

// TODO: check on the client side that the input is square
// Returns the number of morphisms
pub fn check_morph_count(
    comp_table: &CompositionTable,
) -> Result<(&CompositionTable, usize), CheckerError> {
    let n = comp_table.table.len();

    for row in comp_table.table.iter() {
        if row.len() != n {
            println!("row.len(): {}, n: {}", row.len(), n);
            return Err(CheckerError::NonSquareCompTable);
        }
    }

    Ok((comp_table, n))
}

// TODO: do they need to be public
pub fn check_ids(
    (comp_table, morph_count): (&CompositionTable, usize),
) -> Result<(&CompositionTable, usize, Vec<MorphID>), CheckerError> {
    println!(
        "first row: {:?}\n",
        comp_table
            .get_row(MorphID(0))
            .iter()
            .filter_map(|fioi| fioi.as_ref())
    );


    let all_morphs = comp_table.get_all_morphs();
    let id_morphs = all_morphs.iter()
        // Check left identity
        .filter(|f_row| {
            // println!("rows: {:?}", comp_table.get_row(**f_row).iter());
            comp_table
                .get_row(**f_row).iter()
                .enumerate() // Add column index 
                // Unwrap Option and discard all None
                .filter_map(|(f_col, &fr_o_fc)| fr_o_fc.map(|fioi| (f_col, fioi)))
                .all(|(fr_i, fr_o_fc)| fr_o_fc == MorphID(fr_i))
        })
        // Check right identity
        .filter(|f_col| {
            comp_table
                .get_col(**f_col).iter()
                .enumerate()
                .filter_map(|(f_row, &fc_o_fr)| fc_o_fr.map(|fioi| (f_row, fioi)))
                .all(|(fc_i, fc_o_fr)| fc_o_fr == MorphID(fc_i))
        });

    Ok((comp_table, morph_count, id_morphs.map(|f_i| *f_i).collect()))
}

// pub fn check_source_target(
//     (comp_table, size, ids): (&CompositionTable, usize, Vec<usize>),
// ) -> Result<(&CompositionTable, usize, Vec<usize>, Vec<Link>), CheckerError> {
//     // A vector to map target id to source and target
//     let src_target_map: Vec<Link> = vec![Link {
//         source: ObjID(0),
//         target: ObjID(0),
//     }];

//     Ok((comp_table, size, ids, src_target_map))
// }

// pub fn check_composition(comp_table: &CompositionTable) -> Result<(), CheckerError> {
//     Ok(())
// }

pub fn check_all(comp_table: &CompositionTable) -> Result<(), CheckerError> {
    check_morph_count(comp_table)
        .and_then(check_ids)
        // .and_then(check_source_target)
        // .map(|(_, _, _, _src_target_map)| (_src_target_map))
        // check_composition(comp_table)?;
        // Discard the table and return Ok(())
        .map(|_| ())
    // Ok(())
}
