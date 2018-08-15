/*!
# `wasm_bindgen_webidl`

Converts WebIDL into wasm-bindgen's internal AST form, so that bindings can be
emitted for the types and methods described in the WebIDL.
 */

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/wasm-bindgen-webidl/0.2")]

#[macro_use]
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
extern crate weedle;

mod first_pass;
mod idl_type;
mod util;
mod error;

use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Read};
use std::iter::FromIterator;
use std::path::Path;

use backend::TryToTokens;
use backend::defined::{ImportedTypeDefinitions, RemoveUndefinedImports};
use backend::util::{ident_ty, rust_ident, raw_ident, wrap_import_function};
use failure::ResultExt;
use heck::{ShoutySnakeCase, SnakeCase};
use proc_macro2::{Ident, Span};
use weedle::argument::Argument;
use weedle::attribute::{ExtendedAttribute, ExtendedAttributeList};

use first_pass::{FirstPass, FirstPassRecord, OperationId};
use util::{public, webidl_const_v_to_backend_const_v, TypePosition, camel_case_ident, mdn_doc};
use idl_type::{IdlType, ToIdlType};

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
    let definitions = match weedle::parse(webidl_source) {
        Ok(def) => def,
        Err(e) => {
            return Err(match &e {
                weedle::Err::Incomplete(needed) => {
                    format_err!("needed {:?} more bytes", needed)
                        .context(ErrorKind::ParsingWebIDLSource).into()
                }
                weedle::Err::Error(cx) |
                weedle::Err::Failure(cx) => {
                    let remaining = match cx {
                        weedle::Context::Code(remaining, _) => remaining,
                    };
                    let pos = webidl_source.len() - remaining.len();
                    format_err!("failed to parse WebIDL")
                        .context(ErrorKind::ParsingWebIDLSourcePos(pos)).into()
                }
                // webidl::ParseError::UnrecognizedToken { token: Some((start, ..)), .. } => {
                //     ErrorKind::ParsingWebIDLSourcePos(*start)
                // }
                // webidl::ParseError::ExtraToken { token: (start, ..) } => {
                //     ErrorKind::ParsingWebIDLSourcePos(*start)
                // },
                // _ => ErrorKind::ParsingWebIDLSource
            });
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
            "ArrayBuffer", "Object", "Promise",
        ].into_iter()
            .map(|id| proc_macro2::Ident::new(id, proc_macro2::Span::call_site())),
    );
    ast.imported_type_definitions(&mut |id| {
        defined.insert(id.clone());
    });
    ast.remove_undefined_imports(&|id| defined.contains(id));

    let mut tokens = proc_macro2::TokenStream::new();
    if let Err(e) = ast.try_to_tokens(&mut tokens) {
        e.panic();
    }
    tokens.to_string()
}

/// The main trait for parsing WebIDL AST into wasm-bindgen AST.
trait WebidlParse<'src, Ctx> {
    /// Parse `self` into wasm-bindgen AST, and insert it into `program`.
    fn webidl_parse(
        &'src self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        context: Ctx,
    ) -> Result<()>;
}

impl<'src> WebidlParse<'src, ()> for [weedle::Definition<'src>] {
    fn webidl_parse(
        &'src self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        (): (),
    ) -> Result<()> {
        for def in self {
            def.webidl_parse(program, first_pass, ())?;
        }
        Ok(())
    }
}

impl<'src> WebidlParse<'src, ()> for weedle::Definition<'src> {
    fn webidl_parse(
        &'src self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        (): (),
    ) -> Result<()> {
        match self {
            weedle::Definition::Enum(enumeration) => {
                enumeration.webidl_parse(program, first_pass, ())?
            }
            weedle::Definition::Interface(interface) => {
                interface.webidl_parse(program, first_pass, ())?
            }
            weedle::Definition::PartialInterface(interface) => {
                interface.webidl_parse(program, first_pass, ())?
            }
            | weedle::Definition::Typedef(_)
            | weedle::Definition::InterfaceMixin(_)
            | weedle::Definition::PartialInterfaceMixin(_)
            | weedle::Definition::IncludesStatement(..)
            | weedle::Definition::PartialNamespace(..)=> {
                // handled in the first pass
            }
            weedle::Definition::Implements(..) => {
                // nothing to do for this, ignore it
            }
            weedle::Definition::Namespace(namespace) => {
                namespace.webidl_parse(program, first_pass, ())?
            }

            // TODO
            weedle::Definition::Callback(..) => {
                warn!("Unsupported WebIDL Callback definition: {:?}", self)
            }
            weedle::Definition::CallbackInterface(..) => {
                warn!("Unsupported WebIDL CallbackInterface definition: {:?}", self)
            }
            weedle::Definition::Dictionary(..) => {
                warn!("Unsupported WebIDL Dictionary definition: {:?}", self)
            }
            weedle::Definition::PartialDictionary(..) => {
                warn!("Unsupported WebIDL PartialDictionary definition: {:?}", self)
            }
        }
        Ok(())
    }
}

impl<'src> WebidlParse<'src, ()> for weedle::InterfaceDefinition<'src> {
    fn webidl_parse(
        &'src self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if util::is_no_interface_object(&self.attributes) {
            return Ok(());
        }

        let doc_comment = Some(format!(
            "The `{}` object\n\n{}",
            self.identifier.0,
            mdn_doc(self.identifier.0, None),
        ));

        program.imports.push(backend::ast::Import {
            module: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Type(backend::ast::ImportType {
                vis: public(),
                rust_name: rust_ident(camel_case_ident(self.identifier.0).as_str()),
                js_name: self.identifier.0.to_string(),
                attrs: Vec::new(),
                doc_comment,
                instanceof_shim: format!("__widl_instanceof_{}", self.identifier.0),
                extends: first_pass.all_superclasses(self.identifier.0)
                    .map(|name| Ident::new(&name, Span::call_site()))
                    .collect(),
            }),
        });

        if let Some(attrs) = &self.attributes {
            for attr in &attrs.body.list {
                attr.webidl_parse(program, first_pass, self)?;
            }
        }

        fn parse<'src>(
            program: &mut backend::ast::Program,
            first_pass: &FirstPassRecord<'src>,
            self_name: &str,
            mixin_name: &str,
        ) -> Result<()> {
            if let Some(mixin_data) = first_pass.mixins.get(mixin_name) {
                for member in &mixin_data.members {
                    member.webidl_parse(program, first_pass, self_name)?;
                }
            }
            if let Some(mixin_names) = first_pass.includes.get(mixin_name) {
                for mixin_name in mixin_names {
                    parse(program, first_pass, self_name, mixin_name)?;
                }
            }
            Ok(())
        }

        for member in &self.members.body {
            member.webidl_parse(program, first_pass, self.identifier.0)?;
        }

        parse(program, first_pass, self.identifier.0, self.identifier.0)?;

        Ok(())
    }
}

impl<'src> WebidlParse<'src, ()> for weedle::PartialInterfaceDefinition<'src> {
    fn webidl_parse(
        &'src self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if first_pass
            .interfaces
            .get(self.identifier.0)
            .map(|interface_data| interface_data.partial)
            .unwrap_or(true) {
            info!(
                "Partial interface {} missing non-partial interface",
                self.identifier.0
            );
        }

        for member in &self.members.body {
            member.webidl_parse(program, first_pass, self.identifier.0)?;
        }

        Ok(())
    }
}

impl<'src> WebidlParse<'src, &'src weedle::InterfaceDefinition<'src>> for ExtendedAttribute<'src> {
    fn webidl_parse(
        &'src self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        interface: &'src weedle::InterfaceDefinition<'src>,
    ) -> Result<()> {
        let mut add_constructor = |arguments: &[Argument], class: &str| {
            let (overloaded, same_argument_names) = first_pass.get_operation_overloading(
                arguments,
                &::first_pass::OperationId::Constructor,
                interface.identifier.0,
                false,
            );

            let self_ty = ident_ty(rust_ident(camel_case_ident(interface.identifier.0).as_str()));

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

            for import_function in first_pass.create_function(
                "new",
                overloaded,
                same_argument_names,
                &match first_pass.convert_arguments(arguments) {
                    None => return,
                    Some(arguments) => arguments
                },
                IdlType::Interface(interface.identifier.0),
                kind,
                structural,
                throws,
                None,
            ) {
                program.imports.push(wrap_import_function(import_function));
            }
        };

        match self {
            ExtendedAttribute::ArgList(list)
                if list.identifier.0 == "Constructor" =>
            {
                add_constructor(&list.args.body.list, interface.identifier.0)
            }
            ExtendedAttribute::NoArgs(other) if (other.0).0 == "Constructor" => {
                add_constructor(&[], interface.identifier.0)
            }
            ExtendedAttribute::NamedArgList(list)
                if list.lhs_identifier.0 == "NamedConstructor" =>
            {
                add_constructor(&list.args.body.list, list.rhs_identifier.0)
            }

            // these appear to be mapping to gecko preferences, seems like we
            // can safely ignore
            ExtendedAttribute::Ident(id) if id.lhs_identifier.0 == "Pref" => {}

            // looks to be a gecko-specific attribute to tie WebIDL back to C++
            // functions perhaps
            ExtendedAttribute::Ident(id) if id.lhs_identifier.0 == "Func" => {}
            ExtendedAttribute::Ident(id) if id.lhs_identifier.0 == "JSImplementation" => {}
            ExtendedAttribute::Ident(id) if id.lhs_identifier.0 == "HeaderFile" => {}

            // Not actually mentioned in the spec and presumably a hint to
            // Gecko's JS engine? Unsure, but seems like it doesn't matter to us
            ExtendedAttribute::NoArgs(id)
                if (id.0).0 == "ProbablyShortLivingWrapper" => {}

            // Indicates something about enumerable properties, we're not too
            // interested in it
            // https://heycam.github.io/webidl/#LegacyUnenumerableNamedProperties
            ExtendedAttribute::NoArgs(id)
                if (id.0).0 == "LegacyUnenumerableNamedProperties" => {}

            // Indicates where objects are defined (web workers and such), we
            // may later want to use this for cfgs but for now we ignore it.
            // https://heycam.github.io/webidl/#Exposed
            ExtendedAttribute::Ident(id) if id.lhs_identifier.0 == "Exposed" => {}
            ExtendedAttribute::IdentList(id) if id.identifier.0 == "Exposed" => {}

            // We handle this with the "structural" attribute elsewhere
            ExtendedAttribute::IdentList(id) if id.identifier.0 == "Global" => {}

            // Seems like it's safe to ignore for now, just telling us where a
            // binding appears
            // https://heycam.github.io/webidl/#SecureContext
            ExtendedAttribute::NoArgs(id) if (id.0).0 == "SecureContext" => {}

            // We handle this elsewhere
            ExtendedAttribute::NoArgs(id) if (id.0).0 == "Unforgeable" => {}

            // Looks like this attribute just says that we can't call the
            // constructor
            // https://html.spec.whatwg.org/multipage/dom.html#htmlconstructor
            ExtendedAttribute::NoArgs(id) if (id.0).0 == "HTMLConstructor" => {}

            ExtendedAttribute::ArgList(_)
            | ExtendedAttribute::Ident(_)
            | ExtendedAttribute::IdentList(_)
            | ExtendedAttribute::NamedArgList(_)
            | ExtendedAttribute::NoArgs(_) => {
                warn!("Unsupported WebIDL extended attribute: {:?}", self);
            }
        }

        Ok(())
    }
}

impl<'src> WebidlParse<'src, &'src str> for weedle::interface::InterfaceMember<'src> {
    fn webidl_parse(
        &'src self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        self_name: &'src str,
    ) -> Result<()> {
        use weedle::interface::InterfaceMember::*;

        match self {
            Attribute(attr) => {
                attr.webidl_parse(program, first_pass, self_name)
            }
            Operation(op) => {
                op.webidl_parse(program, first_pass, self_name)
            }
            Const(const_) => {
                const_.webidl_parse(program, first_pass, self_name)
            }
            Iterable(iterable) => {
                iterable.webidl_parse(program, first_pass, self_name)
            }
            // TODO
            Maplike(_) => {
                warn!("Unsupported WebIDL Maplike interface member: {:?}", self);
                Ok(())
            }
            Stringifier(_) => {
                warn!("Unsupported WebIDL Stringifier interface member: {:?}", self);
                Ok(())
            }
            Setlike(_) => {
                warn!("Unsupported WebIDL Setlike interface member: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a, 'src> WebidlParse<'src, &'a str> for weedle::mixin::MixinMember<'src> {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        self_name: &'a str,
    ) -> Result<()> {
        match self {
            weedle::mixin::MixinMember::Attribute(attr) => {
                attr.webidl_parse(program, first_pass, self_name)
            }
            weedle::mixin::MixinMember::Operation(op) => {
                op.webidl_parse(program, first_pass, self_name)
            }
            weedle::mixin::MixinMember::Const(const_) => {
                const_.webidl_parse(program, first_pass, self_name)
            }
            // TODO
            weedle::mixin::MixinMember::Stringifier(_) => {
                warn!("Unsupported WebIDL stringifier mixin member: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'src> WebidlParse<'src, &'src str> for weedle::interface::AttributeInterfaceMember<'src> {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        self_name: &'src str,
    ) -> Result<()> {
        member_attribute(
            program,
            first_pass,
            self_name,
            &self.attributes,
            self.modifier,
            self.readonly.is_some(),
            &self.type_,
            self.identifier.0,
        )
    }
}

impl<'src> WebidlParse<'src, &'src str> for weedle::mixin::AttributeMixinMember<'src> {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        self_name: &'src str,
    ) -> Result<()> {
        member_attribute(
            program,
            first_pass,
            self_name,
            &self.attributes,
            if let Some(s) = self.stringifier {
                Some(weedle::interface::StringifierOrInheritOrStatic::Stringifier(s))
            } else {
                None
            },
            self.readonly.is_some(),
            &self.type_,
            self.identifier.0,
        )
    }
}

fn member_attribute<'src>(
    program: &mut backend::ast::Program,
    first_pass: &FirstPassRecord<'src>,
    self_name: &'src str,
    attrs: &'src Option<ExtendedAttributeList>,
    modifier: Option<weedle::interface::StringifierOrInheritOrStatic>,
    readonly: bool,
    type_: &'src weedle::types::AttributedType<'src>,
    identifier: &'src str,
) -> Result<()> {
    use weedle::interface::StringifierOrInheritOrStatic::*;

    if util::is_chrome_only(attrs) {
        return Ok(());
    }

    let is_static = match modifier {
        Some(Stringifier(_)) => {
            warn!("Unsupported stringifier on type: {:?}", (self_name, identifier));
            return Ok(())
        }
        Some(Inherit(_)) => false,
        Some(Static(_)) => true,
        None => false,
    };

    if type_.attributes.is_some() {
        warn!("Unsupported attributes on type: {:?}", (self_name, identifier));
        return Ok(())
    }

    let is_structural = util::is_structural(attrs);
    let throws = util::throws(attrs);

    for import_function in first_pass.create_getter(
        identifier,
        &type_.type_,
        self_name,
        is_static,
        is_structural,
        throws,
    ) {
        program.imports.push(wrap_import_function(import_function));
    }

    if !readonly {
        for import_function in first_pass.create_setter(
            identifier,
            type_.type_.clone(),
            self_name,
            is_static,
            is_structural,
            throws,
        ) {
            program.imports.push(wrap_import_function(import_function));
        }
    }

    Ok(())
}

impl<'src> WebidlParse<'src, &'src str> for weedle::interface::OperationInterfaceMember<'src> {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        self_name: &'src str,
    ) -> Result<()> {
        member_operation(
            program,
            first_pass,
            self_name,
            &self.attributes,
            self.modifier,
            &self.specials,
            &self.return_type,
            &self.args.body.list,
            &self.identifier,
        )
    }
}

impl<'src> WebidlParse<'src, &'src str> for weedle::mixin::OperationMixinMember<'src> {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        self_name: &'src str,
    ) -> Result<()> {
        member_operation(
            program,
            first_pass,
            self_name,
            &self.attributes,
            None,
            &[],
            &self.return_type,
            &self.args.body.list,
            &self.identifier,
        )
    }
}

fn member_operation<'src>(
    program: &mut backend::ast::Program,
    first_pass: &FirstPassRecord<'src>,
    self_name: &'src str,
    attrs: &'src Option<ExtendedAttributeList>,
    modifier: Option<weedle::interface::StringifierOrStatic>,
    specials: &[weedle::interface::Special],
    return_type: &'src weedle::types::ReturnType<'src>,
    args: &'src [Argument],
    identifier: &Option<weedle::common::Identifier<'src>>,
) -> Result<()> {
    use weedle::interface::StringifierOrStatic::*;
    use weedle::interface::Special;

    if util::is_chrome_only(attrs) {
        return Ok(());
    }

    let is_static = match modifier {
        Some(Stringifier(_)) => {
            warn!("Unsupported stringifier on type: {:?}", (self_name, identifier));
            return Ok(())
        }
        Some(Static(_)) => true,
        None => false,
    };

    let mut operation_ids = vec![
        OperationId::Operation(identifier.map(|s| s.0)),
    ];
    if specials.len() > 1 {
        warn!(
            "Unsupported specials: ({:?}) on type {:?}",
            specials,
            (self_name, identifier),
        );
        return Ok(())
    } else if specials.len() == 1 {
        let id = match specials[0] {
            Special::Getter(weedle::term::Getter) => OperationId::IndexingGetter,
            Special::Setter(weedle::term::Setter) => OperationId::IndexingSetter,
            Special::Deleter(weedle::term::Deleter) => OperationId::IndexingDeleter,
            Special::LegacyCaller(weedle::term::LegacyCaller) => return Ok(()),
        };
        operation_ids.push(id);
    }

    for id in operation_ids {
        let methods = first_pass
            .create_basic_method(
                args,
                id,
                return_type,
                self_name,
                is_static,
                match id {
                    OperationId::IndexingGetter |
                    OperationId::IndexingSetter |
                    OperationId::IndexingDeleter => true,
                    _ => {
                        first_pass
                            .interfaces
                            .get(self_name)
                            .map(|interface_data| interface_data.global)
                            .unwrap_or(false)
                    }
                },
                util::throws(attrs),
            );

        for method in methods {
            program.imports.push(wrap_import_function(method));
        }
    }
    Ok(())
}

impl<'src> WebidlParse<'src, &'src str> for weedle::interface::IterableInterfaceMember<'src> {
    fn webidl_parse(
        &self,
        _program: &mut backend::ast::Program,
        _first_pass: &FirstPassRecord<'src>,
        _self_name: &'src str,
    ) -> Result<()> {
        // if util::is_chrome_only(&self.attributes) {
        //     return Ok(());
        // }

/* TODO
        let throws = util::throws(&self.extended_attributes);
        let return_value = weedle::ReturnType::NonVoid(self.value_type.clone());
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

impl<'src> WebidlParse<'src, ()> for weedle::EnumDefinition<'src> {
    fn webidl_parse(
        &'src self,
        program: &mut backend::ast::Program,
        _: &FirstPassRecord<'src>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        let variants = &self.values.body.list;
        program.imports.push(backend::ast::Import {
            module: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Enum(backend::ast::ImportEnum {
                vis: public(),
                name: rust_ident(camel_case_ident(self.identifier.0).as_str()),
                variants: variants
                    .iter()
                    .map(|v| {
                        if !v.0.is_empty() {
                            rust_ident(camel_case_ident(&v.0).as_str())
                        } else {
                            rust_ident("None")
                        }
                    })
                    .collect(),
                variant_values: variants.iter().map(|v| v.0.to_string()).collect(),
                rust_attrs: vec![parse_quote!(#[derive(Copy, Clone, PartialEq, Debug)])],
            }),
        });

        Ok(())
    }
}

impl<'src> WebidlParse<'src, &'src str> for weedle::interface::ConstMember<'src> {
    fn webidl_parse(
        &'src self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        self_name: &'src str,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        let idl_type = match self.const_type.to_idl_type(first_pass) {
            None => return Ok(()),
            Some(idl_type) => idl_type,
        };

        let ty = match idl_type.to_syn_type(TypePosition::Return) {
            None => {
                warn!(
                    "Cannot convert const type to syn type: {:?} in {:?} on {:?}",
                    idl_type,
                    self,
                    self_name
                );
                return Ok(());
            },
            Some(ty) => ty,
        };

        program.consts.push(backend::ast::Const {
            vis: public(),
            name: rust_ident(self.identifier.0.to_shouty_snake_case().as_str()),
            class: Some(rust_ident(camel_case_ident(&self_name).as_str())),
            ty,
            value: webidl_const_v_to_backend_const_v(&self.const_value),
        });

        Ok(())
    }
}

impl<'src> WebidlParse<'src, ()> for weedle::NamespaceDefinition<'src> {
    fn webidl_parse(
        &'src self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if let Some(attrs) = &self.attributes {
            for attr in &attrs.body.list {
                attr.webidl_parse(program, first_pass, self)?;
            }
        }

        let mut module = backend::ast::Module {
            vis: public(),
            name: rust_ident(self.identifier.0.to_snake_case().as_str()),
            imports: Default::default(),
        };

        if let Some(namespace_data) = first_pass.namespaces.get(&self.identifier.0) {
            for member in &namespace_data.members {
                member.webidl_parse(program, first_pass, (&self.identifier.0, &mut module))?;
            }
        }

        program.modules.push(module);

        Ok(())
    }
}

impl<'src> WebidlParse<'src, &'src weedle::NamespaceDefinition<'src>> for ExtendedAttribute<'src> {
    fn webidl_parse(
        &'src self,
        _program: &mut backend::ast::Program,
        _first_pass: &FirstPassRecord<'src>,
        _namespace: &'src weedle::NamespaceDefinition<'src>,
    ) -> Result<()> {
        warn!("Unsupported WebIDL extended attribute: {:?}", self);
        Ok(())
    }
}

impl<'src> WebidlParse<'src, (&'src str, &'src mut backend::ast::Module)> for weedle::namespace::NamespaceMember<'src> {
    fn webidl_parse(
        &'src self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        (self_name, module): (&'src str, &mut backend::ast::Module),
    ) -> Result<()> {
        match self {
            weedle::namespace::NamespaceMember::Operation(op) => {
                op.webidl_parse(program, first_pass, (self_name, module))?;
            }
            weedle::namespace::NamespaceMember::Attribute(attr) => {
                warn!("Unsupported attribute namespace member: {:?}", attr)
            }
        }
        Ok(())
    }
}

impl<'src> WebidlParse<'src, (&'src str, &'src mut backend::ast::Module)> for weedle::namespace::OperationNamespaceMember<'src> {
    fn webidl_parse(
        &'src self,
        _program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'src>,
        (self_name, module): (&'src str, &mut backend::ast::Module),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        for import_function in first_pass.create_namespace_operation(
            &self.args.body.list,
            self.identifier.as_ref().map(|id| id.0),
            &self.return_type,
            self_name,
            util::throws(&self.attributes)
        ) {
            module.imports.push(
                backend::ast::Import {
                    module: None,
                    js_namespace: Some(raw_ident(self_name)),
                    kind: backend::ast::ImportKind::Function(import_function),
                }
            );
        };

        Ok(())
    }
}
