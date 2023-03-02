use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Foo {
    internal: i32,
    vecvals: Vec<i32>,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32, vecvals: Vec<i32>) -> Foo {
        Foo {
            internal: val,
            vecvals: vecvals,
        }
    }

    pub fn get(&self) -> i32 {
        self.internal
    }

    pub fn set(&mut self, val: i32) {
        self.internal = val;
    }
}