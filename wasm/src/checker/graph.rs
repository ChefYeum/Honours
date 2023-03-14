use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub source: usize,
    pub target: usize,
}

// Map ID to Link
// pub struct LinkMap<'a> {
//     links: Box<[&'a Link]>,
// }

// impl LinkMap<'_> {
//     pub fn get_link(&self, id: usize) -> &Link {
//         &self.links[id]
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct CompositionTable {
    pub table: Box<[Box<[Option<usize>]>]>,
}

impl CompositionTable {
    pub fn new(table: Vec<Vec<Option<usize>>>) -> Self {
        CompositionTable {
            table: table
                .into_iter()
                .map(|row| row.into_boxed_slice())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub fn get_row(&self, id: usize) -> &Box<[Option<usize>]> {
        &self.table[id]
    }

    pub fn get_col(&self, id: usize) -> Box<[Option<usize>]> {
        self.table
            .iter()
            .map(|row| row[id])
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

