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
        Type::Arrow(
            Box::new(Type::Named("T".into())),
            Box::new(Type::Named("T".into())),
        ),
    );
    let f = ValueDef {
        anno: Type::Wildcard,
        body: f,
        vars: vec!["T".into()],
    };

    let x = Expr::Lit(Literal::Integer(5));
    let x = ValueDef {
        anno: Type::Range(0, 10),
        body: x,
        vars: vec![],
    };

    let y = Expr::Call(
        Box::new(Expr::Name("f".into())),
        Box::new(Expr::Name("x".into())),
    );
    let y = ValueDef {
        anno: Type::Wildcard,
        body: y,
        vars: vec![],
    };

    let prog = Decls {
        values: HashMap::from([("x".into(), x), ("y".into(), y), ("f".into(), f)]),
    };

    let checked = typeck(prog);

    println!("{checked:#?}");
}
