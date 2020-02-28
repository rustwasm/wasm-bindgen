use std::collections::BTreeSet;
use proc_macro2::TokenStream;
use quote::quote;
use proc_macro2::Literal;
use syn::{Ident, Type, Attribute};
use wasm_bindgen_backend::util::{raw_ident, rust_ident};

use crate::constants::{BUILTIN_IDENTS, POLYFILL_INTERFACES};
use crate::traverse::TraverseType;
use crate::util::{snake_case_ident, required_doc_string, get_cfg_features, mdn_doc};


fn add_features(features: &mut BTreeSet<String>, ty: &impl TraverseType, self_name: &str) {
    ty.traverse_type(&mut |ident| {
        let ident = ident.to_string();

        if ident != self_name && !BUILTIN_IDENTS.contains(ident.as_str()) {
            features.insert(ident);
        }
    });
}

fn get_features_doc(name: String) -> Option<String> {
    let mut features = BTreeSet::new();
    features.insert(name);
    required_doc_string(&features)
}

fn get_features(ty: &impl TraverseType, name: String) -> (Option<Attribute>, Option<String>) {
    let mut features = BTreeSet::new();
    add_features(&mut features, ty, &name);
    let cfg = get_cfg_features(&features);
    features.insert(name);
    let doc = required_doc_string(&features);
    (cfg, doc)
}

fn comment(mut comment: String, features: &Option<String>) -> String {
    if let Some(s) = features {
        comment.push_str(s);
    }

    comment
}



pub struct EnumVariant {
    pub name: Ident,
    pub value: String,
}

impl EnumVariant {
    fn generate(&self) -> TokenStream {
        let EnumVariant {
            name,
            value,
        } = self;

        quote!( #name = #value )
    }
}


pub struct Enum {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

impl Enum {
    pub fn generate(&self) -> TokenStream {
        let Enum {
            name,
            variants,
        } = self;

        let doc_comment = comment(format!("The `{}` enum.", name), &get_features_doc(name.to_string()));

        let variants = variants.into_iter()
            .map(|variant| variant.generate())
            .collect::<Vec<_>>();

        quote! {
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            #[doc = #doc_comment]
            #[derive(Copy, Clone, PartialEq, Debug)]
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
}

impl InterfaceAttribute {
    fn generate(&self, parent_name: &Ident, parent_js_name: &str) -> TokenStream {
        let InterfaceAttribute {
            js_name,
            ty,
            is_static,
            structural,
            catch,
            kind,
        } = self;

        let static_method_of = raw_ident(&parent_js_name);

        let mdn_docs = mdn_doc(parent_js_name, Some(js_name));

        let (cfg_features, doc_comment) = get_features(ty, parent_name.to_string());

        let structural = if *structural {
            quote!( structural, )

        } else {
            quote!( final, )
        };

        let (static_method, this) = if *is_static {
            (
                Some(quote!( static_method_of = #static_method_of, )),
                None,
            )

        } else {
            (
                None,
                Some(quote!( this: &#parent_name, )),
            )
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
                    quote!( getter ),
                    quote!( pub fn #name(#this) -> #ty; ),
                )
            },

            InterfaceAttributeKind::Setter => {
                let name = rust_ident(&format!("set_{}", snake_case_ident(js_name)));

                let ret_ty = if *catch {
                    Some(quote!( -> Result<(), JsValue> ))

                } else {
                    None
                };

                (
                    "Setter",
                    quote!( setter ),
                    quote!( pub fn #name(#this value: #ty) #ret_ty; ),
                )
            },
        };

        let catch = if *catch {
            Some(quote!( catch, ))

        } else {
            None
        };

        let doc_comment = comment(format!("{} for the `{}` field of this object.\n\n{}", prefix, js_name, mdn_docs), &doc_comment);

        let js_name = raw_ident(js_name);

        quote! {
            #[wasm_bindgen(
                #structural
                #catch
                #static_method
                method,
                #attr,
                js_name = #js_name
            )]
            #cfg_features
            #[doc = #doc_comment]
            #def
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum InterfaceMethodKind {
    Constructor,
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
}

impl InterfaceMethod {
    fn generate(&self, parent_name: &Ident, parent_js_name: String) -> TokenStream {
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
        } = self;

        let static_method_of = raw_ident(&parent_js_name);

        let mut is_constructor = false;
        let mut extra_args = vec![];

        let doc_comment = match kind {
            InterfaceMethodKind::Constructor => {
                is_constructor = true;
                extra_args.push(quote!( constructor ));
                format!(
                    "The `new {}(..)` constructor, creating a new \
                     instance of `{0}`.\n\n{}",
                    parent_name,
                    mdn_doc(&parent_js_name, Some(&parent_js_name))
                )
            },
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
            },
            InterfaceMethodKind::IndexingGetter => {
                extra_args.push(quote!( indexing_getter ));
                format!("Indexing getter.\n\n")
            },
            InterfaceMethodKind::IndexingSetter => {
                extra_args.push(quote!( indexing_setter ));
                format!("Indexing setter.\n\n")
            },
            InterfaceMethodKind::IndexingDeleter => {
                extra_args.push(quote!( indexing_deleter ));
                format!("Indexing deleter.\n\n")
            },
        };

        let mut features = BTreeSet::new();

        for (_, ty) in arguments.iter() {
            add_features(&mut features, ty, &parent_name.to_string());
        }

        if let Some(ty) = ret_ty {
            add_features(&mut features, ty, &parent_name.to_string());
        }

        let cfg_features = get_cfg_features(&features);

        features.insert(parent_name.to_string());

        let doc_comment = comment(doc_comment, &required_doc_string(&features));

        let ret = ret_ty.as_ref().map(|ret| quote!( #ret ));

        let ret = if *catch {
            let ret = ret.unwrap_or_else(|| quote!(()));
            Some(quote!( Result<#ret, JsValue> ))

        } else {
            ret
        };

        let ret = ret.as_ref().map(|ret| quote!( -> #ret ));

        let catch = if *catch {
            Some(quote!( catch, ))

        } else {
            None
        };

        let method = if *structural {
            quote!( method, structural, )

        } else {
            quote!( method, final, )
        };

        let method = if is_constructor {
            None

        } else {
            Some(method)
        };

        let variadic = if *variadic {
            Some(quote!( variadic, ))

        } else {
            None
        };

        let (static_method, this) = if *is_static {
            (
                Some(quote!( static_method_of = #static_method_of, )),
                None,
            )

        } else {
            (
                None,
                Some(quote!( this: &#parent_name, )),
            )
        };

        let arguments = arguments.into_iter()
            .map(|(name, ty)| {
                quote!( #name: #ty )
            })
            .collect::<Vec<_>>();

        quote! {
            #cfg_features
            #[wasm_bindgen(
                #catch
                #method
                #variadic
                #static_method
                #(#extra_args),*
            )]
            #[doc = #doc_comment]
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
    pub ty: syn::Type,
    pub value: InterfaceConstValue,
}

impl InterfaceConst {
    fn generate(&self) -> TokenStream {
        let name = &self.name;
        let ty = &self.ty;
        let value = self.value.generate();

        quote! {
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
}

impl Interface {
    pub fn generate(&self) -> TokenStream {
        let Interface {
            name,
            js_name,
            deprecated,
            has_interface,
            parents,
            consts,
            attributes,
            methods,
        } = self;

        let doc_comment = comment(
            format!("The `{}` class.\n\n{}", name, mdn_doc(js_name, None)),
            &get_features_doc(name.to_string()),
        );

        let deprecated = deprecated.as_ref().map(|msg| {
            quote!( #[deprecated(note = #msg)] )
        });

        let is_type_of = if *has_interface {
            None
        } else {
            Some(quote!( is_type_of = |_| false, ))
        };

        let prefixes = if POLYFILL_INTERFACES.contains(js_name.as_str()) {
            Some(quote!( vendor_prefix = webkit, ))
        } else {
            None
        };

        let extends = parents.into_iter()
            .map(|x| quote!( extends = #x, ))
            .collect::<Vec<_>>();

        let consts = consts.into_iter()
            .map(|x| x.generate())
            .collect::<Vec<_>>();

        let consts = if consts.is_empty() {
            None

        } else {
            Some(quote! {
                impl #name {
                    #(#deprecated #consts)*
                }
            })
        };

        let attributes = attributes.into_iter()
            .map(|x| x.generate(&name, js_name))
            .collect::<Vec<_>>();

        let methods = methods.into_iter()
            .map(|x| x.generate(&name, js_name.to_string()))
            .collect::<Vec<_>>();

        let js_name = raw_ident(js_name);

        quote! {
            use super::*;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(
                    #is_type_of
                    #prefixes
                    #(#extends)*
                    extends = ::js_sys::Object,
                    js_name = #js_name,
                    typescript_name = #js_name
                )]
                #[derive(Debug, Clone, PartialEq, Eq)]
                #[doc = #doc_comment]
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
    fn generate_rust(&self, parent_name: String) -> TokenStream {
        let DictionaryField {
            name,
            js_name,
            ty,
            required: _,
        } = self;

        let (cfg_features, doc_comment) = get_features(ty, parent_name);

        let doc_comment = comment(format!("Change the `{}` field of this object.", js_name), &doc_comment);

        quote! {
            #cfg_features
            #[doc = #doc_comment]
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
}

impl Dictionary {
    pub fn generate(&self) -> TokenStream {
        let Dictionary {
            name,
            js_name,
            fields,
        } = self;

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
                add_features(&mut required_features, &field.ty, &name.to_string());
            }
        }

        let cfg_features = get_cfg_features(&required_features);

        required_features.insert(name.to_string());

        let doc_features = required_doc_string(&required_features);

        let doc_comment = comment(format!("The `{}` dictionary.", name), &doc_features);
        let ctor_doc_comment = comment(format!("Construct a new `{}`.", name), &doc_features);

        let fields = fields.into_iter()
            .map(|field| field.generate_rust(name.to_string()))
            .collect::<Vec<_>>();

        quote! {
            use super::*;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(extends = ::js_sys::Object, js_name = #js_name)]
                #[doc = #doc_comment]
                pub type #name;
            }

            impl #name {
                #cfg_features
                #[doc = #ctor_doc_comment]
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
    pub structural: bool,
    pub catch: bool,
    pub variadic: bool,
}

impl Function {
    fn generate(&self, parent_name: &Ident, parent_js_name: String) -> TokenStream {
        let Function {
            name,
            js_name,
            arguments,
            ret_ty,
            structural,
            catch,
            variadic,
        } = self;

        let js_namespace = raw_ident(&parent_js_name);

        let doc_comment = format!(
            "The `{}.{}()` function.\n\n{}",
            parent_js_name,
            js_name,
            mdn_doc(&parent_js_name, Some(&js_name))
        );

        let mut features = BTreeSet::new();

        for (_, ty) in arguments.iter() {
            add_features(&mut features, ty, &parent_name.to_string());
        }

        if let Some(ty) = ret_ty {
            add_features(&mut features, ty, &parent_name.to_string());
        }

        let cfg_features = get_cfg_features(&features);

        features.insert(parent_name.to_string());

        let doc_comment = comment(doc_comment, &required_doc_string(&features));

        let ret = ret_ty.as_ref().map(|ret| quote!( #ret ));

        let ret = if *catch {
            let ret = ret.unwrap_or_else(|| quote!(()));
            Some(quote!( Result<#ret, JsValue> ))

        } else {
            ret
        };

        let ret = ret.as_ref().map(|ret| quote!( -> #ret ));

        let catch = if *catch {
            Some(quote!( catch, ))

        } else {
            None
        };

        let structural = if *structural {
            quote!( structural, )

        } else {
            quote!( final, )
        };

        let variadic = if *variadic {
            Some(quote!( variadic, ))

        } else {
            None
        };

        let arguments = arguments.into_iter()
            .map(|(name, ty)| {
                quote!( #name: #ty )
            })
            .collect::<Vec<_>>();

        let js_name = raw_ident(js_name);

        quote! {
            #cfg_features
            #[wasm_bindgen(
                #catch
                #variadic
                method,
                #structural
                js_namespace = #js_namespace,
                js_name = #js_name
            )]
            #[doc = #doc_comment]
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
    pub fn generate(&self) -> TokenStream {
        let Namespace {
            name,
            js_name,
            functions,
        } = self;

        let functions = functions.into_iter()
            .map(|x| x.generate(&name, js_name.to_string()))
            .collect::<Vec<_>>();

        quote! {
            pub mod #name {
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
