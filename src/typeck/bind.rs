use super::Checker;
use crate::hir;
use crate::mir;
use crate::types::TypeId;

impl Checker {
    pub fn bind(&mut self, pat: hir::Pat, ty: TypeId) -> mir::Pat {
        match pat {
            hir::Pat::Constructor(..) => todo!(),

            hir::Pat::Bind(name) => {
                self.context.insert(name.clone(), ty);
                mir::Pat::Bind(name)
            }

            hir::Pat::Apply(..) => todo!(),

            hir::Pat::Lit(hir::Literal::Boolean(v)) => {
                self.check_bool(ty);
                mir::Pat::Lit(mir::Literal::Boolean(v))
            }

            hir::Pat::Lit(hir::Literal::Integer(v)) => {
                self.check_int(ty, v);
                mir::Pat::Lit(mir::Literal::Integer(v))
            }

            hir::Pat::Lit(hir::Literal::Regex(v)) => {
                self.check_regex(ty);
                mir::Pat::Lit(mir::Literal::Regex(v))
            }

            hir::Pat::Lit(hir::Literal::String(v)) => {
                self.check_str(ty, &v);
                mir::Pat::Lit(mir::Literal::String(v))
            }

            hir::Pat::Wildcard => mir::Pat::Wildcard,
        }
    }
}