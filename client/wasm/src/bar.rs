use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use gloo_utils::format::JsValueSerdeExt;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum InvalidCategory {
    NoIdentity(usize),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    source: usize,
    target: usize,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DiMultGraph {
    node_count: usize,
    links: Vec<Link>,
}

// #[wasm_bindgen]
impl DiMultGraph {
    pub fn new(node_count: usize) -> DiMultGraph {
        DiMultGraph {
            node_count: node_count,
            links: vec![],
        }
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }

}

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

fn op_err_to_js_err(e: InvalidCategory) -> JsValue {
    JsValue::from_str(&format!("{:?}", e))
}

#[wasm_bindgen]
pub fn check_json_model(val: JsValue) -> Result<(), JsValue> {
    let g: DiMultGraph = val.into_serde().unwrap();
    check_identity(g).map_err(op_err_to_js_err)
}