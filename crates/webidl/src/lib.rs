/*!
# `wasm_bindgen_webidl`

Converts WebIDL into wasm-bindgen's internal AST form, so that bindings can be
emitted for the types and methods described in the WebIDL.
 */

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

extern crate failure;
extern crate proc_macro2;
extern crate quote;
extern crate syn;
extern crate wasm_bindgen_backend as backend;
extern crate webidl;

use failure::ResultExt;
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
            | webidl::ast::Definition::Typedef(..) => Ok(()),
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
            webidl::ast::Interface::Callback(..) | webidl::ast::Interface::Partial(..) => Ok(()),
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
                name: Ident::new(&self.name, proc_macro2::Span::call_site()),
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
            | webidl::ast::InterfaceMember::Setlike(_) => Ok(()),
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
            | webidl::ast::Operation::Stringifier(_) => Ok(()),
        }
    }
}

fn simple_path_ty<I>(segments: I) -> syn::Type
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let segments: Vec<_> = segments
        .into_iter()
        .map(|s| syn::PathSegment {
            ident: syn::Ident::new(s.as_ref(), proc_macro2::Span::call_site()),
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

fn shared_ref(ty: syn::Type) -> syn::Type {
    syn::TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: Box::new(ty),
    }.into()
}

fn webidl_ty_to_syn_ty(ty: &webidl::ast::Type) -> Option<syn::Type> {
    Some(match ty.kind {
        // `any` becomes `::wasm_bindgen::JsValue`.
        webidl::ast::TypeKind::Any => simple_path_ty(&["wasm_bindgen", "JsValue"]),

        // A reference to a type by name becomes the same thing in the
        // bindings.
        webidl::ast::TypeKind::Identifier(ref id) => simple_path_ty(Some(id)),

        // Scalars.
        webidl::ast::TypeKind::Boolean => simple_path_ty(Some("bool")),
        webidl::ast::TypeKind::Byte => simple_path_ty(Some("i8")),
        webidl::ast::TypeKind::Octet => simple_path_ty(Some("u8")),
        webidl::ast::TypeKind::RestrictedDouble | webidl::ast::TypeKind::UnrestrictedDouble => {
            simple_path_ty(Some("f64"))
        }
        webidl::ast::TypeKind::RestrictedFloat | webidl::ast::TypeKind::UnrestrictedFloat => {
            simple_path_ty(Some("f32"))
        }
        webidl::ast::TypeKind::SignedLong => simple_path_ty(Some("i32")),
        webidl::ast::TypeKind::SignedLongLong => simple_path_ty(Some("i64")),
        webidl::ast::TypeKind::SignedShort => simple_path_ty(Some("i16")),
        webidl::ast::TypeKind::UnsignedLong => simple_path_ty(Some("u32")),
        webidl::ast::TypeKind::UnsignedLongLong => simple_path_ty(Some("u64")),
        webidl::ast::TypeKind::UnsignedShort => simple_path_ty(Some("u16")),

        // Support for these types is not yet implemented, so skip
        // generating any bindings for this function.
        webidl::ast::TypeKind::ArrayBuffer
        | webidl::ast::TypeKind::ByteString
        | webidl::ast::TypeKind::DOMString
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

fn simple_fn_arg(ident: proc_macro2::Ident, ty: syn::Type) -> syn::FnArg {
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
        let fn_name = match self.name {
            None => return Ok(()),
            Some(ref name) => Ident::new(name, proc_macro2::Span::call_site()),
        };

        let (output, ret) = match self.return_type {
            webidl::ast::ReturnType::Void => (syn::ReturnType::Default, None),
            webidl::ast::ReturnType::NonVoid(ref ty) => match webidl_ty_to_syn_ty(ty) {
                None => return Ok(()),
                Some(ty) => (
                    syn::ReturnType::Type(Default::default(), Box::new(ty.clone())),
                    Some(ty),
                ),
            },
        };

        let mut inputs = Vec::with_capacity(self.arguments.len() + 1);
        let mut arguments = Vec::with_capacity(self.arguments.len() + 1);

        let self_ty = simple_path_ty(Some(self_name));
        let self_ref_ty = shared_ref(self_ty.clone());
        inputs.push(simple_fn_arg(
            proc_macro2::Ident::new("self_", proc_macro2::Span::call_site()),
            self_ref_ty.clone(),
        ));
        arguments.push(self_ref_ty);

        for arg in &self.arguments {
            if arg.optional || arg.variadic {
                // We don't support optional or variadic functions yet; skip
                // bindings for this this whole function.
                return Ok(());
            }

            match webidl_ty_to_syn_ty(&arg.type_) {
                None => return Ok(()),
                Some(ty) => {
                    inputs.push(simple_fn_arg(
                        proc_macro2::Ident::new(&arg.name, proc_macro2::Span::call_site()),
                        ty.clone(),
                    ));
                    arguments.push(ty);
                }
            }
        }

        let rust_name = fn_name.clone();
        let shim = proc_macro2::Ident::new(
            &format!("__wbg_f_{}_{}_{}", fn_name, fn_name, self_name),
            proc_macro2::Span::call_site(),
        );

        program.imports.push(backend::ast::Import {
            module: None,
            version: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Function(backend::ast::ImportFunction {
                function: backend::ast::Function {
                    name: fn_name,
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
