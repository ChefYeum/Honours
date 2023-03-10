// use self::{errors::EndoCheckFail, graph::DiMultGraph, checker::check_endomorphism};

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use gloo_utils::format::JsValueSerdeExt;

use self::errors::EndoCheckFail;

pub mod checker;
pub mod graph;
pub mod errors;

fn op_err_to_js_err(e: EndoCheckFail) -> JsValue {
    JsValue::from_str(&format!("{:?}", e))
}

#[wasm_bindgen]
pub fn check_json_model(val: JsValue) -> Result<(), JsValue> {
    // let g: DiMultGraph = val.into_serde().unwrap();
    // check_endomorphism(g).map_err(op_err_to_js_err)
    Ok(())
}