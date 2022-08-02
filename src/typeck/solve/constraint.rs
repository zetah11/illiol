use crate::mir::Literal;
use crate::typeck::types::TypeId;

#[derive(Debug)]
pub enum Constraint {
    FromLit(Literal, TypeId),
}
