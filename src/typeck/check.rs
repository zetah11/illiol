use super::tween as mir;
use super::types::Type;
use super::Checker;
use crate::hir;

impl Checker {
    pub fn check_expr(&mut self, expr: hir::Expr, ty: Type) -> mir::Expr {
        let node = match expr {
            hir::Expr::Fun(bind, body) => {
                let (bind_ty, body_ty) = self.as_fun_ty(ty.clone());
                let bind = self.bind(bind, bind_ty);
                let body = self.check_expr(*body, body_ty);
                mir::ExprNode::Fun(bind, Box::new(body))
            }

            hir::Expr::Let {
                pat,
                bound,
                then,
                elze,
            } => {
                let bound = Box::new(self.infer_expr(*bound));
                let pat = self.bind(pat, bound.anno.clone());
                let then = Box::new(self.check_expr(*then, ty.clone()));
                let elze = Box::new(self.check_expr(*elze, ty.clone()));

                mir::ExprNode::Let {
                    pat,
                    bound,
                    then,
                    elze,
                }
            }

            hir::Expr::Lit(hir::Literal::Integer(v)) => {
                self.check_lit(mir::Literal::Integer(v), ty.clone());
                mir::ExprNode::Lit(mir::Literal::Integer(v))
            }

            hir::Expr::Lit(hir::Literal::String(v)) => {
                self.check_lit(mir::Literal::String(v.clone()), ty.clone());
                mir::ExprNode::Lit(mir::Literal::String(v))
            }

            e => {
                let inferred = self.infer_expr(e);
                self.check_assignable(ty, inferred.anno.clone());
                return inferred;
            }
        };

        mir::Expr { node, anno: ty }
    }
}
