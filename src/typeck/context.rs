use std::collections::HashMap;

use log::trace;

use super::solve::Constraint;
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
        trace!("Instantiating {ty:?}");

        let vars: HashMap<_, _> = ty
            .params
            .iter()
            .map(|name| (name.clone(), self.fresh_type(Mutability::Mutable)))
            .collect();

        self.inst_ty(ty.uninst.clone(), &vars)
    }

    pub fn inst_ty(&mut self, ty: Type, vars: &HashMap<Name, Type>) -> Type {
        match ty {
            Type::Bottom
            | Type::Bool
            | Type::Regex
            | Type::Range(..)
            | Type::String(..)
            | Type::Error => ty.clone(),

            Type::Var(mutability, v) => {
                if let Some(ty) = self.subst.get(&v) {
                    let ty = ty.clone();
                    self.inst_ty(ty, vars)
                } else {
                    let w = self.fresh_tyvar();
                    self.worklist.push(Constraint::Instantiate(
                        vars.clone(),
                        v,
                        Type::Var(mutability, v),
                    ));
                    Type::Var(Mutability::Mutable, w)
                }
            }

            Type::Named(name) => vars
                .get(&name)
                .cloned()
                .unwrap_or_else(|| Type::Named(name.clone())),

            Type::Arrow(from, into) => {
                let from = self.inst_ty(*from, vars);
                let into = self.inst_ty(*into, vars);
                Type::Arrow(Box::new(from), Box::new(into))
            }
        }
    }
}
