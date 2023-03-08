use serde::{Serialize, Deserialize};
// use wasm_bindgen::prelude::wasm_bindgen;

// #[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub source: usize,
    pub target: usize,
}

// #[wasm_bindgen] // TODO: check if I need this
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DiMultGraph {
    pub node_count: usize,
    pub links: Vec<Link>,
}

// TODO: do I need getters
// impl DiMultGraph {
//     pub fn new(node_count: usize) -> DiMultGraph {
//         DiMultGraph {
//             node_count: node_count,
//             links: vec![],
//         }
//     }

//     pub fn node_count(&self) -> usize {
//         self.node_count
//     }
// }
