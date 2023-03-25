use std::{marker::PhantomData};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct MorphID(pub usize);

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct ObjID(pub usize);

impl MorphID {
    pub fn id(&self) -> usize {
        self.0
    }
}

impl ObjID {
    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Morphism {
    // TOOO: make them private with getters
    pub id: MorphID,
    pub source: ObjID,
    pub target: ObjID,
}

#[derive(Debug)]
pub enum Composition {}
#[derive(Debug)]
pub enum TensorProduct {}

pub trait CarleyOp: Debug {}

impl CarleyOp for Composition {}
impl CarleyOp for TensorProduct {}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarleyTable<T: CarleyOp> {
    pub table: Box<[Box<[Option<MorphID>]>]>,

    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T: CarleyOp> CarleyTable<T> {
    pub fn new(table: Vec<Vec<Option<MorphID>>>) -> Self {
        CarleyTable {
            table: table
                .into_iter()
                .map(|row| row.into_boxed_slice())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            _phantom: PhantomData,
        }
    }

    pub fn get_all_morphs(&self) -> Box<[MorphID]> {
        (0..self.table.len())
            .map(|i| MorphID(i))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn get_row(&self, morph: MorphID) -> &[Option<MorphID>] {
        &self.table[morph.id()]
    }

    pub fn get_col(&self, id: MorphID) -> Box<[Option<MorphID>]> {
        self.table
            .iter()
            .map(|row| row[id.id()])
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn get_product(&self, f: MorphID, g: MorphID) -> Option<MorphID> {
        // Error if f or g is out of bounds
        assert!(f.id() < self.table.len() && g.id() < self.table.len());
        self.table[f.id()][g.id()]
    }
}

impl CarleyTable<Composition> {
    // Get composition f o g
    pub fn get_composition(&self, f: MorphID, g: MorphID) -> Option<MorphID> {
        self.get_product(f, g)
    }
}

impl CarleyTable<TensorProduct> {
    pub fn get_tensor_product(&self, f: MorphID, g: MorphID) -> Option<MorphID> {
        self.get_product(f, g)
    }
}
