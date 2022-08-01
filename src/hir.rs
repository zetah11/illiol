use std::collections::HashMap;

use crate::Regex;

pub type Name = String;

#[derive(Clone, Debug)]
pub struct Decls {
    pub values: HashMap<Name, ValueDef>,
}

#[derive(Clone, Debug)]
pub struct ValueDef {
    pub anno: Type,
    pub body: Expr,
}

#[derive(Clone, Debug)]
pub enum Type {
    Bool,
    Regex,

    Range(i64, i64),
    String(Regex),

    Arrow(Box<Type>, Box<Type>),

    Invalid,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Anno(Box<Expr>, Type),

    Fun(Pat, Box<Expr>),

    Let {
        pat: Pat,
        bound: Box<Expr>,
        then: Box<Expr>,
        elze: Box<Expr>,
    },

    Call(Box<Expr>, Box<Expr>),

    Op(Operator, Vec<Expr>),

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
