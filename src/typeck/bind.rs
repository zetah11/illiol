use super::tween as mir;
use super::types::Type;
use super::Checker;
use crate::hir;

impl Checker {
    pub fn bind(&mut self, pat: hir::Pat, ty: Type) -> mir::Pat {
        match pat {
            hir::Pat::Constructor(..) => todo!(),

            hir::Pat::Bind(name) => {
                self.context.insert(name.clone(), ty);
                mir::Pat::Bind(name)
            }

            hir::Pat::Apply(..) => todo!(),

            hir::Pat::Lit(hir::Literal::Boolean(v)) => {
                self.check_lit(mir::Literal::Boolean(v), ty);
                mir::Pat::Lit(mir::Literal::Boolean(v))
            }

            hir::Pat::Lit(hir::Literal::Integer(v)) => {
                self.check_lit(mir::Literal::Integer(v), ty);
                mir::Pat::Lit(mir::Literal::Integer(v))
            }

            hir::Pat::Lit(hir::Literal::Regex(v)) => {
                self.check_lit(mir::Literal::Regex(v.clone()), ty);
                mir::Pat::Lit(mir::Literal::Regex(v))
            }

            hir::Pat::Lit(hir::Literal::String(v)) => {
                self.check_lit(mir::Literal::String(v.clone()), ty);
                mir::Pat::Lit(mir::Literal::String(v))
            }

            hir::Pat::Wildcard => mir::Pat::Wildcard,
        }
    }
}
