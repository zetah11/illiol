use std::collections::HashMap;

use crate::types::{TypeId, Types};

pub type Name = String;

#[derive(Debug)]
pub struct Program {
    pub context: HashMap<Name, Template>,
    pub decls: Decls,
    pub types: Types,
}

#[derive(Debug)]
pub struct Template {
    pub params: Vec<Name>,
    pub uninst: TypeId,
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

    Tuple(Vec<Expr>),

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

#[derive(Clone, Debug)]
pub enum Literal {
    Boolean(bool),
    Integer(i64),
    String(String),
    Regex(String),
}
