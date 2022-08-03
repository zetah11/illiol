use std::collections::HashMap;

use illiol::hir::{Decls, Expr, Literal, Pat, Type, ValueDef};
use illiol::typeck;

fn main() {
    env_logger::init();

    let f = Expr::Anno(
        Box::new(Expr::Fun(
            Pat::Bind("a".into()),
            Box::new(Expr::Name("a".into())),
        )),
        Type::Arrow(Box::new(Type::Range(0, 10)), Box::new(Type::Range(0, 10))),
    );
    let f = ValueDef {
        anno: Type::Wildcard,
        body: f,
    };

    let x = Expr::Call(
        Box::new(Expr::Name("f".into())),
        Box::new(Expr::Lit(Literal::Integer(5))),
    );
    let x = ValueDef {
        anno: Type::Wildcard,
        body: x,
    };

    let prog = Decls {
        values: HashMap::from([("x".into(), x), ("f".into(), f)]),
    };

    let checked = typeck(prog);

    println!("{checked:#?}");
}
