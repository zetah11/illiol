use crate::mir::Literal;
use crate::typeck::types::Type;

#[derive(Debug)]
pub enum Constraint {
    FromLit(Literal, Type),
    Assignable(Type, Type),
}
