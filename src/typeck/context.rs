use std::collections::HashMap;

use super::tween::{Mutability, Name};
use super::types::Type;
use super::Checker;

#[derive(Clone, Debug)]
pub struct Template {
    pub params: Vec<Name>,
    pub uninst: Type,
}

impl Template {
    pub fn mono(ty: Type) -> Self {
        Self {
            params: Vec::new(),
            uninst: ty,
        }
    }
}

impl Checker {
    pub fn instantiate(&mut self, ty: &Template) -> Type {
        let vars: HashMap<_, _> = ty
            .params
            .iter()
            .map(|name| (name, self.fresh_type(Mutability::Mutable)))
            .collect();

        ty.uninst.instantiate(&vars)
    }
}
