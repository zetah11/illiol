use super::types::{Type, TypeId};
use super::Checker;

impl Checker {
    pub fn check_assignable(&mut self, into: TypeId, from: TypeId) {
        match (self.types.get(&into), self.types.get(&from)) {
            (_, Type::Bottom) => (),
            (Type::Bool, Type::Bool) => (),
            (Type::Regex, Type::Regex) => (),
            (Type::Range(lo1, hi1), Type::Range(lo2, hi2)) => {
                assert_eq!(lo1, lo2);
                assert_eq!(hi1, hi2);
            }
            (Type::String(pat1), Type::String(pat2)) => {
                assert_eq!(pat1, pat2);
            }
            (Type::Error, _) | (_, Type::Error) => (),
            _ => panic!("inequal types"),
        }
    }

    pub fn as_fun_ty(&mut self, ty: TypeId) -> (TypeId, TypeId) {
        match self.types.get(&ty) {
            Type::Arrow(from, into) => (*from, *into),
            _ => panic!("not a function type"),
        }
    }

    pub fn check_bool(&mut self, ty: TypeId) {
        let bool = self.boolean_type();
        self.check_assignable(bool, ty);
    }

    pub fn check_int(&mut self, ty: TypeId, val: i64) {
        match self.types.get(&ty) {
            Type::Range(lo, hi) => {
                assert!(lo <= &val && &val < hi);
            }
            _ => panic!("not an integer type"),
        }
    }

    pub fn check_regex(&mut self, ty: TypeId) {
        let regex = self.regex_type();
        self.check_assignable(regex, ty);
    }

    pub fn check_str(&mut self, ty: TypeId, val: &str) {
        match self.types.get(&ty) {
            Type::String(pat) => assert!(pat.is_match(val)),
            _ => panic!("not a string type"),
        }
    }
}
