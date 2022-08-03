use std::collections::HashMap;

use crate::mir::{Literal, Name};
use crate::typeck::types::{Type, TypeVar};

#[derive(Debug)]
pub enum Constraint {
    FromLit(Literal, Type),
    Assignable(Type, Type),
    Instantiate(HashMap<Name, Type>, TypeVar, Type),
}
