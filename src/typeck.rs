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

use log::debug;

use self::solve::Constraint;
use self::types::TypeVar;
use crate::hir;
use crate::mir;
use crate::types as varless;

use types::{TypeId, Types};

pub fn typeck(prog: hir::Decls) -> mir::Program {
    let mut checker = Checker::new();
    for (name, item) in prog.values.iter() {
        let ty = checker.lower_type(&item.anno);
        checker.declare(name.clone(), ty);
    }

    let mut values = HashMap::with_capacity(prog.values.len());
    for (name, item) in prog.values {
        let ty = checker.lower_type(&item.anno);
        let item = checker.check_expr(item.body, ty);
        values.insert(name, item);
    }

    checker.solve_constraints();

    let values = values
        .into_iter()
        .map(|(name, expr)| (name, checker.substitute(expr)))
        .collect();
    let (context, types) = checker.ctx_and_types();

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

    pub fn declare(&mut self, name: mir::Name, ty: TypeId) {
        self.context.insert(name, ty);
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

    pub fn ctx_and_types(mut self) -> (HashMap<mir::Name, varless::TypeId>, varless::Types) {
        debug!("Substituting type context");
        let ctx: HashMap<_, _> = self.context.drain().collect();
        let ctx = ctx
            .into_iter()
            .map(|(name, ty)| (name, self.subst_typeid(ty)))
            .collect();

        debug!("Substituting type definitions");
        let old_types = std::mem::take(&mut self.types);
        let mut types = varless::Types::new();

        for (id, ty) in old_types {
            types.add(varless::TypeId(id.0), self.subst_type(ty));
        }

        (ctx, types)
    }

    fn fresh_tyvar(&mut self) -> TypeVar {
        let v = self.curr_tyvar;
        self.curr_tyvar = TypeVar(self.curr_tyvar.0 + 1);
        v
    }
}
