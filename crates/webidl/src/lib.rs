/*!
# `wasm_bindgen_webidl`

Converts WebIDL into wasm-bindgen's internal AST form, so that bindings can be
emitted for the types and methods described in the WebIDL.
 */

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/wasm-bindgen-webidl/0.2")]

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate heck;
#[macro_use]
extern crate log;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate wasm_bindgen_backend as backend;
extern crate webidl;

mod first_pass;
mod util;
mod error;

use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Read};
use std::iter::FromIterator;
use std::path::Path;

use backend::defined::{ImportedTypeDefinitions, RemoveUndefinedImports};
use backend::util::{ident_ty, rust_ident, wrap_import_function};
use failure::{ResultExt, Fail};
use heck::{ShoutySnakeCase};
use quote::ToTokens;

use first_pass::{FirstPass, FirstPassRecord};
use util::{public, webidl_const_ty_to_syn_ty, webidl_const_v_to_backend_const_v, TypePosition, camel_case_ident, mdn_doc};

pub use error::{Error, ErrorKind, Result};

/// Parse the WebIDL at the given path into a wasm-bindgen AST.
fn parse_file(webidl_path: &Path) -> Result<backend::ast::Program> {
    let file = fs::File::open(webidl_path).context(ErrorKind::OpeningWebIDLFile)?;
    let mut file = io::BufReader::new(file);
    let mut source = String::new();
    file.read_to_string(&mut source).context(ErrorKind::ReadingWebIDLFile)?;
    parse(&source)
}

/// Parse a string of WebIDL source text into a wasm-bindgen AST.
fn parse(webidl_source: &str) -> Result<backend::ast::Program> {
    let definitions = match webidl::parse_string(webidl_source) {
        Ok(def) => def,
        Err(e) => {
            let kind = match &e {
                webidl::ParseError::InvalidToken { location } => {
                    ErrorKind::ParsingWebIDLSourcePos(*location)
                }
                webidl::ParseError::UnrecognizedToken { token: Some((start, ..)), .. } => {
                    ErrorKind::ParsingWebIDLSourcePos(*start)
                }
                webidl::ParseError::ExtraToken { token: (start, ..) } => {
                    ErrorKind::ParsingWebIDLSourcePos(*start)
                },
                _ => ErrorKind::ParsingWebIDLSource
            };
            return Err(e.context(kind).into());
        }
    };

    let mut first_pass_record = Default::default();
    definitions.first_pass(&mut first_pass_record, ())?;
    let mut program = Default::default();
    definitions.webidl_parse(&mut program, &first_pass_record, ())?;

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

/// Run codegen on the AST to generate rust code.
fn compile_ast(mut ast: backend::ast::Program) -> String {
    let mut defined = BTreeSet::from_iter(
        vec![
            "str", "char", "bool", "JsValue", "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64",
            "usize", "isize", "f32", "f64", "Result", "String", "Vec", "Option",
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

/// The main trait for parsing WebIDL AST into wasm-bindgen AST.
trait WebidlParse<Ctx> {
    /// Parse `self` into wasm-bindgen AST, and insert it into `program`.
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        context: Ctx,
    ) -> Result<()>;
}

impl WebidlParse<()> for [webidl::ast::Definition] {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        for def in self {
            def.webidl_parse(program, first_pass, ())?;
        }
        Ok(())
    }
}

impl WebidlParse<()> for webidl::ast::Definition {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        match self {
            webidl::ast::Definition::Enum(enumeration) => {
                enumeration.webidl_parse(program, first_pass, ())?
            }
            webidl::ast::Definition::Includes(includes) => {
                includes.webidl_parse(program, first_pass, ())?
            }
            webidl::ast::Definition::Interface(interface) => {
                interface.webidl_parse(program, first_pass, ())?
            }
            webidl::ast::Definition::Typedef(typedef) => {
                typedef.webidl_parse(program, first_pass, ())?
            }
            // TODO
            webidl::ast::Definition::Callback(..)
            | webidl::ast::Definition::Dictionary(..)
            | webidl::ast::Definition::Implements(..)
            | webidl::ast::Definition::Namespace(..) => {
                warn!("Unsupported WebIDL definition: {:?}", self)
            }
            webidl::ast::Definition::Mixin(_) => {
                // handled in the first pass
            }
        }
        Ok(())
    }
}

impl WebidlParse<()> for webidl::ast::Includes {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        match first_pass.mixins.get(&self.includee) {
            Some(mixin) => {
                if let Some(non_partial) = mixin.non_partial {
                    for member in &non_partial.members {
                        member.webidl_parse(program, first_pass, &self.includer)?;
                    }
                }
                for partial in &mixin.partials {
                    for member in &partial.members {
                        member.webidl_parse(program, first_pass, &self.includer)?;
                    }
                }
            }
            None => warn!("Tried to include missing mixin {}", self.includee),
        }
        Ok(())
    }
}

impl WebidlParse<()> for webidl::ast::Interface {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        match self {
            webidl::ast::Interface::NonPartial(interface) => {
                interface.webidl_parse(program, first_pass, ())
            }
            webidl::ast::Interface::Partial(interface) => {
                interface.webidl_parse(program, first_pass, ())
            }
            // TODO
            webidl::ast::Interface::Callback(..) => {
                warn!("Unsupported WebIDL interface: {:?}", self);
                Ok(())
            }
        }
    }
}

impl WebidlParse<()> for webidl::ast::Typedef {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        let dest = rust_ident(camel_case_ident(&self.name).as_str());
        let src = match first_pass.webidl_ty_to_syn_ty(&self.type_, TypePosition::Return) {
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
            vis: public(),
            dest,
            src,
        });

        Ok(())
    }
}

impl WebidlParse<()> for webidl::ast::NonPartialInterface {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        if util::is_no_interface_object(&self.extended_attributes) {
            return Ok(());
        }

        let doc_comment = Some(format!("The `{}` object\n\n{}", &self.name, mdn_doc(&self.name, None)));

        program.imports.push(backend::ast::Import {
            module: None,
            version: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Type(backend::ast::ImportType {
                vis: public(),
                name: rust_ident(camel_case_ident(&self.name).as_str()),
                attrs: Vec::new(),
                doc_comment,
            }),
        });

        for extended_attribute in &self.extended_attributes {
            extended_attribute.webidl_parse(program, first_pass, self)?;
        }

        for member in &self.members {
            member.webidl_parse(program, first_pass, &self.name)?;
        }

        Ok(())
    }
}

impl WebidlParse<()> for webidl::ast::PartialInterface {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        if !first_pass.interfaces.contains_key(&self.name) {
            warn!(
                "Partial interface {} missing non-partial interface",
                self.name
            );
        }

        for member in &self.members {
            member.webidl_parse(program, first_pass, &self.name)?;
        }

        Ok(())
    }
}

impl<'a> WebidlParse<&'a webidl::ast::NonPartialInterface> for webidl::ast::ExtendedAttribute {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        interface: &'a webidl::ast::NonPartialInterface,
    ) -> Result<()> {
        let mut add_constructor = |arguments: &[webidl::ast::Argument], class: &str| {
            let self_ty = ident_ty(rust_ident(camel_case_ident(&interface.name).as_str()));

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

            first_pass
                .create_function(
                    "new",
                    first_pass.interfaces
                        .get(&interface.name)
                        .map(|interface_data| interface_data.has_overloaded_constructors)
                        .unwrap_or(false),
                    arguments
                        .iter()
                        .map(|arg| (&*arg.name, &*arg.type_, arg.variadic)),
                    Some(self_ty),
                    kind,
                    structural,
                    throws,
                    None,
                )
                .map(wrap_import_function)
                .map(|import| program.imports.push(import));
        };

        match self {
            webidl::ast::ExtendedAttribute::ArgumentList(
                webidl::ast::ArgumentListExtendedAttribute { arguments, name },
            )
                if name == "Constructor" =>
            {
                add_constructor(arguments, &interface.name)
            }
            webidl::ast::ExtendedAttribute::NoArguments(webidl::ast::Other::Identifier(name))
                if name == "Constructor" =>
            {
                add_constructor(&[], &interface.name)
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
                add_constructor(rhs_arguments, rhs_name)
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
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_name: &'a str,
    ) -> Result<()> {
        match self {
            webidl::ast::InterfaceMember::Attribute(attr) => {
                attr.webidl_parse(program, first_pass, self_name)
            }
            webidl::ast::InterfaceMember::Operation(op) => {
                op.webidl_parse(program, first_pass, self_name)
            }
            webidl::ast::InterfaceMember::Const(cnst) => {
                cnst.webidl_parse(program, first_pass, self_name)
            }
            webidl::ast::InterfaceMember::Iterable(iterable) => {
                iterable.webidl_parse(program, first_pass, self_name)
            }
            // TODO
            | webidl::ast::InterfaceMember::Maplike(_)
            | webidl::ast::InterfaceMember::Setlike(_) => {
                warn!("Unsupported WebIDL interface member: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::MixinMember {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_name: &'a str,
    ) -> Result<()> {
        match self {
            webidl::ast::MixinMember::Attribute(attr) => {
                attr.webidl_parse(program, first_pass, self_name)
            }
            webidl::ast::MixinMember::Operation(op) => {
                op.webidl_parse(program, first_pass, self_name)
            }
            // TODO
            webidl::ast::MixinMember::Const(_) => {
                warn!("Unsupported WebIDL interface member: {:?}", self);
                Ok(())
            }
        }
    }
}
impl<'a> WebidlParse<&'a str> for webidl::ast::Attribute {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_name: &'a str,
    ) -> Result<()> {
        match self {
            webidl::ast::Attribute::Regular(attr) => {
                attr.webidl_parse(program, first_pass, self_name)
            }
            webidl::ast::Attribute::Static(attr) => {
                attr.webidl_parse(program, first_pass, self_name)
            }
            // TODO
            webidl::ast::Attribute::Stringifier(_) => {
                warn!("Unsupported WebIDL attribute: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::Operation {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_name: &'a str,
    ) -> Result<()> {
        match self {
            webidl::ast::Operation::Regular(op) => op.webidl_parse(program, first_pass, self_name),
            webidl::ast::Operation::Static(op) => op.webidl_parse(program, first_pass, self_name),
            // TODO
            webidl::ast::Operation::Special(_) | webidl::ast::Operation::Stringifier(_) => {
                warn!("Unsupported WebIDL operation: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::RegularAttribute {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_name: &'a str,
    ) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        let is_structural = util::is_structural(&self.extended_attributes);
        let throws = util::throws(&self.extended_attributes);

        first_pass
            .create_getter(
                &self.name,
                &self.type_,
                self_name,
                false,
                is_structural,
                throws,
            )
            .map(wrap_import_function)
            .map(|import| program.imports.push(import));

        if !self.read_only {
            first_pass
                .create_setter(
                    &self.name,
                    &self.type_,
                    self_name,
                    false,
                    is_structural,
                    throws,
                )
                .map(wrap_import_function)
                .map(|import| program.imports.push(import));
        }

        Ok(())
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::Iterable {
    fn webidl_parse(
        &self,
        _program: &mut backend::ast::Program,
        _first_pass: &FirstPassRecord<'_>,
        _self_name: &'a str,
    ) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

/* TODO
        let throws = util::throws(&self.extended_attributes);
        let return_value = webidl::ast::ReturnType::NonVoid(self.value_type.clone());
        let args = [];
        first_pass
            .create_basic_method(
                &args,
                Some(&"values".to_string()),
                &return_value,
                self_name,
                false,
                false, // Should be false
            )
            .map(wrap_import_function)
            .map(|import| program.imports.push(import));

        first_pass
            .create_basic_method(
                &args,
                Some(&"keys".to_string()),
                &return_value, // Should be a number
                self_name,
                false,
                false, // Should be false
            )
            .map(wrap_import_function)
            .map(|import| program.imports.push(import));
*/

        Ok(())
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::StaticAttribute {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_name: &'a str,
    ) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        let is_structural = util::is_structural(&self.extended_attributes);
        let throws = util::throws(&self.extended_attributes);

        first_pass
            .create_getter(
                &self.name,
                &self.type_,
                self_name,
                true,
                is_structural,
                throws,
            )
            .map(wrap_import_function)
            .map(|import| program.imports.push(import));

        if !self.read_only {
            first_pass
                .create_setter(
                    &self.name,
                    &self.type_,
                    self_name,
                    true,
                    is_structural,
                    throws,
                )
                .map(wrap_import_function)
                .map(|import| program.imports.push(import));
        }

        Ok(())
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::RegularOperation {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_name: &'a str,
    ) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        let throws = util::throws(&self.extended_attributes);

        first_pass
            .create_basic_method(
                &self.arguments,
                self.name.as_ref(),
                first_pass.interfaces
                    .get(self_name)
                    .map(|interface_data| {
                        interface_data.overloaded_operations.contains(&self.name)
                    })
                    .unwrap_or(false),
                &self.return_type,
                self_name,
                false,
                throws,
            )
            .map(wrap_import_function)
            .map(|import| program.imports.push(import));

        Ok(())
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::StaticOperation {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_name: &'a str,
    ) -> Result<()> {
        if util::is_chrome_only(&self.extended_attributes) {
            return Ok(());
        }

        let throws = util::throws(&self.extended_attributes);

        first_pass
            .create_basic_method(
                &self.arguments,
                self.name.as_ref(),
                first_pass.interfaces
                    .get(self_name)
                    .map(|interface_data| {
                        interface_data.overloaded_operations.contains(&self.name)
                    })
                    .unwrap_or(false),
                &self.return_type,
                self_name,
                true,
                throws,
            )
            .map(wrap_import_function)
            .map(|import| program.imports.push(import));

        Ok(())
    }
}

impl<'a> WebidlParse<()> for webidl::ast::Enum {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        _: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        program.imports.push(backend::ast::Import {
            module: None,
            version: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Enum(backend::ast::ImportEnum {
                vis: public(),
                name: rust_ident(camel_case_ident(&self.name).as_str()),
                variants: self
                    .variants
                    .iter()
                    .map(|v| rust_ident(camel_case_ident(&v).as_str()))
                    .collect(),
                variant_values: self.variants.clone(),
                rust_attrs: vec![parse_quote!(#[derive(Copy, Clone, PartialEq, Debug)])],
            }),
        });

        Ok(())
    }
}

impl<'a> WebidlParse<&'a str> for webidl::ast::Const {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        _: &FirstPassRecord<'_>,
        self_name: &'a str,
    ) -> Result<()> {
        let ty = webidl_const_ty_to_syn_ty(&self.type_);

        program.consts.push(backend::ast::Const {
            vis: public(),
            name: rust_ident(self.name.to_shouty_snake_case().as_str()),
            class: Some(rust_ident(camel_case_ident(&self_name).as_str())),
            ty,
            value: webidl_const_v_to_backend_const_v(&self.value),
        });

        Ok(())
    }
}
