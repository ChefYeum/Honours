use crate::checker::category::MorphID;
use crate::checker::category::{CarleyOp, Composition, TensorProduct};

use super::{
    category::{CarleyTable, Morphism, ObjID},
    errors::CheckerError::{self, EmptyCategory, NoValidProduct, NonAssociative, NonSquareTable},
};

// TODO: check on the client side that the input is square
// Returns the number of morphisms
pub fn check_morph_count<Op: CarleyOp>(
    comp_table: &CarleyTable<Op>,
) -> Result<(&CarleyTable<Op>, usize), CheckerError<Op>> {
    let n = comp_table.table.len();

    if n == 0 {
        return Err(EmptyCategory);
    }

    for row in comp_table.table.iter() {
        if row.len() != n {
            return Err(NonSquareTable);
        }
    }

    Ok((comp_table, n))
}

// TODO: do they need to be public
pub fn check_ids<Op: CarleyOp>(
    (comp_table, morph_count): (&CarleyTable<Op>, usize),
) -> Result<(&CarleyTable<Op>, usize, Box<[MorphID]>), CheckerError<Op>> {
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

pub fn check_source_target<Op: CarleyOp>(
    (comp_table, morph_count, ids): (&CarleyTable<Op>, usize, Box<[MorphID]>),
) -> Result<(&CarleyTable<Op>, usize, Box<[MorphID]>, Box<[Morphism]>), CheckerError<Op>> {
    // A vector to map target id to source and target
    let mut links: Vec<Morphism> = Vec::new();

    // let mut srcs = vec![None; morph_count]; // Initialize with None values
    for f in comp_table.get_all_morphs().iter() {
        // There must be an id morphsm g such that g o f = f
        // The corresponding object to g is the source of the link
        // filter ids to find such g
        let gs: Box<[&MorphID]> = ids
            .iter()
            .filter(|g| comp_table.get_product(**g, *f) == Some(*f))
            .collect();

        assert_eq!(gs.len(), 1);

        // There must also be an id morphism h such that f o h = f
        // The corresponding object to h is the target of the link
        // filter ids to find such h
        let hs: Box<[&MorphID]> = ids
            .iter()
            .filter(|h| comp_table.get_product(*f, **h) == Some(*f))
            .collect();

        assert_eq!(hs.len(), 1);

        // Now we know there must be exactly one g and one h
        let h = hs[0];
        let g = gs[0];

        links.push(Morphism {
            id: MorphID(f.id()), // We use the morphism id as the link id
            source: ObjID(g.id()),
            target: ObjID(h.id()),
        });
    }

    assert_eq!(links.len(), morph_count);

    return Ok((comp_table, morph_count, ids, links.into()));
}

pub fn check_product<Op: CarleyOp>(
    (comp_table, morph_count, ids, links): (
        &CarleyTable<Op>,
        usize,
        Box<[MorphID]>,
        Box<[Morphism]>,
    ),
) -> Result<(&CarleyTable<Op>, usize, Box<[MorphID]>, Box<[Morphism]>), CheckerError<Op>> {
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
                .get_product(MorphID(f_id), MorphID(g_id))
                .is_some()
            {
                // If it exists, continue with the next g
                continue;
            } else {
                // If it doesn't exist, return an error
                return Err(NoValidProduct(MorphID(f_id), MorphID(g_id)));
            }
        }
    }
    Ok((comp_table, morph_count, ids, links))
}

pub fn check_assoc<Op: CarleyOp>(
    (comp_table, morph_count, id, links): (
        &CarleyTable<Op>,
        usize,
        Box<[MorphID]>,
        Box<[Morphism]>,
    ),
) -> Result<(&CarleyTable<Op>, usize, Box<[MorphID]>, Box<[Morphism]>), CheckerError<Op>> {
    // Check that the composition table is associative
    for f in comp_table.get_all_morphs().iter() {
        for g in comp_table.get_all_morphs().iter() {
            for h in comp_table.get_all_morphs().iter() {
                let fg = comp_table.get_product(*f, *g);
                let gh = comp_table.get_product(*g, *h);

                if fg.is_none() || gh.is_none() {
                    continue;
                }

                let f_gh = comp_table.get_product(*f, gh.unwrap());
                let fg_h = comp_table.get_product(fg.unwrap(), *h);

                if f_gh != fg_h {
                    return Err(NonAssociative(*f, *g, *h));
                }
            }
        }
    }

    return Ok((comp_table, morph_count, id, links));
}

pub fn check_tensor_distrib(
    (tensor_table, _, _, _, comp_table): (
        &CarleyTable<TensorProduct>,
        usize,
        Box<[MorphID]>,
        Box<[Morphism]>,
        &CarleyTable<Composition>,
    ),
) -> Result<(), CheckerError<TensorProduct>> {
    // Check that the composition table is associative
    for f in comp_table.get_all_morphs().iter() {
        for g in comp_table.get_all_morphs().iter() {
            for h in comp_table.get_all_morphs().iter() {
                for k in comp_table.get_all_morphs().iter() {
                    let h_o_f = comp_table.get_product(*h, *f);
                    let k_o_g = comp_table.get_product(*k, *g);

                    let k_x_g = tensor_table.get_product(*k, *g);
                    let h_x_f = tensor_table.get_product(*h, *f);

                    if h_o_f.is_none() || k_o_g.is_none() || k_x_g.is_none() || h_x_f.is_none() {
                        continue;
                    }

                    let lhs = tensor_table.get_product(k_o_g.unwrap(), h_o_f.unwrap());
                    let rhs = comp_table.get_product(k_x_g.unwrap(), h_x_f.unwrap());

                    if lhs != rhs {
                        return Err(CheckerError::NonDistributiveTensor(*k, *h, *g, *f));
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn check_category(comp_table: &CarleyTable<Composition>) -> Result<(), CheckerError<Composition>> {
    check_morph_count(comp_table)
        .and_then(check_ids)
        .and_then(check_source_target)
        .and_then(check_product)
        .and_then(check_assoc)
        .map(|_| ())
}

pub fn check_monoidal(
    tensor_table: &CarleyTable<TensorProduct>,
    comp_table: &CarleyTable<Composition>,
) -> Result<(), CheckerError<TensorProduct>> {
    check_morph_count(tensor_table)
        .and_then(check_ids)
        .and_then(check_source_target)
        .and_then(check_product)
        .and_then(check_assoc)
        .and_then(|(tensor_table, n, ids, morphs)| {
            check_tensor_distrib((tensor_table, n, ids, morphs, comp_table))
        })
        .map(|_| ())
}
