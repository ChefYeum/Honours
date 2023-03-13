// use self::{errors::EndoCheckFail, graph::DiMultGraph, checker::check_endomorphism};

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use gloo_utils::format::JsValueSerdeExt;

use self::{graph::CompositionTable, checker::check_nothing, errors::CheckerError};

pub mod checker;
pub mod graph;
pub mod errors;

fn op_err_to_js_err(e: CheckerError) -> JsValue {
    JsValue::from_serde(&e).unwrap()
}

#[wasm_bindgen]
pub fn check_json_model(val: JsValue) -> Result<(), JsValue> {
    let m: CompositionTable = val.into_serde().unwrap();
    check_nothing(m).map_err(op_err_to_js_err)
}