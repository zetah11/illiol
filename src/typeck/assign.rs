use crate::mir::Literal;

use super::solve::Constraint;
use super::tween::Mutability;
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

            (Type::Var(_, v), _) if self.subst.contains_key(v) => {
                let &into = self.subst.get(v).unwrap();
                self.check_assignable(into, from)
            }

            (_, Type::Var(_, w)) if self.subst.contains_key(w) => {
                let &from = self.subst.get(w).unwrap();
                self.check_assignable(into, from)
            }

            (Type::Var(Mutability::Mutable, v), _) => {
                self.subst.insert(*v, from);
            }

            (_, Type::Var(Mutability::Mutable, w)) => {
                self.subst.insert(*w, into);
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
