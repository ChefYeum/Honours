use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// #[derive(Debug, PartialEq)]
// pub struct Node {
//     id: u32,
// }

// #[wasm_bindgen]
// #[derive(Debug, PartialEq)]
// pub struct Link {
//     source: u32,
//     target: u32,
// }

// Define DiMultGraph which has list of Nodes and list of Links
// Node is denoted by a number and a Link is a mapping from a Node to a Node
#[wasm_bindgen]
// #[derive(Clone, Debug, PartialEq, Eq)]
#[derive(Debug, PartialEq)]
pub struct DiMultGraph {
    // nodes: Vec<Node>,
    // links: Vec<Link>,
    ints: Vec<u32>,
}

#[wasm_bindgen]
impl DiMultGraph {
    #[wasm_bindgen(constructor)]
    // pub fn new(nodes: Vec<Node>, links: Vec<Link>) -> DiMultGraph {
    pub fn new(val: u32) -> DiMultGraph {
        // DiMultGraph { nodes, links }
        DiMultGraph { ints: vec![val] }
    }

    pub fn get(&self) -> Vec<u32> {
        self.ints.clone()
    }
}




// #[wasm_bindgen]
// pub fn check_identity(g: DiMultGraph) -> bool {
//     // let nodes = g.nodes.iter().map(|n| n.id);
//     let nodes: Vec<Node> = g.nodes.iter().cloned().collect();
//     let mut id_morphs = g
//         .links
//         .iter()
//         .filter(|l| l.source == l.target)
//         .map(|l| l.source);

//     return nodes.iter().all(|n| id_morphs.any(|morph| morph == n.id));
// }