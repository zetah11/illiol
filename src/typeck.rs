mod assign;
mod bind;
mod check;
mod infer;
mod lower;
mod solve;
mod substitute;
mod tween;
mod types;

use std::collections::HashMap;

use bimap::BiMap;
use log::debug;

use self::solve::Constraint;
use self::tween::Mutability;
use self::types::TypeVar;
use crate::hir;
use crate::mir;
use crate::types as varless;

use types::{TypeId, Types};

pub fn typeck(prog: hir::Decls) -> mir::Program {
    let mut checker = Checker::new();
    for (name, item) in prog.values.iter() {
        checker.declare(name.clone(), &item.anno);
    }

    let mut values = HashMap::with_capacity(prog.values.len());
    for (name, item) in prog.values {
        let body = checker.define(&name, item.body);
        values.insert(name, body);
    }

    checker.solve_constraints();

    // TODO: ew
    #[allow(clippy::needless_collect)]
    let values: Vec<_> = values
        .into_iter()
        .map(|(name, expr)| (name, checker.substitute(expr)))
        .collect();

    let (context, types, new_ids) = checker.ctx_and_types();

    debug!("Fixing up value defs");
    let values = values
        .into_iter()
        .map(|(name, expr)| (name, fixup_expr(expr, &new_ids)))
        .collect();

    mir::Program {
        context,
        types,
        decls: mir::Decls { values },
    }
}

#[derive(Debug)]
struct Checker {
    context: HashMap<mir::Name, TypeId>,
    types: Types,
    subst: HashMap<TypeVar, TypeId>,

    curr_tyvar: TypeVar,
    worklist: Vec<Constraint>,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
            types: Types::new(),
            subst: HashMap::new(),

            curr_tyvar: TypeVar(0),
            worklist: Vec::new(),
        }
    }

    pub fn declare(&mut self, name: mir::Name, ty: &hir::Type) {
        let ty = self.lower_type(ty, Mutability::Immutable);
        self.context.insert(name, ty);
    }

    pub fn define(&mut self, name: &mir::Name, expr: hir::Expr) -> tween::Expr {
        let &ty = self.context.get(name).unwrap();
        self.types.make_mutable(&ty);
        let item = self.check_expr(expr, ty);
        self.types.make_immutable(&ty);

        item
    }

    pub fn solve_constraints(&mut self) {
        loop {
            let worklist: Vec<_> = self.worklist.drain(..).collect();
            let prev = worklist.len();

            if prev == 0 {
                debug!("Done solving");
                break;
            }

            debug!("Solve loop; {} constraints to solve", prev);

            for ctr in worklist {
                self.solve(ctr);
            }

            if self.worklist.len() >= prev {
                panic!("constraint solving made no progress!")
            }
        }
    }

    pub fn ctx_and_types(
        mut self,
    ) -> (
        HashMap<mir::Name, varless::TypeId>,
        varless::Types,
        HashMap<varless::TypeId, varless::TypeId>,
    ) {
        debug!("Substituting type context");
        let ctx: HashMap<_, _> = self.context.drain().collect();
        let ctx = ctx
            .into_iter()
            .map(|(name, ty)| (name, self.subst_typeid(ty)))
            .collect();

        // NOTE: Although `typeck::types::Types` memoizes its types, we may
        // quickly end up with duplicate types as we solve various constraints.
        // For instance, if we have two types `t = 0 .. 10` and `u = $1`, as
        // well as the constraint `t = u`, we will end up with two names for the
        // same type `t` (`t` and `u`). As we memoize the substituted types,
        // this will be reduced down to a single type `t` - but this means that
        // any ids in other type defs or values will be out of date (e.g.
        // referring to a `u` that no longer exists). To combat this, we keep
        // a surjective map from old names to the new names, and use this to
        // "fix up" the out-of-date names.
        debug!("Substituting type definitions");
        let mut types = BiMap::new();
        let mut new_ids = HashMap::new();

        for (id, ty) in self.types.iter() {
            let subst = self.subst_type(ty.clone());
            let new_id = if let Some(new_id) = types.get_by_right(&subst) {
                *new_id
            } else {
                let id = varless::TypeId(types.len());
                types.insert(id, subst);
                id
            };
            new_ids.insert(varless::TypeId(id.0), new_id);
        }

        debug!("Fixing up types");
        let mut new_types = varless::Types::new();

        for (id, ty) in types {
            use varless::Type;
            let ty = match ty {
                Type::Bottom
                | Type::Bool
                | Type::Regex
                | Type::Range(..)
                | Type::String(..)
                | Type::Error => ty,

                Type::Arrow(t, u) => {
                    let t = new_ids.get(&t).copied().unwrap_or(t);
                    let u = new_ids.get(&u).copied().unwrap_or(u);
                    Type::Arrow(t, u)
                }
            };

            new_types.add(id, ty);
        }

        (ctx, new_types, new_ids)
    }

    fn fresh_tyvar(&mut self) -> TypeVar {
        let v = self.curr_tyvar;
        self.curr_tyvar = TypeVar(self.curr_tyvar.0 + 1);
        v
    }
}

fn fixup_expr(expr: mir::Expr, new_ids: &HashMap<varless::TypeId, varless::TypeId>) -> mir::Expr {
    use mir::{Expr, ExprNode};

    let node = match expr.node {
        ExprNode::Fun(pat, body) => {
            let pat = fixup_pat(pat, new_ids);
            let body = Box::new(fixup_expr(*body, new_ids));
            ExprNode::Fun(pat, body)
        }

        ExprNode::Let {
            pat,
            bound,
            then,
            elze,
        } => {
            let pat = fixup_pat(pat, new_ids);
            let bound = Box::new(fixup_expr(*bound, new_ids));
            let then = Box::new(fixup_expr(*then, new_ids));
            let elze = Box::new(fixup_expr(*elze, new_ids));
            ExprNode::Let {
                pat,
                bound,
                then,
                elze,
            }
        }

        ExprNode::Tuple(args) => {
            let args = args
                .into_iter()
                .map(|arg| fixup_expr(arg, new_ids))
                .collect();
            ExprNode::Tuple(args)
        }

        ExprNode::Call(func, arg) => {
            let func = Box::new(fixup_expr(*func, new_ids));
            let arg = Box::new(fixup_expr(*arg, new_ids));
            ExprNode::Call(func, arg)
        }

        ExprNode::Lit(lit) => ExprNode::Lit(lit),
        ExprNode::Name(name) => ExprNode::Name(name),
        ExprNode::Impossible => ExprNode::Impossible,
        ExprNode::Invalid => ExprNode::Invalid,
    };

    let anno = new_ids.get(&expr.anno).copied().unwrap_or(expr.anno);

    Expr { node, anno }
}

fn fixup_pat(pat: mir::Pat, _new_ids: &HashMap<varless::TypeId, varless::TypeId>) -> mir::Pat {
    pat
}
