//! P02 Stage A semantic types. No LLVM or target host type dependency.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Type {
    Error,
    Unit,
    Int32,
    Bool,
    String,
    Function,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TypeId(usize);

#[derive(Debug, Default, Eq, PartialEq)]
pub struct TypeInterner {
    types: Vec<Type>,
}
impl TypeInterner {
    pub fn intern(&mut self, ty: Type) -> TypeId {
        if let Some(index) = self.types.iter().position(|existing| *existing == ty) {
            return TypeId(index);
        }
        let id = TypeId(self.types.len());
        self.types.push(ty);
        id
    }
    pub fn get(&self, id: TypeId) -> Option<Type> {
        self.types.get(id.0).copied()
    }
    /// ErrorType suppresses errors originating in an earlier check.
    pub fn compatible(&self, actual: TypeId, expected: TypeId) -> bool {
        match (self.get(actual), self.get(expected)) {
            (Some(Type::Error), Some(_)) | (Some(_), Some(Type::Error)) => true,
            (Some(left), Some(right)) => left == right,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable_interning_and_error_compatibility() {
        let mut types = TypeInterner::default();
        let int = types.intern(Type::Int32);
        let string = types.intern(Type::String);
        let error = types.intern(Type::Error);
        for _ in 0..100 {
            assert_eq!(types.intern(Type::Int32), int);
        }
        assert!(!types.compatible(int, string));
        assert!(types.compatible(error, string));
        assert!(types.compatible(int, error));
        assert_eq!(types.get(TypeId(100)), None);
        assert!(!types.compatible(TypeId(100), error));
    }
}
