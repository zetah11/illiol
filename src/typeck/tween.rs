pub use crate::mir::{Literal, Name};

use std::collections::HashMap;

use super::types::Type;

#[derive(Clone, Debug)]
pub struct Decls {
    pub values: HashMap<Name, Expr>,
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub node: ExprNode,
    pub anno: Type,
}

#[derive(Clone, Debug)]
pub enum ExprNode {
    Fun(Pat, Box<Expr>),

    Let {
        pat: Pat,
        bound: Box<Expr>,
        then: Box<Expr>,
        elze: Box<Expr>,
    },

    Call(Box<Expr>, Box<Expr>),

    Lit(Literal),

    Name(Name),
    Instantiated(Name),

    Impossible,
    Invalid,
}

#[derive(Clone, Debug)]
pub enum Pat {
    Constructor(Name),
    Bind(Name),
    Apply(Box<Pat>, Box<Pat>),
    Lit(Literal),
    Wildcard,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Mutability {
    Immutable,
    Mutable,
}
