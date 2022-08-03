use super::tween::Mutability;
use super::types::Type;
use super::Checker;
use crate::hir;

impl Checker {
    pub fn lower_type(&mut self, ty: &hir::Type, mutability: Mutability) -> Type {
        match ty {
            hir::Type::Bool => self.boolean_type(),
            hir::Type::Regex => self.regex_type(),
            hir::Type::Range(lo, hi) => Type::Range(*lo, *hi),
            hir::Type::String(pat) => Type::String(pat.clone()),
            hir::Type::Arrow(from, into) => {
                let from = self.lower_type(&*from, mutability);
                let into = self.lower_type(&*into, mutability);
                self.fun_type(from, into)
            }
            hir::Type::Named(name) => Type::Named(name.clone()),
            hir::Type::Wildcard => self.fresh_type(mutability),
            hir::Type::Invalid => self.error_type(),
        }
    }

    pub fn boolean_type(&mut self) -> Type {
        Type::Bool
    }

    pub fn bottom_type(&mut self) -> Type {
        Type::Bottom
    }

    pub fn error_type(&mut self) -> Type {
        Type::Error
    }

    pub fn fresh_type(&mut self, mutability: Mutability) -> Type {
        let v = self.fresh_tyvar();
        Type::Var(mutability, v)
    }

    pub fn fun_type(&mut self, from: Type, into: Type) -> Type {
        Type::Arrow(Box::new(from), Box::new(into))
    }

    pub fn regex_type(&mut self) -> Type {
        Type::Regex
    }
}
