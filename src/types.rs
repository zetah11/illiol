use std::collections::HashMap;

use crate::mir::Name;
use crate::Regex;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TypeId(pub(crate) usize);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Bottom,
    Bool,
    Regex,

    Range(i64, i64),
    String(Regex),

    Arrow(TypeId, TypeId),
    Named(Name),

    Error,
}

#[derive(Debug, Default)]
pub struct Types {
    types: HashMap<TypeId, Type>,
}

impl Types {
    pub(crate) fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, id: TypeId, ty: Type) {
        self.types.insert(id, ty);
    }

    pub fn get(&self, id: &TypeId) -> &Type {
        self.types.get(id).unwrap()
    }
}

impl FromIterator<(TypeId, Type)> for Types {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (TypeId, Type)>,
    {
        let mut types = Self::new();
        for (id, ty) in iter {
            types.add(id, ty)
        }
        types
    }
}
