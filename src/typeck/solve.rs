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
                trace!("Solving FromLit({:?}, {:?})", lit, ty);
                match (lit, self.types.get(&ty)) {
                    (Literal::Boolean(_), Type::Bool) => (),
                    (Literal::Integer(val), Type::Range(lo, hi)) => {
                        assert!(lo <= &val && &val < hi);
                    }
                    (Literal::Regex(_), Type::Regex) => (),
                    (Literal::String(val), Type::String(pat)) => {
                        assert!(pat.is_match(&val));
                    }

                    (..) => panic!("literal does not conform to type"),
                }
            }
        }
    }
}
