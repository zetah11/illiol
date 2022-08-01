mod assign;
mod bind;
mod check;
mod infer;
mod substitute;
mod tween;
mod types;

use std::collections::HashMap;

use crate::hir;
use crate::mir;
use crate::types as varless;

use types::{Type, TypeId, Types};

pub fn typeck(prog: hir::Decls) -> mir::Program {
    let mut checker = Checker::new();
    for (name, item) in prog.values.iter() {
        let ty = checker.make_type(&item.anno);
        checker.declare(name.clone(), ty);
    }

    let mut values = HashMap::with_capacity(prog.values.len());
    for (name, item) in prog.values {
        let ty = checker.make_type(&item.anno);
        let item = checker.check_expr(item.body, ty);
        values.insert(name, item);
    }

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

#[derive(Debug, Default)]
struct Checker {
    context: HashMap<mir::Name, TypeId>,
    types: Types,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
            types: Types::new(),
        }
    }

    pub fn declare(&mut self, name: mir::Name, ty: TypeId) {
        self.context.insert(name, ty);
    }

    pub fn ctx_and_types(mut self) -> (HashMap<mir::Name, varless::TypeId>, varless::Types) {
        let ctx: HashMap<_, _> = self.context.drain().collect();
        let ctx = ctx
            .into_iter()
            .map(|(name, ty)| (name, self.subst_typeid(ty)))
            .collect();

        let old_types = std::mem::take(&mut self.types);
        let mut types = varless::Types::new();

        for (id, ty) in old_types {
            types.add(varless::TypeId(id.0), self.subst_type(ty));
        }

        (ctx, types)
    }

    fn make_type(&mut self, ty: &hir::Type) -> TypeId {
        match ty {
            hir::Type::Bool => self.boolean_type(),
            hir::Type::Regex => self.regex_type(),
            hir::Type::Range(lo, hi) => self.types.add(Type::Range(*lo, *hi)),
            hir::Type::String(pat) => self.types.add(Type::String(pat.clone())),
            hir::Type::Arrow(from, into) => {
                let from = self.make_type(&*from);
                let into = self.make_type(&*into);
                self.types.add(Type::Arrow(from, into))
            }
            hir::Type::Invalid => self.error_type(),
        }
    }

    fn boolean_type(&mut self) -> TypeId {
        self.types.add(Type::Bool)
    }

    fn bottom_type(&mut self) -> TypeId {
        self.types.add(Type::Bottom)
    }

    fn error_type(&mut self) -> TypeId {
        self.types.add(Type::Error)
    }

    fn regex_type(&mut self) -> TypeId {
        self.types.add(Type::Regex)
    }
}
