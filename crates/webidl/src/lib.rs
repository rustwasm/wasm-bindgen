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

mod util;

use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Read};
use std::iter::FromIterator;
use std::path::Path;

use backend::defined::{ImportedTypeDefinitions, RemoveUndefinedImports};
use backend::util::{ident_ty, rust_ident, wrap_import_function};
use failure::ResultExt;
use heck::CamelCase;
use quote::ToTokens;

use util::{
    create_basic_method, create_function, create_getter, create_setter, webidl_ty_to_syn_ty,
    TypePosition,
};

/// Either `Ok(t)` or `Err(failure::Error)`.
pub type Result<T> = ::std::result::Result<T, failure::Error>;

/// Parse the WebIDL at the given path into a wasm-bindgen AST.
fn parse_file(webidl_path: &Path) -> Result<backend::ast::Program> {
    let file = fs::File::open(webidl_path).context("opening WebIDL file")?;
    let mut file = io::BufReader::new(file);
    let mut source = String::new();
    file.read_to_string(&mut source)
        .context("reading WebIDL file")?;
    parse(&source)
}

/// Parse a string of WebIDL source text into a wasm-bindgen AST.
fn parse(webidl_source: &str) -> Result<backend::ast::Program> {
    let definitions = webidl::parse_string(webidl_source).context("parsing WebIDL source text")?;

    let mut program = backend::ast::Program::default();
    definitions.webidl_parse(&mut program, ())?;

    Ok(program)
}

/// Compile the given WebIDL file into Rust source text containing
/// `wasm-bindgen` bindings to the things described in the WebIDL.
pub fn compile_file(webidl_path: &Path) -> Result<String> {
    let ast = parse_file(webidl_path)?;
    Ok(compile_ast(ast))
}

/// Compile the given WebIDL source text into Rust source text containing
/// `wasm-bindgen` bindings to the things described in the WebIDL.
pub fn compile(webidl_source: &str) -> Result<String> {
    let ast = parse(webidl_source)?;
    Ok(compile_ast(ast))
}

fn compile_ast(mut ast: backend::ast::Program) -> String {
    let mut defined = BTreeSet::from_iter(
        vec![
            "str", "char", "bool", "JsValue", "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64",
            "usize", "isize", "f32", "f64", "Result",
        ].into_iter()
            .map(|id| proc_macro2::Ident::new(id, proc_macro2::Span::call_site())),
    );
    ast.imported_type_definitions(&mut |id| {
        defined.insert(id.clone());
    });
    ast.remove_undefined_imports(&|id| defined.contains(id));

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
            webidl::ast::Definition::Enum(ref enumeration) => enumeration.webidl_parse(program, ()),
            // TODO
            webidl::ast::Definition::Callback(..)
            | webidl::ast::Definition::Dictionary(..)
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
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

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
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        program.imports.push(backend::ast::Import {
            module: None,
            version: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Type(backend::ast::ImportType {
                vis: syn::Visibility::Public(syn::VisPublic {
                    pub_token: Default::default(),
                }),
                name: rust_ident(&self.name),
                attrs: Vec::new(),
            }),
        });

        for extended_attribute in &self.extended_attributes {
            extended_attribute.webidl_parse(program, self)?;
        }

        for member in &self.members {
            member.webidl_parse(program, &self.name)?;
        }

        Ok(())
    }
}

impl<'a> WebidlParse<&'a webidl::ast::NonPartialInterface> for webidl::ast::ExtendedAttribute {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        interface: &'a webidl::ast::NonPartialInterface,
    ) -> Result<()> {
        let mut add_constructor = |arguments: &[webidl::ast::Argument], class: &str| {
            let self_ty = ident_ty(rust_ident(&interface.name));

            let kind = backend::ast::ImportFunctionKind::Method {
                class: class.to_string(),
                ty: self_ty.clone(),
                kind: backend::ast::MethodKind::Constructor,
            };

            let structural = false;

            // Constructors aren't annotated with `[Throws]` extended attributes
            // (how could they be, since they themselves are extended
            // attributes?) so we must conservatively assume that they can
            // always throw.
            //
            // From https://heycam.github.io/webidl/#Constructor (emphasis
            // mine):
            //
            // > The prose definition of a constructor must either return an IDL
            // > value of a type corresponding to the interface the
            // > `[Constructor]` extended attribute appears on, **or throw an
            // > exception**.
            let throws = true;

            create_function(
                "new",
                arguments
                    .iter()
                    .map(|arg| (&*arg.name, &*arg.type_, arg.variadic)),
                Some(self_ty),
                kind,
                structural,
                throws,
            ).map(|function| {
                program.imports.push(backend::ast::Import {
                    module: None,
                    version: None,
                    js_namespace: None,
                    kind: backend::ast::ImportKind::Function(function),
                })
            })
        };

        match self {
            webidl::ast::ExtendedAttribute::ArgumentList(
                webidl::ast::ArgumentListExtendedAttribute { arguments, name },
            )
                if name == "Constructor" =>
            {
                add_constructor(arguments, &interface.name);
            }
            webidl::ast::ExtendedAttribute::NoArguments(webidl::ast::Other::Identifier(name))
                if name == "Constructor" =>
            {
                add_constructor(&[], &interface.name);
            }
            webidl::ast::ExtendedAttribute::NamedArgumentList(
                webidl::ast::NamedArgumentListExtendedAttribute {
                    lhs_name,
                    rhs_arguments,
                    rhs_name,
                },
            )
                if lhs_name == "NamedConstructor" =>
            {
                add_constructor(rhs_arguments, rhs_name);
            }
            webidl::ast::ExtendedAttribute::ArgumentList(_)
            | webidl::ast::ExtendedAttribute::Identifier(_)
            | webidl::ast::ExtendedAttribute::IdentifierList(_)
            | webidl::ast::ExtendedAttribute::NamedArgumentList(_)
            | webidl::ast::ExtendedAttribute::NoArguments(_) => {
                warn!("Unsupported WebIDL extended attribute: {:?}", self);
            }
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
        match self {
            webidl::ast::Attribute::Regular(attr) => attr.webidl_parse(program, self_name),
            webidl::ast::Attribute::Static(attr) => attr.webidl_parse(program, self_name),
            // TODO
            webidl::ast::Attribute::Stringifier(_) => {
                warn!("Unsupported WebIDL attribute: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::Operation {
    fn webidl_parse(&self, program: &mut backend::ast::Program, self_name: &'a str) -> Result<()> {
        match self {
            webidl::ast::Operation::Regular(op) => op.webidl_parse(program, self_name),
            webidl::ast::Operation::Static(op) => op.webidl_parse(program, self_name),
            // TODO
            webidl::ast::Operation::Special(_) | webidl::ast::Operation::Stringifier(_) => {
                warn!("Unsupported WebIDL operation: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::RegularAttribute {
    fn webidl_parse(&self, program: &mut backend::ast::Program, self_name: &'a str) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        let is_structural = util::is_structural(&self.extended_attributes);
        let throws = util::throws(&self.extended_attributes);

        create_getter(
            &self.name,
            &self.type_,
            self_name,
            false,
            is_structural,
            throws,
        ).map(wrap_import_function)
            .map(|import| program.imports.push(import));

        if !self.read_only {
            create_setter(
                &self.name,
                &self.type_,
                self_name,
                false,
                is_structural,
                throws,
            ).map(wrap_import_function)
                .map(|import| program.imports.push(import));
        }

        Ok(())
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::StaticAttribute {
    fn webidl_parse(&self, program: &mut backend::ast::Program, self_name: &'a str) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        let is_structural = util::is_structural(&self.extended_attributes);
        let throws = util::throws(&self.extended_attributes);

        create_getter(
            &self.name,
            &self.type_,
            self_name,
            true,
            is_structural,
            throws,
        ).map(wrap_import_function)
            .map(|import| program.imports.push(import));

        if !self.read_only {
            create_setter(
                &self.name,
                &self.type_,
                self_name,
                true,
                is_structural,
                throws,
            ).map(wrap_import_function)
                .map(|import| program.imports.push(import));
        }

        Ok(())
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::RegularOperation {
    fn webidl_parse(&self, program: &mut backend::ast::Program, self_name: &'a str) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        let throws = util::throws(&self.extended_attributes);

        create_basic_method(
            &self.arguments,
            self.name.as_ref(),
            &self.return_type,
            self_name,
            false,
            throws,
        ).map(wrap_import_function)
            .map(|import| program.imports.push(import));

        Ok(())
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::StaticOperation {
    fn webidl_parse(&self, program: &mut backend::ast::Program, self_name: &'a str) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        let throws = util::throws(&self.extended_attributes);

        create_basic_method(
            &self.arguments,
            self.name.as_ref(),
            &self.return_type,
            self_name,
            true,
            throws,
        ).map(wrap_import_function)
            .map(|import| program.imports.push(import));

        Ok(())
    }
}

impl<'a> WebidlParse<()> for webidl::ast::Enum {
    fn webidl_parse(&self, program: &mut backend::ast::Program, _: ()) -> Result<()> {
        program.imports.push(backend::ast::Import {
            module: None,
            version: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Enum(backend::ast::ImportEnum {
                vis: syn::Visibility::Public(syn::VisPublic {
                    pub_token: Default::default(),
                }),
                name: rust_ident(self.name.to_camel_case().as_str()),
                variants: self
                    .variants
                    .iter()
                    .map(|v| rust_ident(v.to_camel_case().as_str()))
                    .collect(),
                variant_values: self.variants.clone(),
            }),
        });

        Ok(())
    }
}
