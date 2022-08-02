use super::types::{Type, TypeId};
use super::Checker;
use crate::hir;

impl Checker {
    pub fn lower_type(&mut self, ty: &hir::Type) -> TypeId {
        match ty {
            hir::Type::Bool => self.boolean_type(),
            hir::Type::Regex => self.regex_type(),
            hir::Type::Range(lo, hi) => self.types.add(Type::Range(*lo, *hi)),
            hir::Type::String(pat) => self.types.add(Type::String(pat.clone())),
            hir::Type::Arrow(from, into) => {
                let from = self.lower_type(&*from);
                let into = self.lower_type(&*into);
                self.types.add(Type::Arrow(from, into))
            }
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

    pub fn regex_type(&mut self) -> TypeId {
        self.types.add(Type::Regex)
    }
}
