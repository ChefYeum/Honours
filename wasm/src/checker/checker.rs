use crate::checker::graph::MorphID;

use super::{
    errors::CheckerError::{
        self, NoValidComposition, NonAssociativeComposition, NonSquareCompTable,
    },
    graph::{CompositionTable, Link, ObjID},
};

// TODO: check on the client side that the input is square
// Returns the number of morphisms
pub fn check_morph_count(
    comp_table: &CompositionTable,
) -> Result<(&CompositionTable, usize), CheckerError> {
    let n = comp_table.table.len();

    for row in comp_table.table.iter() {
        if row.len() != n {
            return Err(NonSquareCompTable);
        }
    }

    Ok((comp_table, n))
}

// TODO: do they need to be public
pub fn check_ids(
    (comp_table, morph_count): (&CompositionTable, usize),
) -> Result<(&CompositionTable, usize, Box<[MorphID]>), CheckerError> {
    println!(
        "first row: {:?}\n",
        comp_table
            .get_row(MorphID(0))
            .iter()
            .filter_map(|fioi| fioi.as_ref())
    );

    // let all_morphs = ;
    let id_morphs = comp_table
        .get_all_morphs()
        .iter()
        // Check left identity
        .filter(|f_row| {
            // println!("rows: {:?}", comp_table.get_row(**f_row).iter());
            comp_table
                .get_row(**f_row)
                .iter()
                .enumerate() // Add column index
                // Unwrap Option and discard all None
                .filter_map(|(f_col, &fr_o_fc)| fr_o_fc.map(|fioi| (f_col, fioi)))
                .all(|(fr_i, fr_o_fc)| fr_o_fc == MorphID(fr_i))
        })
        // Check right identity
        .filter(|f_col| {
            comp_table
                .get_col(**f_col)
                .iter()
                .enumerate()
                .filter_map(|(f_row, &fc_o_fr)| fc_o_fr.map(|fioi| (f_row, fioi)))
                .all(|(fc_i, fc_o_fr)| fc_o_fr == MorphID(fc_i))
        })
        .map(|&f| f)
        .collect::<Vec<_>>()
        .into_boxed_slice();

    Ok((comp_table, morph_count, id_morphs))
}

pub fn check_source_target(
    (comp_table, morph_count, ids): (&CompositionTable, usize, Box<[MorphID]>),
) -> Result<(&CompositionTable, usize, Box<[MorphID]>, Box<[Link]>), CheckerError> {
    // A vector to map target id to source and target
    let mut links: Vec<Link> = Vec::new();

    // let mut srcs = vec![None; morph_count]; // Initialize with None values
    for f in comp_table.get_all_morphs().iter() {
        // There must be an id morphsm g such that g o f = f
        // The corresponding object to g is the source of the link
        // filter ids to find such g
        let gs: Box<[&MorphID]> = ids
            .iter()
            .filter(|g| comp_table.get_composition(**g, *f) == Some(*f))
            .collect();

        assert_eq!(gs.len(), 1);

        // There must also be an id morphism h such that f o h = f
        // The corresponding object to h is the target of the link
        // filter ids to find such h
        let hs: Box<[&MorphID]> = ids
            .iter()
            .filter(|h| comp_table.get_composition(*f, **h) == Some(*f))
            .collect();

        assert_eq!(hs.len(), 1);

        // Now we know there must be exactly one g and one h
        let h = hs[0];
        let g = gs[0];

        links.push(Link {
            link_id: MorphID(f.0), // We use the morphism id as the link id
            source: ObjID(g.0),
            target: ObjID(h.0),
        });
    }

    assert_eq!(links.len(), morph_count);

    return Ok((comp_table, morph_count, ids, links.into()));
}

pub fn check_composition(
    (comp_table, morph_count, ids, links): (&CompositionTable, usize, Box<[MorphID]>, Box<[Link]>),
) -> Result<(&CompositionTable, usize, Box<[MorphID]>, Box<[Link]>), CheckerError> {
    // Enumerate links so that we also have MorphID
    // TODO: make a struct for Vec<Link> that can return the numeration of f_id
    for (f_id, f) in links.iter().enumerate() {
        // Let f: A -> B
        // get all morphisms g: B -> C from links

        // Get the following while keeping the MorphID as well as the Link
        let gs = links
            .iter()
            .enumerate()
            .filter(|(_, g)| f.target == g.source);

        for (g_id, _) in gs {
            // Check if the composition f ∘ g exists in the table
            if comp_table
                .get_composition(MorphID(f_id), MorphID(g_id))
                .is_some()
            {
                // If it exists, continue with the next g
                continue;
            } else {
                // If it doesn't exist, return an error
                return Err(NoValidComposition(MorphID(f_id), MorphID(g_id)));
            }
        }
    }
    Ok((comp_table, morph_count, ids, links))
}

pub fn check_assoc(
    (comp_table, _, _, _): (
        &CompositionTable,
        usize,
        Box<[MorphID]>,
        Box<[Link]>,
    ),
) -> Result<(), CheckerError> {
    // Check that the composition table is associative
    for f in comp_table.get_all_morphs().iter() {
        for g in comp_table.get_all_morphs().iter() {
            for h in comp_table.get_all_morphs().iter() {
                let fg = comp_table.get_composition(*f, *g);
                let gh = comp_table.get_composition(*g, *h);

                if fg.is_none() || gh.is_none() {
                    continue;
                }

                let f_gh = comp_table.get_composition(*f, gh.unwrap());
                let fg_h = comp_table.get_composition(fg.unwrap(), *h);

                if f_gh != fg_h {
                    return Err(NonAssociativeComposition(*f, *g, *h));
                }
            }
        }
    }

    return Ok(());
}

pub fn check_all(comp_table: &CompositionTable) -> Result<(), CheckerError> {
    check_morph_count(comp_table)
        .and_then(check_ids)
        .and_then(check_source_target)
        .and_then(check_composition)
        .and_then(check_assoc)
        .map(|_| ())
}
