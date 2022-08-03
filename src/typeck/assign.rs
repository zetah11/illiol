use log::trace;

use super::solve::Constraint;
use super::tween::Mutability;
use super::types::{Type, TypeVar};
use super::Checker;
use crate::mir::Literal;

impl Checker {
    pub fn check_assignable(&mut self, into: Type, from: Type) {
        trace!("Assign check {into:?} <- {from:?}");

        if into == from {
            return;
        }

        match (into, from) {
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

            (Type::Var(_, v), from) if self.subst.contains_key(&v) => {
                trace!("Unify {v:?} and {from:?}");
                let into = self.subst.get(&v).unwrap().clone();
                self.check_assignable(into, from)
            }

            (into, Type::Var(_, w)) if self.subst.contains_key(&w) => {
                trace!("Unify {into:?} and {w:?}");
                let from = self.subst.get(&w).unwrap().clone();
                self.check_assignable(into, from)
            }

            (Type::Var(Mutability::Mutable, v), from) => {
                trace!("Unify {v:?} and {from:?}");
                if self.occurs(&v, &from) {
                    trace!("Recursive types - {v:?} <- {from:?}");
                    trace!("Context {:?}", self.context);
                    panic!("recursive type!");
                }
                self.subst.insert(v, from);
            }

            (into, Type::Var(Mutability::Mutable, w)) => {
                trace!("Unify {into:?} and {w:?}");
                if self.occurs(&w, &into) {
                    trace!("Recursive types - {into:?} <- {w:?}");
                    trace!("Context {:?}", self.context);
                    panic!("recursive type!");
                }
                self.subst.insert(w, into);
            }

            (into @ Type::Var(Mutability::Immutable, _), from)
            | (into, from @ Type::Var(Mutability::Immutable, _)) => {
                self.worklist.push(Constraint::Assignable(into, from));
            }

            (Type::Arrow(t1, u1), Type::Arrow(t2, u2)) => {
                let (t1, u1) = (*t1, *u1);
                let (t2, u2) = (*t2, *u2);
                self.check_assignable(t1, t2);
                self.check_assignable(u2, u1);
            }

            (Type::Named(n), Type::Named(m)) => {
                if n != m {
                    trace!("Inequal types - {n:?} <- {m:?}");
                    panic!("inequal types");
                }
            }

            (Type::Error, _) | (_, Type::Error) => (),
            (into, from) => {
                trace!("Inequal types - {into:?} <- {from:?}");
                trace!("Context {:?}", self.context);
                panic!("inequal types")
            }
        }
    }

    pub fn as_fun_ty(&mut self, ty: Type) -> (Type, Type) {
        match ty {
            Type::Arrow(from, into) => (*from, *into),
            Type::Var(..) => {
                let from = self.fresh_type(Mutability::Mutable);
                let into = self.fresh_type(Mutability::Mutable);
                let fun_ty = self.fun_type(from.clone(), into.clone());
                self.worklist.push(Constraint::Assignable(fun_ty, ty));
                (from, into)
            }
            _ => panic!("not a function type"),
        }
    }

    pub fn check_lit(&mut self, lit: Literal, ty: Type) {
        self.worklist.push(Constraint::FromLit(lit, ty));
    }

    fn occurs(&self, v: &TypeVar, ty: &Type) -> bool {
        match ty {
            Type::Var(_, w) => v == w,
            Type::Arrow(t, u) => self.occurs(v, t) || self.occurs(v, u),

            Type::Bottom
            | Type::Bool
            | Type::Regex
            | Type::Range(..)
            | Type::String(..)
            | Type::Named(..)
            | Type::Error => false,
        }
    }
}
