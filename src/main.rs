use std::collections::HashMap;

use illiol::hir::{Decls, Expr, Literal, Pat, Type, ValueDef};
use illiol::typeck;

fn main() {
    env_logger::init();

    let x = Expr::Let {
        pat: Pat::Bind("x'".into()),
        bound: Box::new(Expr::Anno(
            Box::new(Expr::Lit(Literal::Integer(5))),
            Type::Range(0, 10),
        )),
        then: Box::new(Expr::Name("x'".into())),
        elze: Box::new(Expr::Impossible),
    };
    let x = ValueDef {
        anno: Type::Wildcard,
        body: x,
    };

    let prog = Decls {
        values: HashMap::from([("x".into(), x)]),
    };

    let checked = typeck(prog);

    println!("{checked:#?}");
}
