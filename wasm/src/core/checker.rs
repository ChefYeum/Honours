use super::{
    errors::{EndoCheckFail, IDCheckFail},
    graph::{GraphWithComp, DiMultGraph, Link},
};

pub fn check_endomorphism(g: DiMultGraph) -> Result<(), EndoCheckFail> {
    // Currently we assume there is only one endomorph for each obj
    // Apparently there can be many but should only have one of them satisfying the identity

    let mut id_morphs = vec![0; g.node_count];
    for l in g.links.iter() {
        if l.source == l.target {
            id_morphs[l.source - 1] += 1;
        }
    }

    Ok(for n in 0..=g.node_count {
        if id_morphs[n] == 0 {
            return Err(EndoCheckFail::NoEndo(n as usize));
        } else if id_morphs[n] > 1 {
            // TODO: search for multiple identities as well
            return Err(EndoCheckFail::ManyEndo(n as usize));
        }
    })
}

pub fn check_identity(x: GraphWithComp) -> Result<(), IDCheckFail> {
    let graph = x.graph;
    let table = x.compTable;

    let endomorphs: Vec<usize> = graph
        .links
        .into_iter()
        .filter(|l| l.source == l.target)
        .map(|l| l.source)
        .collect();

    // iterate endomorphs
    for endo in endomorphs {
        // iterate all other morphisms
        for morph in 0..=graph.node_count {
            let left_comp = table.get_composition(endo, morph);
            if left_comp != morph {
                // a is not a left identity: a o b != b
                return Err(IDCheckFail::LeftIDFail(morph));
            }

            let right_comp = table.get_composition(morph, endo);
            if right_comp != morph {
                // a is not a right identity: b o a != b
                return Err(IDCheckFail::RightIDFail(morph));
            }
        }
    }
    return Ok(());
}

pub fn check_composition(x: GraphWithComp) -> Result<(), IDCheckFail> {
    let graph = x.graph;
    let table = x.compTable;

    let adj_matrix = graph.getAdjacencyMatrix();


    for i in 0..=graph.node_count {
        for j in 0..=graph.node_count {
            let morphs_i_to_j = adj_matrix.get_morphs(i, j);
            if morphs_i_to_j.len() == 0 { continue };
            for k in 0..=graph.node_count {
                let morphs_j_to_k = adj_matrix.get_morphs(j, k);
                if morphs_j_to_k.len() == 0 { continue };

                // check morphs from i to k; these are the possible compositions
                let morphs_i_to_k = adj_matrix.get_morphs(i, k);

            }
        }
    }


    // TODO: map it to enum 
    // Maps the possible values
    let comp_table_vec: Vec<Vec<Vec<Link>>> = vec![vec![vec![]; graph.node_count + 1]; graph.node_count + 1];

    // Loop through link  f: a -> b
    for f in graph.links.iter() {
        // Morphism g: b -> x for all x
        for gs in adj_matrix.get_all_morphs_from(f.target) {
            
        }
    }

    // Check associativity of composition
    // for i in 0..=graph.node_count {





    return Ok(());
}