use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct MorphID(pub usize);

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct ObjID(pub usize);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub source: ObjID,
    pub target: ObjID,
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
    pub table: Box<[Box<[Option<MorphID>]>]>,
}

impl CompositionTable {
    pub fn new(table: Vec<Vec<Option<MorphID>>>) -> Self {
        CompositionTable {
            table: table
                .into_iter()
                .map(|row| row.into_boxed_slice())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub fn get_row(&self, id: MorphID) -> &Box<[Option<MorphID>]> {
        &self.table[id.0]
    }

    pub fn get_col(&self, id: MorphID) -> Box<[Option<MorphID>]> {
        self.table
            .iter()
            .map(|row| row[id.0])
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn get_all_morphs(&self) -> Box<[MorphID]> {
        (0..self.table.len())
            .map(|i| MorphID(i))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}
