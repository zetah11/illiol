mod constraint;

use log::trace;

use super::types::Type;
use super::Checker;
use crate::mir::Literal;

pub use constraint::Constraint;

impl Checker {
    pub fn solve(&mut self, ctr: Constraint) {
        match ctr {
            Constraint::FromLit(lit, ty) => {
                trace!("Solving FromLit({lit:?}, {ty:?})");
                self.solve_from_lit(lit, ty);
            }

            Constraint::Assignable(into, from) => {
                trace!("Solving Assignable({into:?}, {from:?})");
                self.check_assignable(into, from);
            }
        }
    }

    fn solve_from_lit(&mut self, lit: Literal, ty: Type) {
        match (lit, ty) {
            (Literal::Boolean(_), Type::Bool) => (),
            (Literal::Integer(val), Type::Range(lo, hi)) => {
                assert!(lo <= val && val < hi);
            }

            (Literal::Regex(_), Type::Regex) => (),
            (Literal::String(val), Type::String(pat)) => {
                assert!(pat.is_match(&val));
            }

            (lit, Type::Var(mutability, v)) => {
                if let Some(ty) = self.subst.get(&v) {
                    let ty = ty.clone();
                    self.solve_from_lit(lit, ty);
                } else {
                    self.worklist
                        .push(Constraint::FromLit(lit, Type::Var(mutability, v)))
                }
            }

            (..) => panic!("literal does not conform to type"),
        }
    }
}
