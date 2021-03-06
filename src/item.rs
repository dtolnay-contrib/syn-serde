use super::*;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::{
    ast_enum::{FnArg, ForeignItem, ImplItem, Item, TraitItem, UseTree},
    ast_struct::{
        ForeignItemFn, ForeignItemMacro, ForeignItemStatic, ForeignItemType, ImplItemConst,
        ImplItemMacro, ImplItemMethod, ImplItemType, ItemConst, ItemEnum, ItemExternCrate, ItemFn,
        ItemForeignMod, ItemImpl, ItemMacro, ItemMacro2, ItemStatic, ItemTrait, ItemTraitAlias,
        ItemType, ItemUnion, ItemUse, Signature, TraitItemConst, TraitItemMacro, TraitItemType,
        UseGroup, UseName, UsePath, UseRename,
    },
};

ast_struct! {
    /// A module or module declaration: `mod m` or `mod m { ... }`.
    pub struct ItemMod {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
        // TODO: should not skip_serializing_if
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) content: Option<Vec<Item>>,
        // TODO: can remove
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) semi: bool,
    }
}

ast_struct! {
    /// A struct definition: `struct Foo<A> { x: A }`.
    pub struct ItemStruct {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub(crate) generics: Generics,
        pub(crate) fields: Fields,
        // #[serde(default, skip_serializing_if = "not")]
        // pub(crate) semi_token: bool,
    }
}

ast_struct! {
    /// A trait method within the definition of a trait.
    pub struct TraitItemMethod {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(flatten)]
        pub(crate) sig: Signature,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) default: Option<Block>,
        // #[serde(default, skip_serializing_if = "not")]
        // pub(crate) semi_token: bool,
    }
}

ast_struct! {
    /// The `self` argument of an associated method, whether taken by value
    /// or by reference.
    pub struct Receiver {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(rename = "ref")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) reference: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) lifetime: Option<Lifetime>,
        #[serde(rename = "mut")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) mutability: bool,
    }
}

mod convert {
    use super::*;

    // ItemStruct
    syn_trait_impl!(syn::ItemStruct);
    impl From<&syn::ItemStruct> for ItemStruct {
        fn from(other: &syn::ItemStruct) -> Self {
            let fields: Fields = other.fields.ref_into();
            assert_struct_semi(&fields, other.semi_token.is_some());

            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                fields,
            }
        }
    }
    impl From<&ItemStruct> for syn::ItemStruct {
        fn from(other: &ItemStruct) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                struct_token: default(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                fields: other.fields.ref_into(),
                semi_token: default_or_none(!other.fields.is_named()),
            }
        }
    }

    // TraitItemMethod
    syn_trait_impl!(syn::TraitItemMethod);
    impl From<&syn::TraitItemMethod> for TraitItemMethod {
        fn from(other: &syn::TraitItemMethod) -> Self {
            if other.default.is_some() {
                // `fn foo() -> bool {};`
                assert!(other.semi_token.is_none(), "unexpected token: `;`");
            } else {
                // `fn foo() -> bool`
                assert!(other.semi_token.is_some(), "expected `;`");
            }

            Self {
                attrs: other.attrs.map_into(),
                sig: other.sig.ref_into(),
                default: other.default.map_into(),
            }
        }
    }
    impl From<&TraitItemMethod> for syn::TraitItemMethod {
        fn from(other: &TraitItemMethod) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                sig: other.sig.ref_into(),
                default: other.default.map_into(),
                semi_token: default_or_none(other.default.is_none()),
            }
        }
    }

    // Receiver
    syn_trait_impl!(syn::Receiver);
    impl From<&syn::Receiver> for Receiver {
        fn from(node: &syn::Receiver) -> Self {
            Self {
                attrs: node.attrs.map_into(),
                reference: node.reference.is_some(),
                lifetime: node.reference.as_ref().and_then(|(_0, _1)| _1.map_into()),
                mutability: node.mutability.is_some(),
            }
        }
    }
    impl From<&Receiver> for syn::Receiver {
        fn from(node: &Receiver) -> Self {
            Self {
                attrs: node.attrs.map_into(),
                reference: if node.reference {
                    Some((default(), node.lifetime.map_into()))
                } else {
                    None
                },
                mutability: default_or_none(node.mutability),
                self_token: default(),
            }
        }
    }
}
