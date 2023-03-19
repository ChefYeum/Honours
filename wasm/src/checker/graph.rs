use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct MorphID(pub usize);

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct ObjID(pub usize);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    // TOOO: make them private with getters
    pub link_id: MorphID,
    pub source: ObjID,
    pub target: ObjID,
}

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

    // Get composition f o g
    pub fn get_composition(&self, f: MorphID, g: MorphID) -> Option<MorphID> {
        // Error if f or g is out of bounds
        assert!(f.0 < self.table.len() && g.0 < self.table.len());
        self.table[f.0][g.0]
    }
}
