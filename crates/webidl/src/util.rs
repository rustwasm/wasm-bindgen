use std::iter::FromIterator;

use backend;
use heck::SnakeCase;
use proc_macro2::{self, Ident};
use syn;
use webidl;

fn is_rust_keyword(name: &str) -> bool {
    match name {
        "abstract" | "alignof" | "as" | "become" | "box" | "break" | "const" | "continue"
        | "crate" | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if"
        | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut"
        | "offsetof" | "override" | "priv" | "proc" | "pub" | "pure" | "ref" | "return"
        | "Self" | "self" | "sizeof" | "static" | "struct" | "super" | "trait" | "true"
        | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while"
        | "yield" | "bool" | "_" => true,
        _ => false,
    }
}

// Create an `Ident`, possibly mangling it if it conflicts with a Rust keyword.
pub fn rust_ident(name: &str) -> Ident {
    if is_rust_keyword(name) {
        Ident::new(&format!("{}_", name), proc_macro2::Span::call_site())
    } else {
        raw_ident(name)
    }
}

// Create an `Ident` without checking to see if it conflicts with a Rust
// keyword.
pub fn raw_ident(name: &str) -> Ident {
    Ident::new(name, proc_macro2::Span::call_site())
}

fn simple_path_ty<I>(segments: I) -> syn::Type
where
    I: IntoIterator<Item = Ident>,
{
    let segments: Vec<_> = segments
        .into_iter()
        .map(|i| syn::PathSegment {
            ident: i,
            arguments: syn::PathArguments::None,
        })
        .collect();

    syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::from_iter(segments),
        },
    }.into()
}

pub fn ident_ty(ident: Ident) -> syn::Type {
    simple_path_ty(Some(ident))
}

fn shared_ref(ty: syn::Type) -> syn::Type {
    syn::TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: Box::new(ty),
    }.into()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypePosition {
    Argument,
    Return,
}

pub fn webidl_ty_to_syn_ty(ty: &webidl::ast::Type, pos: TypePosition) -> Option<syn::Type> {
    // nullable types are not yet supported (see issue #14)
    if ty.nullable {
        return None;
    }
    Some(match ty.kind {
        // `any` becomes `::wasm_bindgen::JsValue`.
        webidl::ast::TypeKind::Any => {
            simple_path_ty(vec![rust_ident("wasm_bindgen"), rust_ident("JsValue")])
        }

        // A reference to a type by name becomes the same thing in the
        // bindings.
        webidl::ast::TypeKind::Identifier(ref id) => ident_ty(rust_ident(id)),

        // Scalars.
        webidl::ast::TypeKind::Boolean => ident_ty(raw_ident("bool")),
        webidl::ast::TypeKind::Byte => ident_ty(raw_ident("i8")),
        webidl::ast::TypeKind::Octet => ident_ty(raw_ident("u8")),
        webidl::ast::TypeKind::RestrictedDouble | webidl::ast::TypeKind::UnrestrictedDouble => {
            ident_ty(raw_ident("f64"))
        }
        webidl::ast::TypeKind::RestrictedFloat | webidl::ast::TypeKind::UnrestrictedFloat => {
            ident_ty(raw_ident("f32"))
        }
        webidl::ast::TypeKind::SignedLong => ident_ty(raw_ident("i32")),
        webidl::ast::TypeKind::SignedLongLong => ident_ty(raw_ident("i64")),
        webidl::ast::TypeKind::SignedShort => ident_ty(raw_ident("i16")),
        webidl::ast::TypeKind::UnsignedLong => ident_ty(raw_ident("u32")),
        webidl::ast::TypeKind::UnsignedLongLong => ident_ty(raw_ident("u64")),
        webidl::ast::TypeKind::UnsignedShort => ident_ty(raw_ident("u16")),

        // `DOMString -> `&str` for arguments
        webidl::ast::TypeKind::DOMString if pos == TypePosition::Argument => {
            shared_ref(ident_ty(raw_ident("str")))
        }
        // `DOMString` is not supported yet in other positions.
        webidl::ast::TypeKind::DOMString => return None,

        // Support for these types is not yet implemented, so skip
        // generating any bindings for this function.
        webidl::ast::TypeKind::ArrayBuffer
        | webidl::ast::TypeKind::ByteString
        | webidl::ast::TypeKind::DataView
        | webidl::ast::TypeKind::Error
        | webidl::ast::TypeKind::Float32Array
        | webidl::ast::TypeKind::Float64Array
        | webidl::ast::TypeKind::FrozenArray(_)
        | webidl::ast::TypeKind::Int16Array
        | webidl::ast::TypeKind::Int32Array
        | webidl::ast::TypeKind::Int8Array
        | webidl::ast::TypeKind::Object
        | webidl::ast::TypeKind::Promise(_)
        | webidl::ast::TypeKind::Record(..)
        | webidl::ast::TypeKind::Sequence(_)
        | webidl::ast::TypeKind::Symbol
        | webidl::ast::TypeKind::USVString
        | webidl::ast::TypeKind::Uint16Array
        | webidl::ast::TypeKind::Uint32Array
        | webidl::ast::TypeKind::Uint8Array
        | webidl::ast::TypeKind::Uint8ClampedArray
        | webidl::ast::TypeKind::Union(_) => {
            return None;
        }
    })
}

fn simple_fn_arg(ident: Ident, ty: syn::Type) -> syn::ArgCaptured {
    syn::ArgCaptured {
        pat: syn::Pat::Ident(syn::PatIdent {
            by_ref: None,
            mutability: None,
            ident,
            subpat: None,
        }),
        colon_token: Default::default(),
        ty,
    }
}

fn webidl_arguments_to_syn_arg_captured<'a, I>(
    arguments: I,
    kind: &backend::ast::ImportFunctionKind,
) -> Option<Vec<syn::ArgCaptured>>
where
    I: Iterator<Item = (&'a str, &'a webidl::ast::Type, bool)>,
{
    let estimate = arguments.size_hint();
    let len = estimate.1.unwrap_or(estimate.0);
    let mut res = if let backend::ast::ImportFunctionKind::Method { ty, .. } = kind {
        let mut res = Vec::with_capacity(len + 1);
        res.push(simple_fn_arg(raw_ident("self_"), shared_ref(ty.clone())));
        res
    } else {
        Vec::with_capacity(len)
    };

    for (name, ty, variadic) in arguments {
        if variadic {
            warn!("Variadic arguments are not supported yet",);
            return None;
        }

        match webidl_ty_to_syn_ty(ty, TypePosition::Argument) {
            None => {
                warn!("Argument's type is not yet supported: {:?}", ty);
                return None;
            }
            Some(ty) => res.push(simple_fn_arg(rust_ident(&name.to_snake_case()), ty)),
        }
    }

    Some(res)
}

pub fn create_function<'a, 'b, I>(
    name: &str,
    arguments: I,
    kind: backend::ast::ImportFunctionKind,
    ret: Option<syn::Type>,
    mut attrs: Vec<backend::ast::BindgenAttr>,
) -> Option<backend::ast::ImportFunction>
where
    I: Iterator<Item = (&'a str, &'a webidl::ast::Type, bool)>,
{
    let rust_name = rust_ident(&name.to_snake_case());
    let name = raw_ident(name);

    let arguments = webidl_arguments_to_syn_arg_captured(arguments, &kind)?;

    let js_ret = ret.clone();

    if let backend::ast::ImportFunctionKind::Method { .. } = kind {
        attrs.push(backend::ast::BindgenAttr::Method);
    }

    let opts = backend::ast::BindgenAttrs { attrs };

    let shim = {
        let ns = match kind {
            backend::ast::ImportFunctionKind::Normal => "",
            backend::ast::ImportFunctionKind::Method { ref class, .. } => class,
            backend::ast::ImportFunctionKind::JsConstructor { ref class, .. } => class,
        };

        raw_ident(&format!("__widl_f_{}_{}", rust_name, ns))
    };

    Some(backend::ast::ImportFunction {
        function: backend::ast::Function {
            name,
            arguments,
            ret,
            opts,
            rust_attrs: vec![],
            rust_vis: syn::Visibility::Public(syn::VisPublic {
                pub_token: Default::default(),
            }),
        },
        rust_name,
        js_ret,
        kind,
        shim,
    })
}
