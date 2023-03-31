use std::marker::PhantomData;

use super::category::{CarleyOp, MorphID};

#[derive(Debug)]
pub enum CheckerError<T: CarleyOp> {
    NonSquareTable,
    NoValidProduct(MorphID, MorphID),
    NonAssociative(MorphID, MorphID, MorphID),
    EmptyCategory,

    // Only for tensor products:
    InterchangeFail(MorphID, MorphID, MorphID, MorphID),
    _Unreachable(PhantomData<T>),
}
