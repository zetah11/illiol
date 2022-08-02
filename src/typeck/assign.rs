use crate::mir::Literal;

use super::solve::Constraint;
use super::types::{Type, TypeId};
use super::Checker;

impl Checker {
    pub fn check_assignable(&mut self, into: TypeId, from: TypeId) {
        match (self.types.get(&into), self.types.get(&from)) {
            (_, Type::Bottom) => (),
            (Type::Bool, Type::Bool) => (),
            (Type::Regex, Type::Regex) => (),
            (Type::Range(lo1, hi1), Type::Range(lo2, hi2)) => {
                assert_eq!(lo1, lo2);
                assert_eq!(hi1, hi2);
            }
            (Type::String(pat1), Type::String(pat2)) => {
                assert_eq!(pat1, pat2);
            }
            (Type::Error, _) | (_, Type::Error) => (),
            _ => panic!("inequal types"),
        }
    }

    pub fn as_fun_ty(&mut self, ty: TypeId) -> (TypeId, TypeId) {
        match self.types.get(&ty) {
            Type::Arrow(from, into) => (*from, *into),
            _ => panic!("not a function type"),
        }
    }

    pub fn check_lit(&mut self, lit: Literal, ty: TypeId) {
        self.worklist.push(Constraint::FromLit(lit, ty));
    }
}
