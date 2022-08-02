use bimap::BiMap;

use crate::Regex;

use super::tween::Mutability;

#[derive(Debug, Default)]
pub struct Types {
    types: BiMap<TypeId, Type>,
}

impl Types {
    pub fn new() -> Self {
        Self {
            types: BiMap::new(),
        }
    }

    /// Add a type to the structure and get back its id. This function memoizes
    /// its input, such that giving it the same type always returns the same id.
    pub fn add(&mut self, ty: Type) -> TypeId {
        if let Some(id) = self.types.get_by_right(&ty) {
            *id
        } else {
            let id = TypeId(self.types.len());
            self.types.insert(id, ty);
            id
        }
    }

    pub fn get(&self, id: &TypeId) -> &Type {
        self.types.get_by_left(id).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &Type)> {
        self.types.iter()
    }

    /// Turn all immutable type variables into mutable ones for the given type.
    pub fn make_mutable(&mut self, id: &TypeId) {
        self.make_mutability(id, Mutability::Mutable)
    }

    /// Turn all mutable type variables into immutable ones for the given type.
    pub fn make_immutable(&mut self, id: &TypeId) {
        self.make_mutability(id, Mutability::Immutable)
    }

    fn make_mutability(&mut self, id: &TypeId, mutability: Mutability) {
        match self.get(id) {
            Type::Var(_, v) => {
                let v = *v;
                self.types.insert(*id, Type::Var(mutability, v));
            }

            Type::Arrow(t, u) => {
                let (t, u) = (*t, *u);
                self.make_mutability(&t, mutability);
                self.make_mutability(&u, mutability);
            }

            Type::Bottom
            | Type::Bool
            | Type::Regex
            | Type::Range(..)
            | Type::String(..)
            | Type::Error => (),
        }
    }
}

impl IntoIterator for Types {
    type IntoIter = bimap::hash::IntoIter<TypeId, Type>;
    type Item = (TypeId, Type);

    fn into_iter(self) -> Self::IntoIter {
        self.types.into_iter()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TypeId(pub(super) usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TypeVar(pub(super) usize);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Bottom,
    Bool,
    Regex,

    Range(i64, i64),
    String(Regex),

    Arrow(TypeId, TypeId),

    Var(Mutability, TypeVar),

    Error,
}

#[cfg(test)]
mod tests {
    use super::{Type, Types};

    #[test]
    fn types_is_injective() {
        let mut types = Types::new();
        let a = types.add(Type::Bool);
        let b = types.add(Type::Bool);
        let c = types.add(Type::Range(-5, 10));
        let d = types.add(Type::Range(-5, 10));

        let x = types.add(Type::Arrow(a, c));
        let y = types.add(Type::Arrow(b, d));

        assert_eq!(x, y);
        assert_ne!(x, d);
    }
}
