use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    source: u32,
    target: u32,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DiMultGraph {
    node_count: u32,
    links: Vec<Link>,
}

#[wasm_bindgen]
impl DiMultGraph {
    pub fn new() -> DiMultGraph {
        DiMultGraph {
            node_count: 0,
            links: vec![],
        }
    }

    #[wasm_bindgen(getter)]
    pub fn node_count(&self) -> u32 {
        self.node_count
    }

    // pub fn add_node(&mut self) {
    //     self.node_count += 1;
    // }

    // pub fn add_link(&mut self, source: u32, target: u32) {
    //     self.links.push(Link { source, target });
    // }
}

pub fn check_identity(g: DiMultGraph) -> Result<(), i32> {
    // let nodes = g.nodes.iter().map(|n| n.id);
    let mut id_morphs = g
        .links
        .iter()
        .filter(|l| l.source == l.target)
        .map(|l| l.source);

    if (1..=g.node_count)
        .into_iter()
        .all(|n| id_morphs.any(|morph| morph == n)) { Ok(()) } else { Err(1) }
}

// #[wasm_bindgen]
// pub fn process_js_input(val: JsValue) -> JsValue {
//     let g: DiMultGraph = serde_wasm_bindgen::from_value(val).unwrap();

//     // do stuff with g
//     // let pass = check_identity(g) ? "yay" : "nay";
//     // let _pass = check_identity(g);

//     serde_wasm_bindgen::to_value(&check_identity(g)).unwrap()
//     // serde_wasm_bindgen::to_value(g).unwrap()
// }

#[wasm_bindgen]
pub fn init_model() -> Result<JsValue, JsValue> {
    let g = DiMultGraph::new();
    Ok(serde_wasm_bindgen::to_value(&g)?)
}

#[wasm_bindgen]
pub fn check_model(val: JsValue) -> Result<(), JsValue> {
    let g: DiMultGraph = serde_wasm_bindgen::from_value(val).unwrap();
    // check_identity(g).or_else(|e| Err(JsValue::from(e)))?
    check_identity(g)?;
    Ok(())
}
