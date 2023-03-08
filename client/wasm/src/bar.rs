use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

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

#[wasm_bindgen]
impl DiMultGraph {
    #[wasm_bindgen(constructor)]
    pub fn new(node_count: usize) -> DiMultGraph {
        DiMultGraph {
            node_count: node_count,
            links: vec![],
        }
    }

    #[wasm_bindgen(getter)]
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    pub fn add_node(&mut self) {
        self.node_count += 1;
    }

    pub fn add_link(&mut self, source: usize, target: usize) {
        self.links.push(Link {
            source: source,
            target: target,
        });
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
pub fn init_model(node_count: usize) -> Result<JsValue, JsValue> {
    let g = DiMultGraph::new(node_count);
    Ok(serde_wasm_bindgen::to_value(&g)?)
}

#[wasm_bindgen]
pub fn check_model(val: JsValue) -> Result<(), JsValue> {
    let g: DiMultGraph = serde_wasm_bindgen::from_value(val).unwrap();
    check_identity(g).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    Ok(())
}
