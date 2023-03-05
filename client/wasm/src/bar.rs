use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    id: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    source: u32,
    target: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DiMultGraph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

impl DiMultGraph {
    pub fn new(val: u32) -> DiMultGraph {
        DiMultGraph {
            nodes: vec![Node { id: val }],
            links: vec![],
        }
    }

    // pub fn get(&self) -> Vec<u32> {
    //     self.ints.clone()
    // }
}

#[wasm_bindgen]
pub fn process_js_input(val: JsValue) -> JsValue {
    let g: DiMultGraph = serde_wasm_bindgen::from_value(val).unwrap();

    // do stuff with g
    // let pass = check_identity(g) ? "yay" : "nay";
    let _pass = if check_identity(g) { "yay" } else { "nay" };
    serde_wasm_bindgen::to_value("yay").unwrap()
}

pub fn check_identity(g: DiMultGraph) -> bool {
    // let nodes = g.nodes.iter().map(|n| n.id);
    let nodes: Vec<Node> = g.nodes;
    let mut id_morphs = g
        .links
        .iter()
        .filter(|l| l.source == l.target)
        .map(|l| l.source);

    return nodes.iter().all(|n| id_morphs.any(|morph| morph == n.id));
}
