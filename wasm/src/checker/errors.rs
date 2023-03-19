use super::category::MorphID;


#[derive(Debug)]
pub enum CheckerError {
    NonSquareCompTable,
    NoValidComposition(MorphID, MorphID),
    NonAssociativeComposition(MorphID, MorphID, MorphID),
    EmptyCategory
}
