use super::tween::{Mutability, Name};
use crate::Regex;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TypeVar(pub(super) usize);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Bottom,
    Bool,
    Regex,

    Range(i64, i64),
    String(Regex),

    Arrow(Box<Type>, Box<Type>),

    Var(Mutability, TypeVar),
    Named(Name),

    Error,
}

impl Type {
    pub fn make_mutable(self) -> Self {
        self.make_mutability(Mutability::Mutable)
    }

    fn make_mutability(self, mutability: Mutability) -> Self {
        match self {
            Self::Bottom
            | Self::Bool
            | Self::Regex
            | Self::Range(..)
            | Self::String(..)
            | Self::Named(..)
            | Self::Error => self,

            Self::Arrow(from, into) => {
                let from = from.make_mutability(mutability);
                let into = into.make_mutability(mutability);
                Self::Arrow(Box::new(from), Box::new(into))
            }

            Self::Var(_, v) => Self::Var(mutability, v),
        }
    }
}
