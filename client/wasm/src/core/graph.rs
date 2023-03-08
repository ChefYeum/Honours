use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub source: usize,
    pub target: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DiMultGraph {
    pub node_count: usize,
    pub links: Vec<Link>,
}
