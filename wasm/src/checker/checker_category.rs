use crate::checker::category::MorphID;
use crate::checker::category::{CarleyOp, Composition, TensorProduct};

use super::{
    category::{
        CarleyTable,
        Morphism, 
    },
    errors::CheckerError::{self, EmptyCategory, NoValidProduct, NonAssociative, NonSquareTable},
};

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
    (carley_table, morph_count, ids): (&CarleyTable<Op>, usize, Box<[MorphID]>),
) -> Result<(&CarleyTable<Op>, usize, Box<[MorphID]>, Box<[Morphism]>), CheckerError<Op>> {
    // A vector to map target id to source and target
    let mut links: Vec<Morphism> = Vec::new();

    // let mut srcs = vec![None; morph_count]; // Initialize with None values
    for f in carley_table.get_all_morphs().iter() {
        // There must be an id morphsm g such that g o f = f
        // The corresponding object to g is the source of the link
        // filter ids to find such g
        let gs: Box<[&MorphID]> = ids
            .iter()
            .filter(|g| carley_table.get_product(**g, *f) == Some(*f))
            .collect();

        assert_eq!(gs.len(), 1);

        // There must also be an id morphism h such that f o h = f
        // The corresponding object to h is the target of the link
        // filter ids to find such h
        let hs: Box<[&MorphID]> = ids
            .iter()
            .filter(|h| carley_table.get_product(*f, **h) == Some(*f))
            .collect();

        assert_eq!(hs.len(), 1);

        // Now we know there must be exactly one g and one h
        let h = hs[0];
        let g = gs[0];

        links.push(Morphism {
            id: MorphID(f.id()), // We use the morphism id as the link id
            // source: ObjID(g.id()),
            // target: ObjID(h.id()),
            source_id: MorphID(g.id()),
            target_id: MorphID(h.id()),
        });
    }

    assert_eq!(links.len(), morph_count);

    return Ok((carley_table, morph_count, ids, links.into()));
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
            .filter(|(_, g)| f.target_id == g.source_id);

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

pub fn check_interchange_law(
    (tensor_table, comp_table): (&CarleyTable<TensorProduct>, &CarleyTable<Composition>),
) -> Result<(), CheckerError<TensorProduct>> {
    // Check that the composition table is associative
    for f in comp_table.get_all_morphs().iter() {
        for g in comp_table.get_all_morphs().iter() {
            for h in comp_table.get_all_morphs().iter() {
                for k in comp_table.get_all_morphs().iter() {
                    let f_o_g = comp_table.get_composition(*f, *g);
                    let h_o_k = comp_table.get_composition(*h, *k);

                    let f_x_h = tensor_table.get_tensor_product(*f, *h);
                    let g_x_k = tensor_table.get_tensor_product(*g, *k);

                    if f_o_g.is_none() || h_o_k.is_none() || f_x_h.is_none() || g_x_k.is_none() {
                        continue;
                    }

                    let lhs = tensor_table.get_tensor_product(f_o_g.unwrap(), h_o_k.unwrap());
                    let rhs = comp_table.get_composition(f_x_h.unwrap(), g_x_k.unwrap());

                    if lhs != rhs {
                        return Err(CheckerError::BifunctorialityInterchangeFail(*k, *h, *g, *f));
                    }
                }
            }
        }
    }
    Ok(())
}

// Check for id_A x id_B = id_(A x B) which is required for the functoriality
pub fn check_tensor_identities(
    (carley_table, morph_count, ids, links): (
        &CarleyTable<TensorProduct>,
        usize,
        Box<[MorphID]>,
        Box<[Morphism]>,
    ),
) -> Result<
    (
        &CarleyTable<TensorProduct>,
        usize,
        Box<[MorphID]>,
        Box<[Morphism]>,
    ),
    CheckerError<TensorProduct>,
> {
    for id_a in ids.iter() {
        for id_b in ids.iter() {
            let id_a_x_id_b = carley_table.get_tensor_product(*id_a, *id_b);

            // check if this is a member of ids
            // let id_ab = ids.iter().find(|id| **id == id_a_x_id_b.unwrap());

            if id_a_x_id_b.is_some() && ids.contains(&id_a_x_id_b.unwrap()) {
                continue;
            } else {
                return Err(CheckerError::BifunctorialityTensorIDFail(*id_a, *id_b));
            }
        }
    }
    Ok((carley_table, morph_count, ids, links))
}

pub fn check_category(
    comp_table: &CarleyTable<Composition>,
) -> Result<
    (
        &CarleyTable<Composition>,
        usize,
        Box<[MorphID]>,
        Box<[Morphism]>,
    ),
    CheckerError<Composition>,
> {
    check_morph_count(comp_table)
        .and_then(check_ids)
        .and_then(check_source_target)
        .and_then(check_product)
        .and_then(check_assoc)
}

pub fn check_monoidal<'a>(
    tensor_table: &'a CarleyTable<TensorProduct>,
    comp_table: &'a CarleyTable<Composition>,
) -> Result<() , CheckerError<TensorProduct>> {
            check_morph_count(tensor_table)
            .and_then(check_ids)
            .and_then(check_source_target)
            // Bifunctoriality - Identity of tensor
            // If you represent identity morphisms as their corresponding objects:
            // left-right identity of tensor: id_A x id_B = id_(A x B)
            // which is the identity of some object and that object is A x B
            .and_then(check_tensor_identities)
            // // Bifunctoriality - Interchange law
            // // when both compositions exist: (f o g) x (h o k) = (f x h) o (g x k)
    
            // ## Domain & Codomain check
            // If you represent identity morphisms as their corresponding objects:
            // id(dom(f x g)) = id(dom(f)) x id(dom(g))
            // id(cod(f x g)) = id(cod(f)) x id(cod(g))
            .and_then(|(tensor_table, morph_count, ids, links)| {
                for f in links.iter() {
                    for g in links.iter() {
                        let lhs = tensor_table.get_tensor_product(f.id, g.id);
                        let rhs = tensor_table.get_tensor_product(f.source_id, g.source_id);
    
                        if lhs != rhs {
                            return Err(CheckerError::TensorDomainCheckFail(f.id, g.id));
                        }
    
                    }
                }
                return Ok((tensor_table, morph_count, ids, links));
    
            // ## Associativity of tensor
            // If you represent identity morphisms as their corresponding objects, no need to separately check objects
            })
            .and_then(check_assoc)
            .and_then(
                |(tensor_table, _, _, _)| {
                    check_interchange_law((tensor_table, comp_table))
                }
            )

}
