use std::collections::HashMap;

use illiol::hir::{Decls, Expr, Literal, Pat, Type, ValueDef};
use illiol::typeck;

fn main() {
    env_logger::init();

    let x = Expr::Lit(Literal::Integer(5));
    let x = ValueDef {
        anno: Type::Range(0, 10),
        body: x,
    };

    let y = Expr::Name("x".into());
    let y = ValueDef {
        anno: Type::Range(0, 10),
        body: y,
    };

    let z = Expr::Let {
        pat: Pat::Bind("z'".into()),
        bound: Box::new(Expr::Name("y".into())),
        then: Box::new(Expr::Name("z'".into())),
        elze: Box::new(Expr::Impossible),
    };
    let z = ValueDef {
        anno: Type::Wildcard,
        body: z,
    };

    let prog = Decls {
        values: HashMap::from([("z'".into(), z), ("x".into(), x), ("y".into(), y)]),
    };

    let checked = typeck(prog);

    println!("{checked:#?}");
}
