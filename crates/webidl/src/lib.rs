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

use std::fs;
use std::io::{self, Read};
use std::iter;
use std::path::Path;

use failure::ResultExt;
use heck::CamelCase;
use quote::ToTokens;

mod util;

use util::{create_function, ident_ty, raw_ident, rust_ident, webidl_ty_to_syn_ty, TypePosition};

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

trait WebidlParse<Ctx> {
    fn webidl_parse(&self, program: &mut backend::ast::Program, context: Ctx) -> Result<()>;
}

impl WebidlParse<()> for Vec<webidl::ast::Definition> {
    fn webidl_parse(&self, program: &mut backend::ast::Program, _: ()) -> Result<()> {
        for def in self {
            def.webidl_parse(program, ())?;
        }
        Ok(())
    }
}

impl WebidlParse<()> for webidl::ast::Definition {
    fn webidl_parse(&self, program: &mut backend::ast::Program, _: ()) -> Result<()> {
        match *self {
            webidl::ast::Definition::Interface(ref interface) => {
                interface.webidl_parse(program, ())
            }
            webidl::ast::Definition::Typedef(ref typedef) => typedef.webidl_parse(program, ()),
            // TODO
            webidl::ast::Definition::Callback(..)
            | webidl::ast::Definition::Dictionary(..)
            | webidl::ast::Definition::Enum(..)
            | webidl::ast::Definition::Implements(..)
            | webidl::ast::Definition::Includes(..)
            | webidl::ast::Definition::Mixin(..)
            | webidl::ast::Definition::Namespace(..) => {
                warn!("Unsupported WebIDL definition: {:?}", self);
                Ok(())
            }
        }
    }
}

impl WebidlParse<()> for webidl::ast::Interface {
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

impl WebidlParse<()> for webidl::ast::Typedef {
    fn webidl_parse(&self, program: &mut backend::ast::Program, _: ()) -> Result<()> {
        let dest = rust_ident(&self.name);
        let src = match webidl_ty_to_syn_ty(&self.type_, TypePosition::Return) {
            Some(src) => src,
            None => {
                warn!(
                    "typedef's source type is not yet supported: {:?}. Skipping typedef {:?}",
                    *self.type_, self
                );
                return Ok(());
            }
        };

        program.type_aliases.push(backend::ast::TypeAlias {
            vis: syn::Visibility::Public(syn::VisPublic {
                pub_token: Default::default(),
            }),
            dest,
            src,
        });

        Ok(())
    }
}

impl WebidlParse<()> for webidl::ast::NonPartialInterface {
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

impl<'a> WebidlParse<&'a str> for webidl::ast::InterfaceMember {
    fn webidl_parse(&self, program: &mut backend::ast::Program, self_name: &'a str) -> Result<()> {
        match *self {
            webidl::ast::InterfaceMember::Attribute(ref attr) => {
                attr.webidl_parse(program, self_name)
            }
            webidl::ast::InterfaceMember::Operation(ref op) => op.webidl_parse(program, self_name),
            // TODO
            webidl::ast::InterfaceMember::Const(_)
            | webidl::ast::InterfaceMember::Iterable(_)
            | webidl::ast::InterfaceMember::Maplike(_)
            | webidl::ast::InterfaceMember::Setlike(_) => {
                warn!("Unsupported WebIDL interface member: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::Attribute {
    fn webidl_parse(&self, program: &mut backend::ast::Program, self_name: &'a str) -> Result<()> {
        match *self {
            webidl::ast::Attribute::Regular(ref attr) => attr.webidl_parse(program, self_name),
            // TODO
            webidl::ast::Attribute::Static(_) | webidl::ast::Attribute::Stringifier(_) => {
                warn!("Unsupported WebIDL attribute: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::Operation {
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

impl<'a> WebidlParse<&'a str> for webidl::ast::RegularAttribute {
    fn webidl_parse(&self, program: &mut backend::ast::Program, self_name: &'a str) -> Result<()> {
        fn create_getter(
            this: &webidl::ast::RegularAttribute,
            self_name: &str,
        ) -> Option<backend::ast::Import> {
            let ret = match webidl_ty_to_syn_ty(&this.type_, TypePosition::Return) {
                None => {
                    warn!("Attribute's type does not yet support reading: {:?}. Skipping getter binding for {:?}",
                        this.type_, this);
                    return None;
                }
                Some(ty) => Some(ty),
            };

            let kind = backend::ast::ImportFunctionKind::Method {
                class: self_name.to_string(),
                ty: ident_ty(rust_ident(self_name)),
            };

            create_function(
                &this.name,
                iter::empty(),
                kind,
                ret,
                vec![backend::ast::BindgenAttr::Getter(Some(raw_ident(
                    &this.name,
                )))],
            ).map(|function| backend::ast::Import {
                module: None,
                version: None,
                js_namespace: None,
                kind: backend::ast::ImportKind::Function(function),
            })
        }

        fn create_setter(
            this: &webidl::ast::RegularAttribute,
            self_name: &str,
        ) -> Option<backend::ast::Import> {
            let kind = backend::ast::ImportFunctionKind::Method {
                class: self_name.to_string(),
                ty: ident_ty(rust_ident(self_name)),
            };

            create_function(
                &format!("set_{}", this.name.to_camel_case()),
                iter::once((&*this.name, &*this.type_, false)),
                kind,
                None,
                vec![backend::ast::BindgenAttr::Setter(Some(raw_ident(
                    &this.name,
                )))],
            ).map(|function| backend::ast::Import {
                module: None,
                version: None,
                js_namespace: None,
                kind: backend::ast::ImportKind::Function(function),
            })
        }

        create_getter(self, self_name).map(|import| program.imports.push(import));

        if !self.read_only {
            create_setter(self, self_name).map(|import| program.imports.push(import));
        }

        Ok(())
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::RegularOperation {
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

        let kind = backend::ast::ImportFunctionKind::Method {
            class: self_name.to_string(),
            ty: ident_ty(rust_ident(self_name)),
        };

        let ret = match self.return_type {
            webidl::ast::ReturnType::Void => None,
            webidl::ast::ReturnType::NonVoid(ref ty) => {
                match webidl_ty_to_syn_ty(ty, TypePosition::Return) {
                    None => {
                        warn!(
                        "Operation's return type is not yet supported: {:?}. Skipping bindings for {:?}",
                        ty, self
                    );
                        return Ok(());
                    }
                    Some(ty) => Some(ty),
                }
            }
        };

        create_function(
            &name,
            self.arguments
                .iter()
                .map(|arg| (&*arg.name, &*arg.type_, arg.variadic)),
            kind,
            ret,
            Vec::new(),
        ).map(|function| {
            program.imports.push(backend::ast::Import {
                module: None,
                version: None,
                js_namespace: None,
                kind: backend::ast::ImportKind::Function(function),
            })
        });

        Ok(())
    }
}
