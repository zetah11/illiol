use log::trace;

use super::tween as mir;
use super::tween::Mutability;
use super::Checker;
use crate::hir;

impl Checker {
    pub fn infer_expr(&mut self, expr: hir::Expr) -> mir::Expr {
        let (node, ty) = match expr {
            hir::Expr::Anno(expr, ty) => {
                let ty = self.lower_type(&ty, Mutability::Mutable);
                return self.check_expr(*expr, ty);
            }

            hir::Expr::Let {
                pat,
                bound,
                then,
                elze,
            } => {
                let bound = Box::new(self.infer_expr(*bound));
                let pat = self.bind(pat, bound.anno.clone());
                let then = Box::new(self.infer_expr(*then));
                let elze = Box::new(self.check_expr(*elze, then.anno.clone()));
                let ty = then.anno.clone();
                (
                    mir::ExprNode::Let {
                        pat,
                        bound,
                        then,
                        elze,
                    },
                    ty,
                )
            }

            hir::Expr::Call(func, arg) => {
                let func = Box::new(self.infer_expr(*func));
                let (arg_ty, ret_ty) = self.as_fun_ty(func.anno.clone());
                let arg = Box::new(self.check_expr(*arg, arg_ty));
                (mir::ExprNode::Call(func, arg), ret_ty)
            }

            hir::Expr::Lit(hir::Literal::Boolean(v)) => (
                mir::ExprNode::Lit(mir::Literal::Boolean(v)),
                self.boolean_type(),
            ),

            hir::Expr::Lit(hir::Literal::Regex(v)) => (
                mir::ExprNode::Lit(mir::Literal::Regex(v)),
                self.regex_type(),
            ),

            hir::Expr::Name(name) => match self.context.get(&name) {
                Some(ty) => {
                    trace!("`{name}` infers {ty:?}");
                    (mir::ExprNode::Name(name), ty.clone())
                }
                None => (mir::ExprNode::Invalid, self.error_type()),
            },

            hir::Expr::Impossible => (mir::ExprNode::Impossible, self.bottom_type()),
            hir::Expr::Invalid => (mir::ExprNode::Invalid, self.error_type()),

            _ => {
                panic!("error: ambiguous expression")
            }
        };

        mir::Expr { node, anno: ty }
    }
}
