/*!
# `wasm_bindgen_webidl`

Converts WebIDL into wasm-bindgen's internal AST form, so that bindings can be
emitted for the types and methods described in the WebIDL.
 */

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

extern crate failure;
extern crate proc_macro2;
extern crate syn;
extern crate wasm_bindgen_backend as backend;
extern crate webidl;

use failure::ResultExt;
use proc_macro2::Ident;
use std::fs;
use std::io::{self, Read};
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
    definitions.webidl_parse(&mut program)?;

    Ok(program)
}

trait WebidlParse {
    fn webidl_parse(&self, program: &mut backend::ast::Program) -> Result<()>;
}

impl WebidlParse for Vec<webidl::ast::Definition> {
    fn webidl_parse(&self, program: &mut backend::ast::Program) -> Result<()> {
        for def in self {
            def.webidl_parse(program)?;
        }
        Ok(())
    }
}

impl WebidlParse for webidl::ast::Definition {
    fn webidl_parse(&self, program: &mut backend::ast::Program) -> Result<()> {
        match *self {
            webidl::ast::Definition::Interface(ref interface) => interface.webidl_parse(program),
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

impl WebidlParse for webidl::ast::Interface {
    fn webidl_parse(&self, program: &mut backend::ast::Program) -> Result<()> {
        match *self {
            webidl::ast::Interface::NonPartial(ref interface) => interface.webidl_parse(program),
            // TODO
            webidl::ast::Interface::Callback(..) | webidl::ast::Interface::Partial(..) => Ok(()),
        }
    }
}

impl WebidlParse for webidl::ast::NonPartialInterface {
    fn webidl_parse(&self, program: &mut backend::ast::Program) -> Result<()> {
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
        println!("{:#?}", self);
        Ok(())
    }
}
