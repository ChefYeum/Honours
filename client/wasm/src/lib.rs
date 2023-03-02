// use std::{collections::HashSet};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add_wasm(a: i32, b: i32) -> i32 {
    a + b
}


// Real stuff from here:

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node {
    pub id: u32,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Link {
    pub source: u32,
    pub target: u32,
}

// Define DiMultGraph which has list of Nodes and list of Links
// Node is denoted by a number and a Link is a mapping from a Node to a Node
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiMultGraph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

#[wasm_bindgen]
pub fn check_identity(g: DiMultGraph) -> bool {
    // let nodes = g.nodes.iter().map(|n| n.id);
    let nodes: Vec<Node> = g.nodes.iter().cloned().collect();
    let mut id_morphs = g 
        .links
        .iter()
        .filter(|l| l.source == l.target)
        .map(|l| l.source);

    return nodes.iter().all(
        |n| id_morphs
            .any(|morph| morph == n.id)
    );
}


#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Foo {
    internal: i32,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> Foo {
        Foo { internal: val }
    }

    pub fn get(&self) -> i32 {
        self.internal
    }

    pub fn set(&mut self, val: i32) {
        self.internal = val;
    }
}

// #[wasm_bindgen]
// impl DiMultGraph {
//     #[wasm_bindgen(constructor)]
//     pub fn new(nodes: Vec<Node>, links: Vec<Link>) -> DiMultGraph {
//         DiMultGraph {
//             nodes,
//             links
//         }
//     }

//     pub fn get(&self) -> (Vec<Node>, Vec<Link>) {
//         (self.nodes.clone(), self.links.clone())
//     }

//     pub fn set(&mut self, nodes: Vec<Node>, links: Vec<Link>) {
//         self.nodes = nodes;
//         self.links = links;
//     }
// }

#[cfg(test)]
mod tests {


    use crate::check_identity;
    use super::DiMultGraph;
    use super::Foo;

    // #[test]
    #[test]
    fn it_works() {
        assert_eq!(Foo::new(2), Foo::new(2));
    }

    #[test]
    fn _empty_graph() {
        let g = DiMultGraph {
                nodes: [].to_vec(),
                links: [].to_vec()
            };
        assert_eq!(check_identity(g), true);
    }
}