use log::trace;

use super::tween;
use super::types::Type;
use super::Checker;
use crate::mir;
use crate::types as varless;

impl Checker {
    pub fn substitute(&mut self, expr: tween::Expr) -> mir::Expr {
        let node = match expr.node {
            tween::ExprNode::Fun(pat, body) => {
                let pat = self.subst_pat(pat);
                let body = Box::new(self.substitute(*body));
                mir::ExprNode::Fun(pat, body)
            }

            tween::ExprNode::Let {
                pat,
                bound,
                then,
                elze,
            } => {
                let pat = self.subst_pat(pat);
                let bound = Box::new(self.substitute(*bound));
                let then = Box::new(self.substitute(*then));
                let elze = Box::new(self.substitute(*elze));

                mir::ExprNode::Let {
                    pat,
                    bound,
                    then,
                    elze,
                }
            }

            tween::ExprNode::Call(func, expr) => {
                let func = Box::new(self.substitute(*func));
                let expr = Box::new(self.substitute(*expr));

                mir::ExprNode::Call(func, expr)
            }

            tween::ExprNode::Lit(lit) => mir::ExprNode::Lit(lit),
            tween::ExprNode::Name(name) => mir::ExprNode::Name(name),
            tween::ExprNode::Instantiated(name) => mir::ExprNode::Instantiated(name),
            tween::ExprNode::Impossible => mir::ExprNode::Impossible,
            tween::ExprNode::Invalid => mir::ExprNode::Invalid,
        };

        mir::Expr {
            node,
            anno: self.subst_type(expr.anno),
        }
    }

    pub fn subst_type(&mut self, ty: Type) -> varless::TypeId {
        let ty = match ty {
            Type::Bottom => varless::Type::Bottom,
            Type::Bool => varless::Type::Bool,
            Type::Regex => varless::Type::Regex,
            Type::Range(lo, hi) => varless::Type::Range(lo, hi),
            Type::String(pat) => varless::Type::String(pat),
            Type::Arrow(from, into) => {
                varless::Type::Arrow(self.subst_type(*from), self.subst_type(*into))
            }
            Type::Var(_, v) => {
                trace!("Substituting typevar {v:?}");
                if let Some(ty) = self.subst.get(&v) {
                    let ty = ty.clone();
                    return self.subst_type(ty);
                } else {
                    trace!("Unsolved type var {v:?}");
                    trace!("Subst {:?}", self.subst);
                    panic!("unsolved type var!");
                }
            }
            Type::Named(name) => varless::Type::Named(name),
            Type::Error => varless::Type::Error,
        };

        if let Some(id) = self.lower.get_by_right(&ty) {
            *id
        } else {
            let id = varless::TypeId(self.lower.len());
            self.lower.insert(id, ty);
            id
        }
    }

    fn subst_pat(&self, pat: tween::Pat) -> mir::Pat {
        match pat {
            tween::Pat::Constructor(name) => mir::Pat::Constructor(name),
            tween::Pat::Bind(name) => mir::Pat::Bind(name),
            tween::Pat::Apply(ctr, arg) => {
                let ctr = Box::new(self.subst_pat(*ctr));
                let arg = Box::new(self.subst_pat(*arg));

                mir::Pat::Apply(ctr, arg)
            }
            tween::Pat::Lit(lit) => mir::Pat::Lit(lit),
            tween::Pat::Wildcard => mir::Pat::Wildcard,
        }
    }
}
