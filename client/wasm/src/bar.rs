use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use gloo_utils::format::JsValueSerdeExt;
use crate::graph::DiMultGraph;

#[derive(Debug)]
pub enum InvalidCategory {
    NoIdentity(usize),
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