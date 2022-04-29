//! Structs to receive attribute options for each supported syntax type.
//!
//! Dedicated structs are used per-syntax-type to make sure unexpected meta properties produce errors
//! at compile-time.

use std::{convert::TryFrom, ops::Deref};

use darling::{
    util::{Flag, Override, SpannedValue},
    Error, FromMeta,
};
use syn::{Ident, LitStr};
use wasm_bindgen_backend::ast;

use crate::{meta, rhs::Rhs};

use self::fields::SkipTypescript;

/// A JS namespace, which can be represented as `"value"` or `r#"["first", "second"]#"`.
pub struct JsNamespace(Vec<syn::LitStr>);

impl JsNamespace {
    pub fn to_strings(&self) -> Vec<String> {
        self.0.iter().map(|s| s.value()).collect()
    }
}

impl FromMeta for JsNamespace {
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        if let syn::Lit::Str(s) = value {
            if let Ok(array) = s.parse::<syn::ExprArray>() {
                let mut errors = Error::accumulator();
                let terms = array
                    .elems
                    .into_iter()
                    .filter_map(|elem| errors.handle(get_string_literal(elem)))
                    .collect();
                errors.finish_with(Self(terms))
            } else if let Ok(_ident) = s.parse::<syn::Ident>() {
                Ok(Self(vec![s.clone()]))
            } else {
                Err(Error::custom(format!(
                    "Value is not an ident or a list of idents"
                )))
            }
        } else {
            Err(Error::unexpected_lit_type(value))
        }
    }
}

fn get_string_literal(expr: syn::Expr) -> darling::Result<LitStr> {
    if let syn::Expr::Lit(syn::ExprLit {
        attrs: _,
        lit: syn::Lit::Str(s),
    }) = expr
    {
        Ok(s)
    } else {
        Err(darling::Error::custom("Expected string literal").with_span(&expr))
    }
}

pub mod fields {
    use darling::util::Flag;

    macro_rules! field_trait {
        ($name:ident, $return:ty, $field:ident) => {
            pub trait $name {
                fn $field(&self) -> $return;
            }
        };
    }

    field_trait!(JsName, Option<&syn::LitStr>, js_name);
    field_trait!(SkipTypescript, Flag, skip_typescript);
}

macro_rules! impl_field_trait {
    (JsName for $name:ident) => {
        impl fields::JsName for $name {
            fn js_name(&self) -> Option<&LitStr> {
                self.js_name.as_ref()
            }
        }
    };
    (SkipTypescript for $name:ident) => {
        impl fields::SkipTypescript for $name {
            fn skip_typescript(&self) -> Flag {
                self.skip_typescript
            }
        }
    };
}

pub trait ParseAttr {
    type Parsed: FromMeta;

    /// Parse the `wasm_bindgen` attr from the explicitly-passed list of meta items.
    fn parse_attr_args(args: &[syn::NestedMeta]) -> darling::Result<Self::Parsed>;

    /// Find the first `#[wasm_bindgen]` attribute in `self`, remove it, and parse it.
    fn parse_contained_attr(&mut self) -> darling::Result<Self::Parsed>;
}

macro_rules! impl_parse_attr {
    ($syn:path, $parsed:ident) => {
        impl ParseAttr for $syn {
            type Parsed = $parsed;

            fn parse_attr_args(args: &[syn::NestedMeta]) -> darling::Result<Self::Parsed> {
                Self::Parsed::from_list(args)
            }

            fn parse_contained_attr(&mut self) -> darling::Result<Self::Parsed> {
                extract_wasm_bindgen::<Self::Parsed>(&mut self.attrs)
            }
        }
    };
}

fn extract_wasm_bindgen<T: FromMeta>(attrs: &mut Vec<syn::Attribute>) -> darling::Result<T> {
    let wasm_bindgen_attr = if let Some((index, _attr)) = attrs
        .iter()
        .enumerate()
        .find(|(_, val)| val.path.is_ident("wasm_bindgen"))
    {
        Some(meta::Meta::<Rhs>::try_from(attrs.remove(index))?.into())
    } else {
        None
    };

    match wasm_bindgen_attr {
        Some(syn::Meta::Path(_)) | None => T::from_list(&[]),
        Some(meta) => T::from_meta(&meta),
    }
}

#[derive(FromMeta)]
pub struct ImplItemMethod {
    pub constructor: Flag,
    pub getter: Option<SpannedValue<Override<Ident>>>,
    pub indexing_deleter: Flag,
    pub indexing_getter: Flag,
    pub indexing_setter: Flag,
    pub js_name: Option<LitStr>,
    pub setter: Option<SpannedValue<Override<Ident>>>,
    pub skip: Flag,
    pub skip_typescript: Flag,
}

impl_parse_attr!(syn::ImplItemMethod, ImplItemMethod);
impl_field_trait!(JsName for ImplItemMethod);
impl_field_trait!(SkipTypescript for ImplItemMethod);

#[derive(FromMeta)]
pub struct ItemEnum {
    pub js_name: Option<LitStr>,
    pub skip_typescript: Flag,
}

impl_parse_attr!(syn::ItemEnum, ItemEnum);

#[derive(Debug, FromMeta)]
pub struct ItemStruct {
    pub js_name: Option<LitStr>,
    pub getter_with_clone: Flag,
    pub inspectable: Flag,
    pub skip_typescript: Flag,
}

impl_parse_attr!(syn::ItemStruct, ItemStruct);
impl_field_trait!(JsName for ItemStruct);
impl_field_trait!(SkipTypescript for ItemStruct);

#[derive(FromMeta)]
pub struct ItemFn {
    pub getter: Option<SpannedValue<Override<Ident>>>,
    pub indexing_deleter: Flag,
    pub indexing_getter: Flag,
    pub indexing_setter: Flag,
    pub js_name: Option<LitStr>,
    pub setter: Option<SpannedValue<Override<Ident>>>,
    pub skip: Flag,
    pub skip_typescript: Flag,
    pub start: Flag,
}

impl_parse_attr!(syn::ItemFn, ItemFn);
impl_field_trait!(JsName for ItemFn);
impl_field_trait!(SkipTypescript for ItemFn);

#[derive(FromMeta)]
pub struct ItemImpl {
    pub js_class: Option<LitStr>,
}

impl_parse_attr!(syn::ItemImpl, ItemImpl);

#[derive(FromMeta)]
pub struct Field {
    pub getter_with_clone: Flag,
    pub js_name: Option<LitStr>,
    pub readonly: Flag,
    pub skip: Flag,
    pub skip_typescript: Flag,
}

impl_parse_attr!(syn::Field, Field);

#[derive(FromMeta)]
pub struct Variant {
    pub js_name: Option<LitStr>,
    pub skip: Flag,
}

impl_parse_attr!(syn::Variant, Variant);

#[derive(FromMeta)]
pub struct ItemConst {
    // This requires the word be present, and no value be provided. If this becomes optional
    // in the future, switch to `Flag` instead.
    pub typescript_custom_section: SpannedValue<()>,
}

impl_parse_attr!(syn::ItemConst, ItemConst);

#[derive(FromMeta)]
#[darling(and_then = "Self::finish")]
pub struct ForeignMod {
    pub module: Option<LitStr>,
    pub raw_module: Option<LitStr>,
    pub inline_js: Option<LitStr>,
    /// This field prevents construction of a `ForeignMod` without it going through the validation
    /// in [`Self::finish`].
    #[darling(skip)]
    __do_not_construct: (),
}

impl_parse_attr!(syn::ItemForeignMod, ForeignMod);

impl ForeignMod {
    fn finish(self) -> darling::Result<Self> {
        let mut errors = Error::accumulator();
        if let Some(v) = &self.module {
            if self.inline_js.is_some() {
                let msg = "cannot specify both `module` and `inline_js`";
                errors.push(Error::custom(msg).with_span(v));
            }
            if self.raw_module.is_some() {
                let msg = "cannot specify both `module` and `raw_module`";
                errors.push(Error::custom(msg).with_span(v));
            }
        } else if let Some(v) = &self.raw_module {
            if self.inline_js.is_some() {
                let msg = "cannot specify both `raw_module` and `inline_js`";
                errors.push(Error::custom(msg).with_span(v));
            }
        }

        errors.finish_with(self)
    }
}

#[derive(FromMeta)]
pub struct ForeignItemFn {
    pub assert_no_shim: Flag,
    pub catch: Flag,
    pub constructor: Flag,
    #[darling(rename = "final")]
    pub r#final: Flag,
    pub getter: Option<SpannedValue<Override<Ident>>>,
    pub indexing_deleter: Flag,
    pub indexing_getter: Flag,
    pub indexing_setter: Flag,
    pub js_class: Option<LitStr>,
    pub js_name: Option<LitStr>,
    pub js_namespace: Option<JsNamespace>,
    pub method: Flag,
    pub structural: Flag,
    pub variadic: Flag,
    pub setter: Option<SpannedValue<Override<Ident>>>,
    pub static_method_of: Option<Ident>,
}

impl_parse_attr!(syn::ForeignItemFn, ForeignItemFn);
impl_field_trait!(JsName for ForeignItemFn);

impl SkipTypescript for ForeignItemFn {
    fn skip_typescript(&self) -> Flag {
        Flag::default()
    }
}

#[derive(FromMeta)]
pub struct ForeignItemStatic {
    pub js_name: Option<LitStr>,
    pub js_namespace: Option<JsNamespace>,
}

impl_parse_attr!(syn::ForeignItemStatic, ForeignItemStatic);

#[derive(FromMeta)]
pub struct ForeignItemType {
    #[darling(multiple)]
    pub extends: Vec<syn::Path>,
    pub is_type_of: Option<syn::Expr>,
    pub js_name: Option<LitStr>,
    pub js_namespace: Option<JsNamespace>,
    pub no_deref: Flag,
    pub typescript_type: Option<LitStr>,
    #[darling(multiple)]
    pub vendor_prefix: Vec<Ident>,
}

impl_parse_attr!(syn::ForeignItemType, ForeignItemType);

impl<'a> From<&'a ImplItemMethod> for ast::OperationKind {
    fn from(opts: &'a ImplItemMethod) -> Self {
        let mut operation_kind = ast::OperationKind::Regular;
        if let Some(g) = &opts.getter {
            operation_kind = ast::OperationKind::Getter(g.deref().clone().explicit());
        }
        if let Some(s) = &opts.setter {
            operation_kind = ast::OperationKind::Setter(s.deref().clone().explicit());
        }
        if opts.indexing_getter.is_present() {
            operation_kind = ast::OperationKind::IndexingGetter;
        }
        if opts.indexing_setter.is_present() {
            operation_kind = ast::OperationKind::IndexingSetter;
        }
        if opts.indexing_deleter.is_present() {
            operation_kind = ast::OperationKind::IndexingDeleter;
        }

        operation_kind
    }
}

impl<'a> From<&'a ItemFn> for ast::OperationKind {
    fn from(opts: &'a ItemFn) -> Self {
        let mut operation_kind = ast::OperationKind::Regular;
        if let Some(g) = &opts.getter {
            operation_kind = ast::OperationKind::Getter(g.deref().clone().explicit());
        }
        if let Some(s) = &opts.setter {
            operation_kind = ast::OperationKind::Setter(s.deref().clone().explicit());
        }
        if opts.indexing_getter.is_present() {
            operation_kind = ast::OperationKind::IndexingGetter;
        }
        if opts.indexing_setter.is_present() {
            operation_kind = ast::OperationKind::IndexingSetter;
        }
        if opts.indexing_deleter.is_present() {
            operation_kind = ast::OperationKind::IndexingDeleter;
        }

        operation_kind
    }
}

impl<'a> From<&'a ForeignItemFn> for ast::OperationKind {
    fn from(opts: &'a ForeignItemFn) -> Self {
        let mut operation_kind = ast::OperationKind::Regular;
        if let Some(g) = &opts.getter {
            operation_kind = ast::OperationKind::Getter(g.deref().clone().explicit());
        }
        if let Some(s) = &opts.setter {
            operation_kind = ast::OperationKind::Setter(s.deref().clone().explicit());
        }
        if opts.indexing_getter.is_present() {
            operation_kind = ast::OperationKind::IndexingGetter;
        }
        if opts.indexing_setter.is_present() {
            operation_kind = ast::OperationKind::IndexingSetter;
        }
        if opts.indexing_deleter.is_present() {
            operation_kind = ast::OperationKind::IndexingDeleter;
        }

        operation_kind
    }
}
