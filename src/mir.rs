use std::collections::HashMap;

use crate::types::{TypeId, Types};

pub type Name = String;

#[derive(Debug)]
pub struct Program {
    pub context: HashMap<Name, TypeId>,
    pub decls: Decls,
    pub types: Types,
}

#[derive(Clone, Debug)]
pub struct Decls {
    pub values: HashMap<Name, Expr>,
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub node: ExprNode,
    pub anno: TypeId,
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

    Op(Operator, Vec<Expr>),

    Cast(Box<Expr>),

    Lit(Literal),

    Name(Name),

    Impossible,
    Invalid,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,

    And,
    AndDo,
    Or,
    OrDo,
    Xor,

    Not,

    In,
}

#[derive(Clone, Debug)]
pub enum Pat {
    Constructor(Name),
    Bind(Name),
    Apply(Box<Pat>, Vec<Pat>),
    Lit(Literal),
    Wildcard,
}

#[derive(Clone, Debug)]
pub enum Literal {
    Boolean(bool),
    Integer(i64),
    String(String),
    Regex(String),
}
