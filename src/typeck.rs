mod assign;
mod bind;
mod check;
mod infer;

use std::collections::HashMap;

use crate::hir;
use crate::mir;
use crate::types::{Type, TypeId, Types};

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

    mir::Program {
        context: checker.context,
        decls: mir::Decls { values },
        types: checker.types,
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
