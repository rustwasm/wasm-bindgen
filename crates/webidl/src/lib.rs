/*!
# `wasm_bindgen_webidl`

Converts WebIDL into wasm-bindgen's internal AST form, so that bindings can be
emitted for the types and methods described in the WebIDL.
 */

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

extern crate failure;
extern crate heck;
#[macro_use]
extern crate log;
extern crate proc_macro2;
extern crate quote;
extern crate syn;
extern crate wasm_bindgen_backend as backend;
extern crate webidl;

use failure::ResultExt;
use heck::SnakeCase;
use proc_macro2::Ident;
use quote::ToTokens;
use std::fs;
use std::io::{self, Read};
use std::iter::FromIterator;
use std::path::Path;

/// Either `Ok(t)` or `Err(failure::Error)`.
pub type Result<T> = ::std::result::Result<T, failure::Error>;

/// Parse the WebIDL at the given path into a wasm-bindgen AST.
pub fn parse_file(webidl_path: &Path) -> Result<backend::ast::Program> {
    let file = fs::File::open(webidl_path).context("opening WebIDL file")?;
    let mut file = io::BufReader::new(file);
    let mut source = String::new();
    file.read_to_string(&mut source)
        .context("reading WebIDL file")?;
    parse(&source)
}

/// Parse a string of WebIDL source text into a wasm-bindgen AST.
pub fn parse(webidl_source: &str) -> Result<backend::ast::Program> {
    let definitions = webidl::parse_string(webidl_source).context("parsing WebIDL source text")?;

    let mut program = backend::ast::Program::default();
    definitions.webidl_parse(&mut program, ())?;

    Ok(program)
}

/// Compile the given WebIDL file into Rust source text containing
/// `wasm-bindgen` bindings to the things described in the WebIDL.
pub fn compile_file(webidl_path: &Path) -> Result<String> {
    let ast = parse_file(webidl_path)?;
    Ok(compile_ast(&ast))
}

/// Compile the given WebIDL source text into Rust source text containing
/// `wasm-bindgen` bindings to the things described in the WebIDL.
pub fn compile(webidl_source: &str) -> Result<String> {
    let ast = parse(webidl_source)?;
    Ok(compile_ast(&ast))
}

fn compile_ast(ast: &backend::ast::Program) -> String {
    let mut tokens = proc_macro2::TokenStream::new();
    ast.to_tokens(&mut tokens);
    tokens.to_string()
}

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
fn rust_ident(name: &str) -> Ident {
    if is_rust_keyword(name) {
        Ident::new(&format!("{}_", name), proc_macro2::Span::call_site())
    } else {
        raw_ident(name)
    }
}

// Create an `Ident` without checking to see if it conflicts with a Rust
// keyword.
fn raw_ident(name: &str) -> Ident {
    Ident::new(name, proc_macro2::Span::call_site())
}

trait WebidlParse<'a> {
    type Extra;

    fn webidl_parse(&self, program: &mut backend::ast::Program, extra: Self::Extra) -> Result<()>;
}

impl<'a> WebidlParse<'a> for Vec<webidl::ast::Definition> {
    type Extra = ();

    fn webidl_parse(&self, program: &mut backend::ast::Program, _: ()) -> Result<()> {
        for def in self {
            def.webidl_parse(program, ())?;
        }
        Ok(())
    }
}

impl<'a> WebidlParse<'a> for webidl::ast::Definition {
    type Extra = ();

    fn webidl_parse(&self, program: &mut backend::ast::Program, _: ()) -> Result<()> {
        match *self {
            webidl::ast::Definition::Interface(ref interface) => {
                interface.webidl_parse(program, ())
            }
            // TODO
            webidl::ast::Definition::Callback(..)
            | webidl::ast::Definition::Dictionary(..)
            | webidl::ast::Definition::Enum(..)
            | webidl::ast::Definition::Implements(..)
            | webidl::ast::Definition::Includes(..)
            | webidl::ast::Definition::Mixin(..)
            | webidl::ast::Definition::Namespace(..)
            | webidl::ast::Definition::Typedef(..) => {
                warn!("Unsupported WebIDL definition: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<'a> for webidl::ast::Interface {
    type Extra = ();

    fn webidl_parse(&self, program: &mut backend::ast::Program, _: ()) -> Result<()> {
        match *self {
            webidl::ast::Interface::NonPartial(ref interface) => {
                interface.webidl_parse(program, ())
            }
            // TODO
            webidl::ast::Interface::Callback(..) | webidl::ast::Interface::Partial(..) => {
                warn!("Unsupported WebIDL interface: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<'a> for webidl::ast::NonPartialInterface {
    type Extra = ();

    fn webidl_parse(&self, program: &mut backend::ast::Program, _: ()) -> Result<()> {
        program.imports.push(backend::ast::Import {
            module: None,
            version: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Type(backend::ast::ImportType {
                vis: syn::Visibility::Public(syn::VisPublic {
                    pub_token: Default::default(),
                }),
                name: rust_ident(&self.name),
            }),
        });

        for member in &self.members {
            member.webidl_parse(program, &self.name)?;
        }

        Ok(())
    }
}

impl<'a> WebidlParse<'a> for webidl::ast::InterfaceMember {
    type Extra = &'a str;

    fn webidl_parse(&self, program: &mut backend::ast::Program, self_name: &'a str) -> Result<()> {
        match *self {
            webidl::ast::InterfaceMember::Operation(ref op) => op.webidl_parse(program, self_name),
            // TODO
            webidl::ast::InterfaceMember::Attribute(_)
            | webidl::ast::InterfaceMember::Const(_)
            | webidl::ast::InterfaceMember::Iterable(_)
            | webidl::ast::InterfaceMember::Maplike(_)
            | webidl::ast::InterfaceMember::Setlike(_) => {
                warn!("Unsupported WebIDL interface member: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<'a> for webidl::ast::Operation {
    type Extra = &'a str;

    fn webidl_parse(&self, program: &mut backend::ast::Program, self_name: &'a str) -> Result<()> {
        match *self {
            webidl::ast::Operation::Regular(ref op) => op.webidl_parse(program, self_name),
            // TODO
            webidl::ast::Operation::Special(_)
            | webidl::ast::Operation::Static(_)
            | webidl::ast::Operation::Stringifier(_) => {
                warn!("Unsupported WebIDL operation: {:?}", self);
                Ok(())
            }
        }
    }
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

fn ident_ty(ident: Ident) -> syn::Type {
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
enum TypePosition {
    Argument,
    Return,
}

fn webidl_ty_to_syn_ty(ty: &webidl::ast::Type, pos: TypePosition) -> Option<syn::Type> {
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

fn simple_fn_arg(ident: Ident, ty: syn::Type) -> syn::FnArg {
    syn::FnArg::Captured(syn::ArgCaptured {
        pat: syn::Pat::Ident(syn::PatIdent {
            by_ref: None,
            mutability: None,
            ident,
            subpat: None,
        }),
        colon_token: Default::default(),
        ty,
    })
}

impl<'a> WebidlParse<'a> for webidl::ast::RegularOperation {
    type Extra = &'a str;

    fn webidl_parse(&self, program: &mut backend::ast::Program, self_name: &'a str) -> Result<()> {
        let name = match self.name {
            None => {
                warn!(
                    "Operations without a name are unsupported. Skipping {:?}",
                    self
                );
                return Ok(());
            }
            Some(ref name) => name,
        };

        let rust_name = rust_ident(&name.to_snake_case());
        let name = raw_ident(name);

        let (output, ret) = match self.return_type {
            webidl::ast::ReturnType::Void => (syn::ReturnType::Default, None),
            webidl::ast::ReturnType::NonVoid(ref ty) => {
                match webidl_ty_to_syn_ty(ty, TypePosition::Return) {
                    None => {
                        warn!(
                        "Operation's return type is not yet supported: {:?}. Skipping bindings for {:?}",
                        ty, self
                    );
                        return Ok(());
                    }
                    Some(ty) => (
                        syn::ReturnType::Type(Default::default(), Box::new(ty.clone())),
                        Some(ty),
                    ),
                }
            }
        };

        let mut inputs = Vec::with_capacity(self.arguments.len() + 1);
        let mut arguments = Vec::with_capacity(self.arguments.len() + 1);

        let self_ty = ident_ty(rust_ident(self_name));
        let self_ref_ty = shared_ref(self_ty.clone());
        inputs.push(simple_fn_arg(raw_ident("self_"), self_ref_ty.clone()));
        arguments.push(self_ref_ty);

        for arg in &self.arguments {
            if arg.variadic {
                warn!(
                    "Variadic arguments are not supported yet. Skipping bindings for {:?}",
                    self
                );
                return Ok(());
            }

            match webidl_ty_to_syn_ty(&arg.type_, TypePosition::Argument) {
                None => {
                    warn!(
                        "Argument's type is not yet supported: {:?}. Skipping bindings for {:?}",
                        arg.type_, self
                    );
                    return Ok(());
                }
                Some(ty) => {
                    inputs.push(simple_fn_arg(rust_ident(&arg.name), ty.clone()));
                    arguments.push(ty);
                }
            }
        }

        let shim = rust_ident(&format!("__wbg_f_{}_{}_{}", name, rust_name, self_name));

        program.imports.push(backend::ast::Import {
            module: None,
            version: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Function(backend::ast::ImportFunction {
                function: backend::ast::Function {
                    name,
                    arguments,
                    ret,
                    opts: backend::ast::BindgenAttrs {
                        attrs: vec![backend::ast::BindgenAttr::Method],
                    },
                    rust_attrs: vec![],
                    rust_decl: Box::new(syn::FnDecl {
                        fn_token: Default::default(),
                        generics: Default::default(),
                        paren_token: Default::default(),
                        inputs: syn::punctuated::Punctuated::from_iter(inputs),
                        variadic: None,
                        output,
                    }),
                    rust_vis: syn::Visibility::Public(syn::VisPublic {
                        pub_token: Default::default(),
                    }),
                },
                rust_name,
                kind: backend::ast::ImportFunctionKind::Method {
                    class: self_name.to_string(),
                    ty: self_ty,
                },
                shim,
            }),
        });

        Ok(())
    }
}
