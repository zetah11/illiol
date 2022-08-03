use std::collections::HashMap;

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
    pub fn instantiate(&self, vars: &HashMap<&Name, Self>) -> Self {
        match self {
            Self::Bottom
            | Self::Bool
            | Self::Regex
            | Self::Range(..)
            | Self::String(..)
            | Self::Var(..)
            | Self::Error => self.clone(),

            Self::Named(name) => vars
                .get(&name)
                .cloned()
                .unwrap_or_else(|| Self::Named(name.clone())),

            Self::Arrow(from, into) => {
                let from = from.instantiate(vars);
                let into = into.instantiate(vars);
                Self::Arrow(Box::new(from), Box::new(into))
            }
        }
    }

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
