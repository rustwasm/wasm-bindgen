use std::iter::FromIterator;

use ast;
use proc_macro2::{self, Ident};
use syn;

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

/// Create a path type from the given segments. For example an iterator yielding
/// the idents `[foo, bar, baz]` will result in the path type `foo::bar::baz`.
pub fn simple_path_ty<I>(segments: I) -> syn::Type
where
    I: IntoIterator<Item = Ident>,
{
    path_ty(false, segments)
}

/// Create a global path type from the given segments. For example an iterator
/// yielding the idents `[foo, bar, baz]` will result in the path type
/// `::foo::bar::baz`.
pub fn leading_colon_path_ty<I>(segments: I) -> syn::Type
where
    I: IntoIterator<Item = Ident>,
{
    path_ty(true, segments)
}

fn path_ty<I>(leading_colon: bool, segments: I) -> syn::Type
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
            leading_colon: if leading_colon {
                Some(Default::default())
            } else {
                None
            },
            segments: syn::punctuated::Punctuated::from_iter(segments),
        },
    }.into()
}

pub fn ident_ty(ident: Ident) -> syn::Type {
    simple_path_ty(Some(ident))
}

pub fn wrap_import_function(function: ast::ImportFunction) -> ast::Import {
    ast::Import {
        module: None,
        version: None,
        js_namespace: None,
        kind: ast::ImportKind::Function(Box::new(function)),
    }
}
