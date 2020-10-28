use super::*;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::{
    ast_enum::Type,
    ast_struct::{
        Abi, BareFnArg, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeMacro, TypeParen,
        TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple, Variadic,
    },
};

ast_struct! {
    /// A path like `std::slice::Iter`, optionally qualified with a
    /// self-type as in `<Vec<T> as SomeTrait>::Associated`.
    pub struct TypePath {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) qself: Option<QSelf>,
        #[serde(flatten)]
        pub(crate) path: Path,
    }
}

ast_struct! {
    /// Return type of a function signature.
    #[derive(Default)]
    #[serde(transparent)]
    pub struct ReturnType {
        ty: Option<Box<Type>>,
    }
}

mod convert {
    use super::*;

    // ReturnType
    syn_trait_impl!(syn::ReturnType);
    impl From<&syn::ReturnType> for ReturnType {
        fn from(other: &syn::ReturnType) -> Self {
            use syn::ReturnType;
            match other {
                ReturnType::Default => Self { ty: None },
                ReturnType::Type(_, x) => Self { ty: Some(x.map_into()) },
            }
        }
    }
    impl From<&ReturnType> for syn::ReturnType {
        fn from(other: &ReturnType) -> Self {
            use syn::ReturnType;
            match &other.ty {
                None => ReturnType::Default,
                Some(x) => ReturnType::Type(default(), x.map_into()),
            }
        }
    }
}
