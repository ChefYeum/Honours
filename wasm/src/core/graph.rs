use serde::{Deserialize, Serialize};

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub struct Link {
//     pub source: usize,
//     pub target: usize,
// }

// Map ID to Link
// pub struct LinkMap<'a> {
//     links: Box<[&'a Link]>,
// }

// impl LinkMap<'_> {
//     pub fn get_link(&self, id: usize) -> &Link {
//         &self.links[id]
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct CompositionTable {
    table: Box<[Box<[usize]>]>,
    // table: Box<Link>
}

// impl CompositionTable {
//     pub fn get_composition(&self, a: &Link, b: &Link) -> &Link {
//         &self.table[a.source][b.target]
//     }
// }
