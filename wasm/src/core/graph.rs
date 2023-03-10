use serde::{Serialize, Deserialize};

type LinkID = usize;
type NodeID = usize;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Link {
    pub source: usize,
    pub target: usize,
    pub id: usize
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DiMultGraph {
    pub node_count: usize,
    pub links: Vec<Link>,
}

impl DiMultGraph {
    pub fn getAdjacencyMatrix(&self) -> AdjMatrix {
        let mut matrix = vec![vec![vec![]; self.node_count + 1]; self.node_count + 1];
        for link in self.links.iter() {
            matrix[link.source][link.target].push(link)
        }
        
        AdjMatrix { matrix }
    }
}


// More efficient way to look up morphs
// Maps source & target -> morphisms
pub struct AdjMatrix<'a> {
    pub matrix: Vec<Vec<Vec<&'a Link>>>
}

impl AdjMatrix<'_> {
    pub fn get_morphs(&self, source: usize, target: usize) -> &Vec<&Link> {
        &self.matrix[source][target]
    }

    pub fn get_all_morphs_from(&self, source: usize) -> Vec<&Vec<&Link>> {
        let mut morphs = vec![];
        for i in 0..self.matrix[source].len() {
            morphs.push(&self.matrix[source][i]);
        }
        morphs
    }
}

pub struct CompositionTable {
    // Note the type does not guarantee that the table is square
    pub table: Vec<Vec<LinkID>>
}

impl CompositionTable {
    pub fn get_composition(&self, a: usize, b: usize) -> LinkID {
        self.table[a][b]
    }
}

pub struct GraphWithComp {
    pub graph: DiMultGraph,
    pub compTable: CompositionTable,
}