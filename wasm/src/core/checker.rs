use super::{graph::DiMultGraph, errors::InvalidCategory};

pub fn check_identity(g: DiMultGraph) -> Result<(), InvalidCategory> {
    let mut id_morphs = g
        .links
        .iter()
        .filter(|l| l.source == l.target)
        .map(|l| l.source);

    // if (1..=g.node_count)
    //     .into_iter()
    //     .all(|n| id_morphs.any(|morph| morph == n))
    // {
    //     Ok()
    // } else {
    //     Err(InvalidCategory::NoIdentity(0))
    // }
    // Write a version where we print the node that is missing an identity morphism
    Ok(for n in 1..=g.node_count {
        if !id_morphs.any(|morph| morph == n) {
            return Err(InvalidCategory::NoIdentity(n as usize));
        }
    })
}