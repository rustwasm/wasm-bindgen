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
extern crate weedle;

mod first_pass;
mod type_conversion;
mod util;

use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Read};
use std::iter::FromIterator;
use std::path::Path;

use backend::defined::{ImportedTypeDefinitions, RemoveUndefinedImports};
use backend::util::wrap_import_function;
use failure::ResultExt;
use quote::ToTokens;

use first_pass::{FirstPass, FirstPassRecord};
use type_conversion::ToSynType;
use util::{public, rust_const_ident, rust_type_ident, weedle_const_v_to_backend_const_v};

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
    // TODO: fix lifetime issues
    let definitions = weedle::parse(webidl_source).unwrap(); //.context("parsing WebIDL source text")?;

    let mut first_pass_record = Default::default();
    definitions.first_pass(&mut first_pass_record)?;
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
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        context: Ctx,
    ) -> Result<()>;
}

impl WebidlParse<()> for weedle::Definitions {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        for def in &self.definitions {
            def.webidl_parse(program, first_pass, ())?;
        }
        Ok(())
    }
}

impl WebidlParse<()> for weedle::Definition {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        use weedle::Definition::*;

        match self {
            Interface(interface) => interface.webidl_parse(program, first_pass, ())?,
            PartialInterface(partial_interface) => {
                partial_interface.webidl_parse(program, first_pass, ())?
            }
            Enum(enum_) => enum_.webidl_parse(program, first_pass, ())?,
            Typedef(typedef) => typedef.webidl_parse(program, first_pass, ())?,
            IncludesStatement(includes_statement) => {
                includes_statement.webidl_parse(program, first_pass, ())?
            }
            InterfaceMixin(_) | PartialInterfaceMixin(_) => {
                // handled in the first pass
            }
            // TODO
            Callback(..)
            | CallbackInterface(..)
            | Namespace(..)
            | Dictionary(..)
            | PartialDictionary(..)
            | PartialNamespace(..) => warn!("Unsupported WebIDL definition: {:?}", self),
        }
        Ok(())
    }
}

impl WebidlParse<()> for weedle::InterfaceDefinition {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        program.imports.push(backend::ast::Import {
            module: None,
            version: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Type(backend::ast::ImportType {
                vis: public(),
                name: rust_type_ident(&self.identifier),
                attrs: Vec::new(),
            }),
        });

        if let Some(attrs) = &self.attributes {
            for attr in &attrs.body.list {
                attr.webidl_parse(program, first_pass, self)?;
            }
        }

        for member in &self.members.body {
            member.webidl_parse(program, first_pass, &self.identifier)?;
        }

        Ok(())
    }
}

impl WebidlParse<()> for weedle::PartialInterfaceDefinition {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if util::is_no_interface_object(&self.attributes) {
            return Ok(());
        }

        if !first_pass.interfaces.contains(&*self.identifier.name) {
            warn!(
                "Partial interface {} missing non-partial interface",
                self.identifier.name
            );
        }

        for member in &self.members.body {
            member.webidl_parse(program, first_pass, &self.identifier)?;
        }

        Ok(())
    }
}

impl<'a> WebidlParse<()> for weedle::EnumDefinition {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        _: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        let mut variants = Vec::with_capacity(self.values.body.list.len());
        let mut variant_values = Vec::with_capacity(self.values.body.list.len());

        self.values
            .body
            .list
            .iter()
            .map(|x| &x.0)
            .for_each(|variant| {
                let name = {
                    let name = variant.clone();
                    let identifier = weedle::common::Identifier { name };
                    variants.push(rust_type_ident(&identifier));
                    identifier.name
                };
                variant_values.push(name);
            });

        program.imports.push(backend::ast::Import {
            module: None,
            version: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Enum(backend::ast::ImportEnum {
                vis: public(),
                name: rust_type_ident(&self.identifier),
                variants,
                variant_values,
            }),
        });

        Ok(())
    }
}

impl WebidlParse<()> for weedle::TypedefDefinition {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        let dest = rust_type_ident(&self.identifier);
        let src = match self.type_.to_syn_type(first_pass, true) {
            Some(src) => src,
            None => return Ok(()),
        };

        program.type_aliases.push(backend::ast::TypeAlias {
            vis: public(),
            dest,
            src,
        });

        Ok(())
    }
}

impl WebidlParse<()> for weedle::IncludesStatementDefinition {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        (): (),
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        match first_pass.interface_mixins.get(&*self.rhs_identifier.name) {
            Some(interface_mixin) => {
                if let Some(interface_mixin) = interface_mixin.non_partial {
                    interface_mixin.webidl_parse(program, first_pass, &self.lhs_identifier)?;
                }
                for partial_interface_mixin in &interface_mixin.partials {
                    partial_interface_mixin.webidl_parse(
                        program,
                        first_pass,
                        &self.lhs_identifier,
                    )?;
                }
            }
            None => warn!(
                "Tried to include missing mixin {}",
                self.rhs_identifier.name
            ),
        }
        Ok(())
    }
}

impl<'a> WebidlParse<&'a weedle::InterfaceDefinition> for weedle::attribute::ExtendedAttribute {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        interface: &'a weedle::InterfaceDefinition,
    ) -> Result<()> {
        let mut add_constructor = |arguments, class: &weedle::common::Identifier| {
            let kind = backend::ast::ImportFunctionKind::Method {
                class: class.name.to_string(),
                ty: interface.identifier.to_syn_type(first_pass, true)?,
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
                    &weedle::common::Identifier {
                        name: "new".to_string(),
                    },
                    arguments,
                    Some(&interface.identifier),
                    kind,
                    structural,
                    throws,
                )
                .map(wrap_import_function)
                .map(|import| program.imports.push(import))
        };

        use weedle::attribute::{ExtendedAttribute::*, *};

        match self {
            ArgList(ExtendedAttributeArgList { identifier, args })
                if identifier.name == "Constructor" =>
            {
                add_constructor(&*args.body.list, &interface.identifier);
            }
            NamedArgList(ExtendedAttributeNamedArgList {
                lhs_identifier,
                assign: _,
                rhs_identifier,
                args,
            })
                if lhs_identifier.name == "NamedConstructor" =>
            {
                add_constructor(&*args.body.list, rhs_identifier);
            }
            NoArgs(ExtendedAttributeNoArgs { identifier }) if identifier.name == "Constructor" => {
                add_constructor(&[], &interface.identifier);
            }
            ArgList(_) | NamedArgList(_) | IdentList(_) | Ident(_) | NoArgs(_) => {
                warn!("Unsupported WebIDL extended attribute: {:?}", self);
            }
        }

        Ok(())
    }
}

impl<'a> WebidlParse<&'a weedle::common::Identifier> for weedle::interface::InterfaceMember {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_identifier: &'a weedle::common::Identifier,
    ) -> Result<()> {
        use weedle::interface::InterfaceMember::*;

        match self {
            Const(const_) => const_.webidl_parse(program, first_pass, self_identifier),
            Attribute(attribute) => attribute.webidl_parse(program, first_pass, self_identifier),
            Operation(operation) => operation.webidl_parse(program, first_pass, self_identifier),
            Iterable(_) | Maplike(_) | Setlike(_) | Stringifier(_) => {
                warn!("Unsupported WebIDL interface member: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<&'a weedle::common::Identifier> for weedle::InterfaceMixinDefinition {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_identifier: &'a weedle::common::Identifier,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if !first_pass.interfaces.contains(&*self_identifier.name) {
            warn!(
                "Including mixin {} on missing interface {}",
                self.identifier.name, self_identifier.name
            );
        }

        for member in &self.members.body {
            member.webidl_parse(program, first_pass, &self_identifier)?;
        }

        Ok(())
    }
}

impl<'a> WebidlParse<&'a weedle::common::Identifier> for weedle::PartialInterfaceMixinDefinition {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_identifier: &'a weedle::common::Identifier,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if !first_pass.interfaces.contains(&*self_identifier.name) {
            warn!(
                "Including mixin {} on missing interface {}",
                self.identifier.name, self_identifier.name
            );
        }

        for member in &self.members.body {
            member.webidl_parse(program, first_pass, &self_identifier)?;
        }

        Ok(())
    }
}

impl<'a> WebidlParse<&'a weedle::common::Identifier> for weedle::interface::ConstMember {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_identifier: &'a weedle::common::Identifier,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        let ty = match self.const_type.to_syn_type(first_pass, true) {
            Some(ty) => ty,
            None => return Ok(()),
        };

        program.consts.push(backend::ast::Const {
            vis: public(),
            name: rust_const_ident(&self.identifier),
            class: Some(rust_type_ident(&self_identifier)),
            ty,
            value: weedle_const_v_to_backend_const_v(&self.const_value),
        });

        Ok(())
    }
}

impl<'a> WebidlParse<&'a weedle::common::Identifier>
    for weedle::interface::AttributeInterfaceMember
{
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_identifier: &'a weedle::common::Identifier,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        let mut is_static = false;
        if let Some(modifier) = &self.modifier {
            use weedle::interface::StringifierOrInheritOrStatic::*;

            match modifier {
                Static(weedle::term::Static) => is_static = true,
                Stringifier(_) | Inherit(_) => {
                    warn!("Unsupported WebIDL attribute: {:?}", self);
                    return Ok(());
                }
            }
        }

        let is_structural = util::is_structural(&self.attributes);
        let throws = util::throws(&self.attributes);

        first_pass
            .create_getter(
                &self.identifier,
                &self.type_,
                self_identifier,
                is_static,
                is_structural,
                throws,
            )
            .map(wrap_import_function)
            .map(|import| program.imports.push(import));

        if self.readonly.is_none() {
            first_pass
                .create_setter(
                    &self.identifier,
                    &self.type_,
                    self_identifier,
                    is_static,
                    is_structural,
                    throws,
                )
                .map(wrap_import_function)
                .map(|import| program.imports.push(import));
        }

        Ok(())
    }
}

impl<'a> WebidlParse<&'a weedle::common::Identifier>
    for weedle::interface::OperationInterfaceMember
{
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_identifier: &'a weedle::common::Identifier,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        let mut is_static = false;
        if let Some(modifier) = &self.modifier {
            use weedle::interface::StringifierOrStatic::*;

            match modifier {
                Static(weedle::term::Static) => is_static = true,
                Stringifier(_) => {
                    warn!("Unsupported WebIDL operation: {:?}", self);
                    return Ok(());
                }
            }
        }

        if !self.specials.is_empty() {
            warn!("Unsupported WebIDL operation: {:?}", self);
            return Ok(());
        }

        let throws = util::throws(&self.attributes);

        first_pass
            .create_regular_method(
                &self.identifier,
                &self.args.body.list,
                &self.return_type,
                self_identifier,
                is_static,
                throws,
            )
            .map(wrap_import_function)
            .map(|import| program.imports.push(import));

        Ok(())
    }
}

impl<'a> WebidlParse<&'a weedle::common::Identifier> for weedle::mixin::MixinMember {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_identifier: &'a weedle::common::Identifier,
    ) -> Result<()> {
        use weedle::mixin::MixinMember::*;

        match self {
            Const(const_) => const_.webidl_parse(program, first_pass, self_identifier),
            Operation(operation) => operation.webidl_parse(program, first_pass, self_identifier),
            Attribute(attribute) => attribute.webidl_parse(program, first_pass, self_identifier),
            Stringifier(_) => {
                warn!("Unsupported WebIDL mixin member: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'a> WebidlParse<&'a weedle::common::Identifier> for weedle::mixin::OperationMixinMember {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_identifier: &'a weedle::common::Identifier,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if self.stringifier.is_some() {
            warn!("Unsupported WebIDL operation: {:?}", self);
            return Ok(());
        }

        let throws = util::throws(&self.attributes);

        first_pass
            .create_regular_method(
                &self.identifier,
                &self.args.body.list,
                &self.return_type,
                self_identifier,
                false,
                throws,
            )
            .map(wrap_import_function)
            .map(|import| program.imports.push(import));

        Ok(())
    }
}

impl<'a> WebidlParse<&'a weedle::common::Identifier> for weedle::mixin::AttributeMixinMember {
    fn webidl_parse(
        &self,
        program: &mut backend::ast::Program,
        first_pass: &FirstPassRecord<'_>,
        self_identifier: &'a weedle::common::Identifier,
    ) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if self.stringifier.is_some() {
            warn!("Unsupported WebIDL attribute: {:?}", self);
            return Ok(());
        }

        let is_structural = util::is_structural(&self.attributes);
        let throws = util::throws(&self.attributes);

        first_pass
            .create_getter(
                &self.identifier,
                &self.type_,
                self_identifier,
                false,
                is_structural,
                throws,
            )
            .map(wrap_import_function)
            .map(|import| program.imports.push(import));

        if self.readonly.is_none() {
            first_pass
                .create_setter(
                    &self.identifier,
                    &self.type_,
                    self_identifier,
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
