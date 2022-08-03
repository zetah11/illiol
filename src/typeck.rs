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
use log::{debug, trace};

use self::solve::Constraint;
use self::tween::Mutability;
use self::types::{Type, TypeVar};
use crate::hir;
use crate::mir;
use crate::types as varless;

pub fn typeck(prog: hir::Decls) -> mir::Program {
    debug!("Declaring");
    let mut checker = Checker::new();
    for (name, item) in prog.values.iter() {
        checker.declare(name.clone(), &item.anno);
    }

    trace!("Declared types {:?}", checker.context);

    debug!("Defining & solving");
    let mut values = HashMap::with_capacity(prog.values.len());
    for (name, item) in prog.values {
        let body = checker.define(&name, item.body);
        values.insert(name, body);
    }

    if !checker.solve_constraints() {
        trace!("Unsolved constraints {:?}", checker.worklist);
        trace!("Types {:?}", checker.context);
        panic!("unsolved constraints!");
    }

    debug!("Substituting & memoizing");
    let context = checker.subst_ctx();
    let values = values
        .into_iter()
        .map(|(name, expr)| {
            let expr = checker.substitute(expr);
            (name, expr)
        })
        .collect();
    let types = checker.lower.into_iter().collect();

    mir::Program {
        context,
        types,
        decls: mir::Decls { values },
    }
}

#[derive(Debug)]
struct Checker {
    context: HashMap<mir::Name, Type>,
    subst: HashMap<TypeVar, Type>,

    lower: BiMap<varless::TypeId, varless::Type>,

    curr_tyvar: TypeVar,
    worklist: Vec<Constraint>,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
            subst: HashMap::new(),

            lower: BiMap::new(),

            curr_tyvar: TypeVar(0),
            worklist: Vec::new(),
        }
    }

    pub fn declare(&mut self, name: mir::Name, ty: &hir::Type) {
        let ty = self.lower_type(ty, Mutability::Immutable);
        self.context.insert(name, ty);
    }

    pub fn define(&mut self, name: &mir::Name, expr: hir::Expr) -> tween::Expr {
        let ty = self.context.get(name).unwrap();

        let ty = ty.clone().make_mutable();
        let item = self.check_expr(expr, ty);
        self.solve_constraints(); // solve while vars are still mut

        item
    }

    /// Solve as many constraints as possible. Returns `false` if this was
    /// unsuccessful - i.e. there are unsolved constraints.
    pub fn solve_constraints(&mut self) -> bool {
        loop {
            let worklist: Vec<_> = self.worklist.drain(..).collect();
            let prev = worklist.len();

            if prev == 0 {
                trace!("No more constraints");
                return true;
            }

            trace!("Solve loop; {} constraints to solve", prev);

            for ctr in worklist {
                self.solve(ctr);
            }

            if self.worklist.len() >= prev {
                return false;
            }
        }
    }

    pub fn subst_ctx(&mut self) -> HashMap<mir::Name, varless::TypeId> {
        debug!("Substituting type context");
        let ctx: HashMap<_, _> = self.context.drain().collect();
        ctx.into_iter()
            .map(|(name, ty)| {
                let ty = self.subst_type(ty);
                (name, ty)
            })
            .collect()
    }

    fn fresh_tyvar(&mut self) -> TypeVar {
        let v = self.curr_tyvar;
        self.curr_tyvar = TypeVar(self.curr_tyvar.0 + 1);
        v
    }
}
