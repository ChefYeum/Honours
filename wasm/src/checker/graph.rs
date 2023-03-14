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
    pub table: Box<[Box<[usize]>]>,
}

impl CompositionTable {
    pub fn new(table:
        Vec<Vec<usize>>
    ) -> Self {
        let table = table
            .into_iter()
            .map(|row| row.into_boxed_slice())
            .collect::<Box<[_]>>();
        Self { table }
    }

    // pub fn rows(&self) -> impl Iterator<Item = &Box<[usize]>> {
    //     self.table.iter()
    // }

    pub fn get_row(&self, id: usize) -> &Box<[usize]> {
        &self.table[id]
    }

    pub fn get_col(&self, id: usize) -> Box<[usize]> {
        self.table
            .iter()
            .map(|row| row[id])
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    // pub fn cols(&self) -> impl Iterator<Item = Box<[usize]>> {
    //     let mut cols = Vec::new();

    //     for i in 0..self.table.len() {
    //         cols.push(
    //             self.table
    //                 .iter()
    //                 .map(|row| row[i])
    //                 .collect::<Vec<_>>()
    //                 .into_boxed_slice(),
    //         );
    //     }

    //     cols.into_iter()
    // }
//     pub fn get_composition(&self, a: &Link, b: &Link) -> &Link {
//         &self.table[a.source][b.target]
//     }
}
