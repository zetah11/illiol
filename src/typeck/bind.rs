use super::context::Template;
use super::tween;
use super::types::Type;
use super::Checker;
use crate::hir;

impl Checker {
    pub fn bind(&mut self, pat: hir::Pat, ty: Type) -> tween::Pat {
        match pat {
            hir::Pat::Constructor(..) => todo!(),

            hir::Pat::Bind(name) => {
                self.context.insert(name.clone(), Template::mono(ty));
                tween::Pat::Bind(name)
            }

            hir::Pat::Apply(..) => todo!(),

            hir::Pat::Lit(hir::Literal::Boolean(v)) => {
                self.check_lit(tween::Literal::Boolean(v), ty);
                tween::Pat::Lit(tween::Literal::Boolean(v))
            }

            hir::Pat::Lit(hir::Literal::Integer(v)) => {
                self.check_lit(tween::Literal::Integer(v), ty);
                tween::Pat::Lit(tween::Literal::Integer(v))
            }

            hir::Pat::Lit(hir::Literal::Regex(v)) => {
                self.check_lit(tween::Literal::Regex(v.clone()), ty);
                tween::Pat::Lit(tween::Literal::Regex(v))
            }

            hir::Pat::Lit(hir::Literal::String(v)) => {
                self.check_lit(tween::Literal::String(v.clone()), ty);
                tween::Pat::Lit(tween::Literal::String(v))
            }

            hir::Pat::Wildcard => tween::Pat::Wildcard,
        }
    }
}
