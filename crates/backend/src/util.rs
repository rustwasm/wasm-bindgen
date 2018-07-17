use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};

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
        kind: ast::ImportKind::Function(function),
    }
}

/// Small utility used when generating symbol names.
///
/// Hashes the public field here along with a few cargo-set env vars to
/// distinguish between runs of the procedural macro.
pub struct ShortHash<T>(pub T);

impl<T: Hash> fmt::Display for ShortHash<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        static HASHED: AtomicBool = ATOMIC_BOOL_INIT;
        static HASH: AtomicUsize = ATOMIC_USIZE_INIT;

        // Try to amortize the cost of loading env vars a lot as we're gonna be
        // hashing for a lot of symbols.
        if !HASHED.load(SeqCst) {
            let mut h = DefaultHasher::new();
            env::var("CARGO_PKG_NAME")
                .expect("should have CARGO_PKG_NAME env var")
                .hash(&mut h);
            env::var("CARGO_PKG_VERSION")
                .expect("should have CARGO_PKG_VERSION env var")
                .hash(&mut h);
            // This may chop off 32 bits on 32-bit platforms, but that's ok, we
            // just want something to mix in below anyway.
            HASH.store(h.finish() as usize, SeqCst);
            HASHED.store(true, SeqCst);
        }

        let mut h = DefaultHasher::new();
        HASH.load(SeqCst).hash(&mut h);
        self.0.hash(&mut h);
        write!(f, "{:016x}", h.finish())
    }
}
