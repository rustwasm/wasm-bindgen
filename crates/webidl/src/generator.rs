use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;
use syn::{Ident, Type};
use wasm_bindgen_backend::util::{raw_ident, rust_ident};

use crate::constants::{BUILTIN_IDENTS, POLYFILL_INTERFACES};
use crate::traverse::TraverseType;
use crate::util::{get_cfg_features, mdn_doc, required_doc_string, snake_case_ident};
use crate::Options;

fn add_features(features: &mut BTreeSet<String>, ty: &impl TraverseType) {
    ty.traverse_type(&mut |ident| {
        let ident = ident.to_string();

        if !BUILTIN_IDENTS.contains(ident.as_str()) {
            features.insert(ident);
        }
    });
}

fn get_features_doc(options: &Options, name: String) -> Option<String> {
    let mut features = BTreeSet::new();
    features.insert(name);
    required_doc_string(options, &features)
}

fn comment(mut comment: String, features: &Option<String>) -> TokenStream {
    if let Some(s) = features {
        comment.push_str(s);
    }

    let lines = comment.lines().map(|doc| quote!( #[doc = #doc] ));

    quote! {
        #(#lines)*
    }
}

fn maybe_unstable_attr(unstable: bool) -> Option<proc_macro2::TokenStream> {
    if unstable {
        Some(quote! {
            #[cfg(web_sys_unstable_apis)]
        })
    } else {
        None
    }
}

fn maybe_unstable_docs(unstable: bool) -> Option<proc_macro2::TokenStream> {
    if unstable {
        Some(quote! {
            #[doc = ""]
            #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
            #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
        })
    } else {
        None
    }
}

fn generate_arguments(arguments: &[(Ident, Type)], variadic: bool) -> Vec<TokenStream> {
    arguments
        .into_iter()
        .enumerate()
        .map(|(i, (name, ty))| {
            if variadic && i + 1 == arguments.len() {
                quote!( #name: &::js_sys::Array )
            } else {
                quote!( #name: #ty )
            }
        })
        .collect::<Vec<_>>()
}

fn generate_variadic(variadic: bool) -> Option<TokenStream> {
    if variadic {
        Some(quote!(variadic,))
    } else {
        None
    }
}

pub struct EnumVariant {
    pub name: Ident,
    pub value: String,
}

impl EnumVariant {
    fn generate(&self) -> TokenStream {
        let EnumVariant { name, value } = self;

        quote!( #name = #value )
    }
}

pub struct Enum {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
    pub unstable: bool,
}

impl Enum {
    pub fn generate(&self, options: &Options) -> TokenStream {
        let Enum {
            name,
            variants,
            unstable,
        } = self;

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);

        let doc_comment = comment(
            format!("The `{}` enum.", name),
            &get_features_doc(options, name.to_string()),
        );

        let variants = variants
            .into_iter()
            .map(|variant| variant.generate())
            .collect::<Vec<_>>();

        quote! {
            #![allow(unused_imports)]
            use wasm_bindgen::prelude::*;

            #unstable_attr
            #[wasm_bindgen]
            #doc_comment
            #unstable_docs
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum #name {
                #(#variants),*
            }
        }
    }
}

pub enum InterfaceAttributeKind {
    Getter,
    Setter,
}

pub struct InterfaceAttribute {
    pub js_name: String,
    pub ty: Type,
    pub is_static: bool,
    pub structural: bool,
    pub catch: bool,
    pub kind: InterfaceAttributeKind,
    pub unstable: bool,
}

impl InterfaceAttribute {
    fn generate(
        &self,
        options: &Options,
        parent_name: &Ident,
        parent_js_name: &str,
        parents: &[Ident],
    ) -> TokenStream {
        let InterfaceAttribute {
            js_name,
            ty,
            is_static,
            structural,
            catch,
            kind,
            unstable,
        } = self;

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);

        let mdn_docs = mdn_doc(parent_js_name, Some(js_name));

        let mut features = BTreeSet::new();

        add_features(&mut features, ty);

        for parent in parents {
            features.remove(&parent.to_string());
        }

        features.remove(&parent_name.to_string());

        let cfg_features = get_cfg_features(options, &features);

        features.insert(parent_name.to_string());

        let doc_comment = required_doc_string(options, &features);

        let structural = if *structural {
            quote!(structural,)
        } else {
            quote!(final,)
        };

        let (method, this) = if *is_static {
            (quote!( static_method_of = #parent_name, ), None)
        } else {
            (quote!(method,), Some(quote!( this: &#parent_name, )))
        };

        let (prefix, attr, def) = match kind {
            InterfaceAttributeKind::Getter => {
                let name = rust_ident(&snake_case_ident(js_name));

                let ty = if *catch {
                    quote!( Result<#ty, JsValue> )
                } else {
                    quote!( #ty )
                };

                (
                    "Getter",
                    quote!(getter,),
                    quote!( pub fn #name(#this) -> #ty; ),
                )
            }

            InterfaceAttributeKind::Setter => {
                let name = rust_ident(&format!("set_{}", snake_case_ident(js_name)));

                let ret_ty = if *catch {
                    Some(quote!( -> Result<(), JsValue> ))
                } else {
                    None
                };

                (
                    "Setter",
                    quote!(setter,),
                    quote!( pub fn #name(#this value: #ty) #ret_ty; ),
                )
            }
        };

        let catch = if *catch { Some(quote!(catch,)) } else { None };

        let doc_comment = comment(
            format!(
                "{} for the `{}` field of this object.\n\n{}",
                prefix, js_name, mdn_docs
            ),
            &doc_comment,
        );

        let js_name = raw_ident(js_name);

        quote! {
            #unstable_attr
            #cfg_features
            #[wasm_bindgen(
                #structural
                #catch
                #method
                #attr
                js_class = #parent_js_name,
                js_name = #js_name
            )]
            #doc_comment
            #unstable_docs
            #def
        }
    }
}

#[derive(Debug, Clone)]
pub enum InterfaceMethodKind {
    Constructor(Option<String>),
    Regular,
    IndexingGetter,
    IndexingSetter,
    IndexingDeleter,
}

pub struct InterfaceMethod {
    pub name: Ident,
    pub js_name: String,
    pub arguments: Vec<(Ident, Type)>,
    pub ret_ty: Option<Type>,
    pub kind: InterfaceMethodKind,
    pub is_static: bool,
    pub structural: bool,
    pub catch: bool,
    pub variadic: bool,
    pub unstable: bool,
}

impl InterfaceMethod {
    fn generate(
        &self,
        options: &Options,
        parent_name: &Ident,
        parent_js_name: String,
        parents: &[Ident],
    ) -> TokenStream {
        let InterfaceMethod {
            name,
            js_name,
            arguments,
            ret_ty,
            kind,
            is_static,
            structural,
            catch,
            variadic,
            unstable,
        } = self;

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);

        let mut is_constructor = false;

        let mut extra_args = vec![quote!( js_class = #parent_js_name )];

        let doc_comment = match kind {
            InterfaceMethodKind::Constructor(name) => {
                is_constructor = true;
                if let Some(name) = name {
                    extra_args[0] = quote!( js_class = #name );
                }
                format!(
                    "The `new {}(..)` constructor, creating a new \
                     instance of `{0}`.\n\n{}",
                    parent_name,
                    mdn_doc(&parent_js_name, Some(&parent_js_name))
                )
            }
            InterfaceMethodKind::Regular => {
                {
                    let js_name = raw_ident(js_name);
                    extra_args.push(quote!( js_name = #js_name ));
                }
                format!(
                    "The `{}()` method.\n\n{}",
                    js_name,
                    mdn_doc(&parent_js_name, Some(js_name))
                )
            }
            InterfaceMethodKind::IndexingGetter => {
                extra_args.push(quote!(indexing_getter));
                format!("Indexing getter.\n\n")
            }
            InterfaceMethodKind::IndexingSetter => {
                extra_args.push(quote!(indexing_setter));
                format!("Indexing setter.\n\n")
            }
            InterfaceMethodKind::IndexingDeleter => {
                extra_args.push(quote!(indexing_deleter));
                format!("Indexing deleter.\n\n")
            }
        };

        let mut features = BTreeSet::new();

        for (_, ty) in arguments.iter() {
            add_features(&mut features, ty);
        }

        if let Some(ty) = ret_ty {
            add_features(&mut features, ty);
        }

        for parent in parents {
            features.remove(&parent.to_string());
        }

        features.remove(&parent_name.to_string());

        let cfg_features = get_cfg_features(options, &features);

        features.insert(parent_name.to_string());

        let doc_comment = comment(doc_comment, &required_doc_string(options, &features));

        let ret = ret_ty.as_ref().map(|ret| quote!( #ret ));

        let ret = if *catch {
            let ret = ret.unwrap_or_else(|| quote!(()));
            Some(quote!( Result<#ret, JsValue> ))
        } else {
            ret
        };

        let ret = ret.as_ref().map(|ret| quote!( -> #ret ));

        let catch = if *catch { Some(quote!(catch,)) } else { None };

        let (method, this) = if is_constructor {
            assert!(!is_static);

            (quote!(constructor,), None)
        } else if *is_static {
            (quote!( static_method_of = #parent_name, ), None)
        } else {
            let structural = if *structural {
                quote!(structural)
            } else {
                quote!(final)
            };

            (
                quote!( method, #structural, ),
                Some(quote!( this: &#parent_name, )),
            )
        };

        let arguments = generate_arguments(arguments, *variadic);
        let variadic = generate_variadic(*variadic);

        quote! {
            #unstable_attr
            #cfg_features
            #[wasm_bindgen(
                #catch
                #method
                #variadic
                #(#extra_args),*
            )]
            #doc_comment
            #unstable_docs
            pub fn #name(#this #(#arguments),*) #ret;
        }
    }
}

pub enum InterfaceConstValue {
    BooleanLiteral(bool),
    FloatLiteral(f64),
    SignedIntegerLiteral(i64),
    UnsignedIntegerLiteral(u64),
}

impl InterfaceConstValue {
    fn generate(&self) -> TokenStream {
        use InterfaceConstValue::*;

        match self {
            BooleanLiteral(false) => quote!(false),
            BooleanLiteral(true) => quote!(true),
            // the actual type is unknown because of typedefs
            // so we cannot use std::fxx::INFINITY
            // but we can use type inference
            FloatLiteral(f) if f.is_infinite() && f.is_sign_positive() => quote!(1.0 / 0.0),
            FloatLiteral(f) if f.is_infinite() && f.is_sign_negative() => quote!(-1.0 / 0.0),
            FloatLiteral(f) if f.is_nan() => quote!(0.0 / 0.0),
            // again no suffix
            // panics on +-inf, nan
            FloatLiteral(f) => {
                let f = Literal::f64_suffixed(*f);
                quote!(#f)
            }
            SignedIntegerLiteral(i) => {
                let i = Literal::i64_suffixed(*i);
                quote!(#i)
            }
            UnsignedIntegerLiteral(i) => {
                let i = Literal::u64_suffixed(*i);
                quote!(#i)
            }
        }
    }
}

pub struct InterfaceConst {
    pub name: Ident,
    pub js_name: String,
    pub ty: syn::Type,
    pub value: InterfaceConstValue,
    pub unstable: bool,
}

impl InterfaceConst {
    fn generate(
        &self,
        options: &Options,
        parent_name: &Ident,
        parent_js_name: &str,
    ) -> TokenStream {
        let name = &self.name;
        let ty = &self.ty;
        let js_name = &self.js_name;
        let value = self.value.generate();
        let unstable = self.unstable;

        let unstable_attr = maybe_unstable_attr(unstable);
        let unstable_docs = maybe_unstable_docs(unstable);

        let doc_comment = comment(
            format!("The `{}.{}` const.", parent_js_name, js_name),
            &get_features_doc(options, parent_name.to_string()),
        );

        quote! {
            #unstable_attr
            #doc_comment
            #unstable_docs
            pub const #name: #ty = #value as #ty;
        }
    }
}

pub struct Interface {
    pub name: Ident,
    pub js_name: String,
    pub deprecated: Option<String>,
    pub has_interface: bool,
    pub parents: Vec<Ident>,
    pub consts: Vec<InterfaceConst>,
    pub attributes: Vec<InterfaceAttribute>,
    pub methods: Vec<InterfaceMethod>,
    pub unstable: bool,
}

impl Interface {
    pub fn generate(&self, options: &Options) -> TokenStream {
        let Interface {
            name,
            js_name,
            deprecated,
            has_interface,
            parents,
            consts,
            attributes,
            methods,
            unstable,
        } = self;

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);

        let doc_comment = comment(
            format!("The `{}` class.\n\n{}", name, mdn_doc(js_name, None)),
            &get_features_doc(options, name.to_string()),
        );

        let deprecated = deprecated
            .as_ref()
            .map(|msg| quote!( #[deprecated(note = #msg)] ));

        let is_type_of = if *has_interface {
            None
        } else {
            Some(quote!(is_type_of = |_| false,))
        };

        let prefixes = if POLYFILL_INTERFACES.contains(js_name.as_str()) {
            Some(quote!(vendor_prefix = webkit,))
        } else {
            None
        };

        let extends = parents
            .into_iter()
            .map(|x| quote!( extends = #x, ))
            .collect::<Vec<_>>();

        let consts = consts
            .into_iter()
            .map(|x| x.generate(options, &name, js_name))
            .collect::<Vec<_>>();

        let consts = if consts.is_empty() {
            None
        } else {
            Some(quote! {
                #unstable_attr
                impl #name {
                    #(#deprecated #consts)*
                }
            })
        };

        let attributes = attributes
            .into_iter()
            .map(|x| x.generate(options, &name, js_name, &parents))
            .collect::<Vec<_>>();

        let methods = methods
            .into_iter()
            .map(|x| x.generate(options, &name, js_name.to_string(), &parents))
            .collect::<Vec<_>>();

        let js_ident = raw_ident(js_name);

        quote! {
            #![allow(unused_imports)]
            use super::*;
            use wasm_bindgen::prelude::*;

            #unstable_attr
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(
                    #is_type_of
                    #prefixes
                    #(#extends)*
                    extends = ::js_sys::Object,
                    js_name = #js_ident,
                    typescript_type = #js_name
                )]
                #[derive(Debug, Clone, PartialEq, Eq)]
                #doc_comment
                #unstable_docs
                #deprecated
                pub type #name;

                #(#deprecated #attributes)*
                #(#deprecated #methods)*
            }

            #consts
        }
    }
}

pub struct DictionaryField {
    pub name: Ident,
    pub js_name: String,
    pub ty: Type,
    pub required: bool,
}

impl DictionaryField {
    fn generate_rust(&self, options: &Options, parent_name: String, unstable: bool) -> TokenStream {
        let DictionaryField {
            name,
            js_name,
            ty,
            required: _,
        } = self;

        let unstable_attr = maybe_unstable_attr(unstable);
        let unstable_docs = maybe_unstable_docs(unstable);

        let mut features = BTreeSet::new();

        add_features(&mut features, ty);

        features.remove(&parent_name);

        let cfg_features = get_cfg_features(options, &features);

        features.insert(parent_name);

        let doc_comment = comment(
            format!("Change the `{}` field of this object.", js_name),
            &required_doc_string(options, &features),
        );

        quote! {
            #unstable_attr
            #cfg_features
            #doc_comment
            #unstable_docs
            pub fn #name(&mut self, val: #ty) -> &mut Self {
                use wasm_bindgen::JsValue;
                let r = ::js_sys::Reflect::set(
                    self.as_ref(),
                    &JsValue::from(#js_name),
                    &JsValue::from(val),
                );
                debug_assert!(r.is_ok(), "setting properties should never fail on our dictionary objects");
                let _ = r;
                self
            }
        }
    }
}

pub struct Dictionary {
    pub name: Ident,
    pub js_name: String,
    pub fields: Vec<DictionaryField>,
    pub unstable: bool,
}

impl Dictionary {
    pub fn generate(&self, options: &Options) -> TokenStream {
        let Dictionary {
            name,
            js_name,
            fields,
            unstable,
        } = self;

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);

        let js_name = raw_ident(js_name);

        let mut required_features = BTreeSet::new();
        let mut required_args = vec![];
        let mut required_calls = vec![];

        for field in fields.iter() {
            if field.required {
                let name = &field.name;
                let ty = &field.ty;
                required_args.push(quote!( #name: #ty ));
                required_calls.push(quote!( ret.#name(#name); ));
                add_features(&mut required_features, &field.ty);
            }
        }

        required_features.remove(&name.to_string());

        let cfg_features = get_cfg_features(options, &required_features);

        required_features.insert(name.to_string());

        let doc_comment = comment(
            format!("The `{}` dictionary.", name),
            &get_features_doc(options, name.to_string()),
        );
        let ctor_doc_comment = comment(
            format!("Construct a new `{}`.", name),
            &required_doc_string(options, &required_features),
        );

        let fields = fields
            .into_iter()
            .map(|field| field.generate_rust(options, name.to_string(), *unstable))
            .collect::<Vec<_>>();

        quote! {
            #![allow(unused_imports)]
            use super::*;
            use wasm_bindgen::prelude::*;

            #unstable_attr
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(extends = ::js_sys::Object, js_name = #js_name)]
                #[derive(Debug, Clone, PartialEq, Eq)]
                #doc_comment
                #unstable_docs
                pub type #name;
            }

            #unstable_attr
            impl #name {
                #cfg_features
                #ctor_doc_comment
                #unstable_docs
                pub fn new(#(#required_args),*) -> Self {
                    #[allow(unused_mut)]
                    let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
                    #(#required_calls)*
                    ret
                }

                #(#fields)*
            }
        }
    }
}

pub struct Function {
    pub name: Ident,
    pub js_name: String,
    pub arguments: Vec<(Ident, Type)>,
    pub ret_ty: Option<Type>,
    pub catch: bool,
    pub variadic: bool,
    pub unstable: bool,
}

impl Function {
    fn generate(
        &self,
        options: &Options,
        parent_name: &Ident,
        parent_js_name: String,
    ) -> TokenStream {
        let Function {
            name,
            js_name,
            arguments,
            ret_ty,
            catch,
            variadic,
            unstable,
        } = self;

        let unstable_attr = maybe_unstable_attr(*unstable);
        let unstable_docs = maybe_unstable_docs(*unstable);

        let js_namespace = raw_ident(&parent_js_name);

        let doc_comment = format!(
            "The `{}.{}()` function.\n\n{}",
            parent_js_name,
            js_name,
            mdn_doc(&parent_js_name, Some(&js_name))
        );

        let mut features = BTreeSet::new();

        for (_, ty) in arguments.iter() {
            add_features(&mut features, ty);
        }

        if let Some(ty) = ret_ty {
            add_features(&mut features, ty);
        }

        features.remove(&parent_name.to_string());

        let cfg_features = get_cfg_features(options, &features);

        features.insert(parent_name.to_string());

        let doc_comment = comment(doc_comment, &required_doc_string(options, &features));

        let ret = ret_ty.as_ref().map(|ret| quote!( #ret ));

        let ret = if *catch {
            let ret = ret.unwrap_or_else(|| quote!(()));
            Some(quote!( Result<#ret, JsValue> ))
        } else {
            ret
        };

        let ret = ret.as_ref().map(|ret| quote!( -> #ret ));

        let catch = if *catch { Some(quote!(catch,)) } else { None };

        let arguments = generate_arguments(arguments, *variadic);
        let variadic = generate_variadic(*variadic);

        let js_name = raw_ident(js_name);

        quote! {
            #unstable_attr
            #cfg_features
            #[wasm_bindgen(
                #catch
                #variadic
                js_namespace = #js_namespace,
                js_name = #js_name
            )]
            #doc_comment
            #unstable_docs
            pub fn #name(#(#arguments),*) #ret;
        }
    }
}

pub struct Namespace {
    pub name: Ident,
    pub js_name: String,
    pub functions: Vec<Function>,
}

impl Namespace {
    pub fn generate(&self, options: &Options) -> TokenStream {
        let Namespace {
            name,
            js_name,
            functions,
        } = self;

        let functions = functions
            .into_iter()
            .map(|x| x.generate(options, &name, js_name.to_string()))
            .collect::<Vec<_>>();

        quote! {
            pub mod #name {
                #![allow(unused_imports)]
                use super::super::*;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    #(#functions)*
                }
            }
        }
    }
}
