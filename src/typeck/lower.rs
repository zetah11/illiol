use super::tween::Mutability;
use super::types::{Type, TypeId};
use super::Checker;
use crate::hir;

impl Checker {
    pub fn lower_type(&mut self, ty: &hir::Type, mutability: Mutability) -> TypeId {
        match ty {
            hir::Type::Bool => self.boolean_type(),
            hir::Type::Regex => self.regex_type(),
            hir::Type::Range(lo, hi) => self.types.add(Type::Range(*lo, *hi)),
            hir::Type::String(pat) => self.types.add(Type::String(pat.clone())),
            hir::Type::Arrow(from, into) => {
                let from = self.lower_type(&*from, mutability);
                let into = self.lower_type(&*into, mutability);
                self.types.add(Type::Arrow(from, into))
            }
            hir::Type::Wildcard => self.fresh_type(mutability),
            hir::Type::Invalid => self.error_type(),
        }
    }

    pub fn boolean_type(&mut self) -> TypeId {
        self.types.add(Type::Bool)
    }

    pub fn bottom_type(&mut self) -> TypeId {
        self.types.add(Type::Bottom)
    }

    pub fn error_type(&mut self) -> TypeId {
        self.types.add(Type::Error)
    }

    pub fn fresh_type(&mut self, mutability: Mutability) -> TypeId {
        let v = self.fresh_tyvar();
        self.types.add(Type::Var(mutability, v))
    }

    pub fn regex_type(&mut self) -> TypeId {
        self.types.add(Type::Regex)
    }
}
