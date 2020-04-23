/*!
# `wasm_bindgen_webidl`

Converts WebIDL into wasm-bindgen's internal AST form, so that bindings can be
emitted for the types and methods described in the WebIDL.
 */

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/wasm-bindgen-webidl/0.2")]

mod constants;
mod first_pass;
mod generator;
mod idl_type;
mod traverse;
mod util;

use crate::first_pass::{CallbackInterfaceData, OperationData};
use crate::first_pass::{FirstPass, FirstPassRecord, InterfaceData, OperationId};
use crate::generator::{
    Dictionary, DictionaryField, Enum, EnumVariant, Function, Interface, InterfaceAttribute,
    InterfaceAttributeKind, InterfaceConst, InterfaceMethod, Namespace,
};
use crate::idl_type::ToIdlType;
use crate::traverse::TraverseType;
use crate::util::{
    camel_case_ident, is_structural, read_dir, shouty_snake_case_ident, snake_case_ident, throws,
    webidl_const_v_to_backend_const_v, TypePosition,
};
use anyhow::Context;
use anyhow::Result;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use sourcefile::SourceFile;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use wasm_bindgen_backend::util::rust_ident;
use weedle::attribute::ExtendedAttributeList;
use weedle::dictionary::DictionaryMember;
use weedle::interface::InterfaceMember;
use weedle::Parse;

/// Options to configure the conversion process
#[derive(Debug)]
pub struct Options {
    /// Whether to generate cfg features or not
    pub features: bool,
}

#[derive(Default)]
struct Program {
    tokens: TokenStream,
    required_features: BTreeSet<String>,
}

impl Program {
    fn to_string(&self) -> Option<String> {
        if self.tokens.is_empty() {
            None
        } else {
            Some(self.tokens.to_string())
        }
    }
}

/// A parse error indicating where parsing failed
#[derive(Debug)]
pub struct WebIDLParseError(pub usize);

impl fmt::Display for WebIDLParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse webidl at byte position {}", self.0)
    }
}

impl std::error::Error for WebIDLParseError {}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ApiStability {
    Stable,
    Unstable,
}

impl ApiStability {
    pub(crate) fn is_unstable(self) -> bool {
        self == Self::Unstable
    }
}

impl Default for ApiStability {
    fn default() -> Self {
        Self::Stable
    }
}

fn parse_source(source: &str) -> Result<Vec<weedle::Definition>> {
    match weedle::Definitions::parse(source) {
        Ok(("", parsed)) => Ok(parsed),

        Ok((remaining, _))
        | Err(weedle::Err::Error((remaining, _)))
        | Err(weedle::Err::Failure((remaining, _))) => {
            Err(WebIDLParseError(source.len() - remaining.len()).into())
        }

        Err(weedle::Err::Incomplete(needed)) => {
            Err(anyhow::anyhow!("needed {:?} more bytes", needed))
        }
    }
}

/// Parse a string of WebIDL source text into a wasm-bindgen AST.
fn parse(
    webidl_source: &str,
    unstable_source: &str,
    options: Options,
) -> Result<BTreeMap<String, Program>> {
    let mut first_pass_record: FirstPassRecord = Default::default();

    let definitions = parse_source(webidl_source)?;
    definitions.first_pass(&mut first_pass_record, ApiStability::Stable)?;

    let unstable_definitions = parse_source(unstable_source)?;
    unstable_definitions.first_pass(&mut first_pass_record, ApiStability::Unstable)?;

    let mut types: BTreeMap<String, Program> = BTreeMap::new();

    for (js_name, e) in first_pass_record.enums.iter() {
        let name = rust_ident(&camel_case_ident(js_name));
        let program = types.entry(name.to_string()).or_default();
        first_pass_record.append_enum(&options, program, name, js_name, e);
    }
    for (js_name, d) in first_pass_record.dictionaries.iter() {
        let name = rust_ident(&camel_case_ident(js_name));
        let program = types.entry(name.to_string()).or_default();
        first_pass_record.append_dictionary(&options, program, name, js_name.to_string(), d);
    }
    for (js_name, n) in first_pass_record.namespaces.iter() {
        let name = rust_ident(&snake_case_ident(js_name));
        let program = types.entry(name.to_string()).or_default();
        first_pass_record.append_ns(&options, program, name, js_name.to_string(), n);
    }
    for (js_name, d) in first_pass_record.interfaces.iter() {
        let name = rust_ident(&camel_case_ident(js_name));
        let program = types.entry(name.to_string()).or_default();
        first_pass_record.append_interface(&options, program, name, js_name.to_string(), d);
    }
    for (js_name, d) in first_pass_record.callback_interfaces.iter() {
        let name = rust_ident(&camel_case_ident(js_name));
        let program = types.entry(name.to_string()).or_default();
        first_pass_record.append_callback_interface(
            &options,
            program,
            name,
            js_name.to_string(),
            d,
        );
    }

    Ok(types)
}

/// Data for a single feature
#[derive(Debug)]
pub struct Feature {
    /// Generated code
    pub code: String,

    /// Required features
    pub required_features: Vec<String>,
}

/// Compile the given WebIDL source text into Rust source text containing
/// `wasm-bindgen` bindings to the things described in the WebIDL.
pub fn compile(
    webidl_source: &str,
    experimental_source: &str,
    options: Options,
) -> Result<BTreeMap<String, Feature>> {
    let ast = parse(webidl_source, experimental_source, options)?;

    let features = ast
        .into_iter()
        .filter_map(|(name, program)| {
            let code = program.to_string()?;
            let required_features = program.required_features.into_iter().collect();
            Some((
                name,
                Feature {
                    required_features,
                    code,
                },
            ))
        })
        .collect();

    Ok(features)
}

impl<'src> FirstPassRecord<'src> {
    fn append_enum(
        &self,
        options: &Options,
        program: &mut Program,
        name: Ident,
        js_name: &str,
        data: &first_pass::EnumData<'src>,
    ) {
        let enum_ = data.definition;
        let unstable = data.stability.is_unstable();

        assert_eq!(js_name, enum_.identifier.0);

        let variants = enum_
            .values
            .body
            .list
            .iter()
            .map(|v| {
                let name = if !v.0.is_empty() {
                    rust_ident(camel_case_ident(&v.0).as_str())
                } else {
                    rust_ident("None")
                };

                let value = v.0.to_string();

                EnumVariant { name, value }
            })
            .collect::<Vec<_>>();

        Enum {
            name,
            variants,
            unstable,
        }
        .generate(options)
        .to_tokens(&mut program.tokens);
    }

    // tons more data for what's going on here at
    // https://www.w3.org/TR/WebIDL-1/#idl-dictionaries
    fn append_dictionary(
        &self,
        options: &Options,
        program: &mut Program,
        name: Ident,
        js_name: String,
        data: &first_pass::DictionaryData<'src>,
    ) {
        let def = match data.definition {
            Some(def) => def,
            None => return,
        };

        assert_eq!(js_name, def.identifier.0);

        let unstable = data.stability.is_unstable();

        let mut fields = Vec::new();

        if !self.append_dictionary_members(&js_name, &mut fields) {
            return;
        }

        Dictionary {
            name,
            js_name,
            fields,
            unstable,
        }
        .generate(options)
        .to_tokens(&mut program.tokens);
    }

    fn append_dictionary_members(&self, dict: &'src str, dst: &mut Vec<DictionaryField>) -> bool {
        let dict_data = &self.dictionaries[&dict];
        let definition = dict_data.definition.unwrap();

        // > The order of the dictionary members on a given dictionary is
        // > such that inherited dictionary members are ordered before
        // > non-inherited members ...
        if let Some(parent) = &definition.inheritance {
            if !self.append_dictionary_members(parent.identifier.0, dst) {
                return false;
            }
        }

        // > ... and the dictionary members on the one dictionary
        // > definition (including any partial dictionary definitions) are
        // > ordered lexicographically by the Unicode codepoints that
        // > comprise their identifiers.
        let start = dst.len();
        let members = definition.members.body.iter();
        let partials = dict_data.partials.iter().flat_map(|d| &d.members.body);
        for member in members.chain(partials) {
            match self.dictionary_field(member) {
                Some(f) => dst.push(f),
                None => {
                    log::warn!(
                        "unsupported dictionary field {:?}",
                        (dict, member.identifier.0),
                    );
                    // If this is required then we can't support the
                    // dictionary at all, but if it's not required we can
                    // avoid generating bindings for the field and keep
                    // going otherwise.
                    if member.required.is_some() {
                        return false;
                    }
                }
            }
        }
        dst[start..].sort_by_key(|f| f.js_name.clone());

        return true;
    }

    fn dictionary_field(&self, field: &'src DictionaryMember<'src>) -> Option<DictionaryField> {
        // use argument position now as we're just binding setters
        let ty = field
            .type_
            .to_idl_type(self)
            .to_syn_type(TypePosition::Argument)
            .unwrap_or(None)?;

        // Slice types aren't supported because they don't implement
        // `Into<JsValue>`
        match ty {
            syn::Type::Reference(ref i) => match &*i.elem {
                syn::Type::Slice(_) => return None,
                _ => (),
            },
            syn::Type::Path(ref path, ..) =>
            // check that our inner don't contains slices either
            {
                for seg in path.path.segments.iter() {
                    if let syn::PathArguments::AngleBracketed(ref arg) = seg.arguments {
                        for elem in &arg.args {
                            if let syn::GenericArgument::Type(syn::Type::Reference(ref i)) = elem {
                                match &*i.elem {
                                    syn::Type::Slice(_) => return None,
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        };

        // Similarly i64/u64 aren't supported because they don't
        // implement `Into<JsValue>`
        let mut any_64bit = false;

        ty.traverse_type(&mut |ident| {
            if !any_64bit {
                if ident == "u64" || ident == "i64" {
                    any_64bit = true;
                }
            }
        });

        if any_64bit {
            return None;
        }

        Some(DictionaryField {
            required: field.required.is_some(),
            name: rust_ident(&snake_case_ident(field.identifier.0)),
            js_name: field.identifier.0.to_string(),
            ty,
        })
    }

    fn append_ns(
        &'src self,
        options: &Options,
        program: &mut Program,
        name: Ident,
        js_name: String,
        ns: &'src first_pass::NamespaceData<'src>,
    ) {
        let mut functions = vec![];

        for (id, data) in ns.operations.iter() {
            self.append_ns_member(&mut functions, &js_name, id, data);
        }

        if !functions.is_empty() {
            Namespace {
                name,
                js_name,
                functions,
            }
            .generate(options)
            .to_tokens(&mut program.tokens);
        }
    }

    fn append_ns_member(
        &self,
        functions: &mut Vec<Function>,
        js_name: &'src str,
        id: &OperationId<'src>,
        data: &OperationData<'src>,
    ) {
        match id {
            OperationId::Operation(Some(_)) => {}
            OperationId::Constructor
            | OperationId::NamedConstructor(_)
            | OperationId::Operation(None)
            | OperationId::IndexingGetter
            | OperationId::IndexingSetter
            | OperationId::IndexingDeleter => {
                log::warn!("Unsupported unnamed operation: on {:?}", js_name);
                return;
            }
        }

        for x in self.create_imports(None, id, data, false) {
            functions.push(Function {
                name: x.name,
                js_name: x.js_name,
                arguments: x.arguments,
                ret_ty: x.ret_ty,
                catch: x.catch,
                variadic: x.variadic,
                unstable: false,
            });
        }
    }

    fn append_const(
        &self,
        consts: &mut Vec<InterfaceConst>,
        member: &'src weedle::interface::ConstMember<'src>,
        unstable: bool,
    ) {
        let idl_type = member.const_type.to_idl_type(self);
        let ty = idl_type.to_syn_type(TypePosition::Return).unwrap().unwrap();

        let js_name = member.identifier.0;
        let name = rust_ident(shouty_snake_case_ident(js_name).as_str());
        let value = webidl_const_v_to_backend_const_v(&member.const_value);

        consts.push(InterfaceConst {
            name,
            js_name: js_name.to_string(),
            ty,
            value,
            unstable,
        });
    }

    fn append_interface(
        &self,
        options: &Options,
        program: &mut Program,
        name: Ident,
        js_name: String,
        data: &InterfaceData<'src>,
    ) {
        let unstable = data.stability.is_unstable();
        let has_interface = data.has_interface;

        let deprecated = data.deprecated.clone();

        let parents = self
            .all_superclasses(&js_name)
            .map(|parent| {
                let ident = rust_ident(&camel_case_ident(&parent));
                program.required_features.insert(parent);
                ident
            })
            .collect::<Vec<_>>();

        let mut consts = vec![];
        let mut attributes = vec![];
        let mut methods = vec![];

        for member in data.consts.iter() {
            self.append_const(&mut consts, member, unstable);
        }

        for member in data.attributes.iter() {
            let unstable = unstable || member.stability.is_unstable();
            let member = member.definition;
            self.member_attribute(
                &mut attributes,
                member.modifier,
                member.readonly.is_some(),
                &member.type_,
                member.identifier.0.to_string(),
                &member.attributes,
                data.definition_attributes,
                unstable,
            );
        }

        for (id, op_data) in data.operations.iter() {
            self.member_operation(&mut methods, data, id, op_data);
        }

        for mixin_data in self.all_mixins(&js_name) {
            for member in &mixin_data.consts {
                self.append_const(&mut consts, member, unstable);
            }

            for member in &mixin_data.attributes {
                let unstable = unstable || member.stability.is_unstable();
                let member = member.definition;
                self.member_attribute(
                    &mut attributes,
                    if let Some(s) = member.stringifier {
                        Some(weedle::interface::StringifierOrInheritOrStatic::Stringifier(s))
                    } else {
                        None
                    },
                    member.readonly.is_some(),
                    &member.type_,
                    member.identifier.0.to_string(),
                    &member.attributes,
                    data.definition_attributes,
                    unstable,
                );
            }

            for (id, op_data) in mixin_data.operations.iter() {
                self.member_operation(&mut methods, data, id, op_data);
            }
        }

        Interface {
            name,
            js_name,
            deprecated,
            has_interface,
            parents,
            consts,
            attributes,
            methods,
            unstable,
        }
        .generate(options)
        .to_tokens(&mut program.tokens);
    }

    fn member_attribute(
        &self,
        attributes: &mut Vec<InterfaceAttribute>,
        modifier: Option<weedle::interface::StringifierOrInheritOrStatic>,
        readonly: bool,
        type_: &'src weedle::types::AttributedType<'src>,
        js_name: String,
        attrs: &'src Option<ExtendedAttributeList<'src>>,
        container_attrs: Option<&'src ExtendedAttributeList<'src>>,
        unstable: bool,
    ) {
        use weedle::interface::StringifierOrInheritOrStatic::*;

        let is_static = match modifier {
            Some(Stringifier(_)) => unreachable!(), // filtered out earlier
            Some(Inherit(_)) => false,
            Some(Static(_)) => true,
            None => false,
        };

        let structural = is_structural(attrs.as_ref(), container_attrs);

        let catch = throws(attrs);

        let ty = type_
            .type_
            .to_idl_type(self)
            .to_syn_type(TypePosition::Return)
            .unwrap_or(None);

        // Skip types which can't be converted
        if let Some(ty) = ty {
            let kind = InterfaceAttributeKind::Getter;
            attributes.push(InterfaceAttribute {
                is_static,
                structural,
                catch,
                ty,
                js_name: js_name.clone(),
                kind,
                unstable,
            });
        }

        if !readonly {
            let ty = type_
                .type_
                .to_idl_type(self)
                .to_syn_type(TypePosition::Argument)
                .unwrap_or(None);

            // Skip types which can't be converted
            if let Some(ty) = ty {
                let kind = InterfaceAttributeKind::Setter;
                attributes.push(InterfaceAttribute {
                    is_static,
                    structural,
                    catch,
                    ty,
                    js_name,
                    kind,
                    unstable,
                });
            }
        }
    }

    fn member_operation(
        &self,
        methods: &mut Vec<InterfaceMethod>,
        data: &InterfaceData<'src>,
        id: &OperationId<'src>,
        op_data: &OperationData<'src>,
    ) {
        let attrs = data.definition_attributes;
        let unstable = data.stability.is_unstable();

        for method in self.create_imports(attrs, id, op_data, unstable) {
            methods.push(method);
        }
    }

    fn append_callback_interface(
        &self,
        options: &Options,
        program: &mut Program,
        name: Ident,
        js_name: String,
        item: &CallbackInterfaceData<'src>,
    ) {
        assert_eq!(js_name, item.definition.identifier.0);

        let mut fields = Vec::new();

        for member in item.definition.members.body.iter() {
            match member {
                InterfaceMember::Operation(op) => {
                    let identifier = match op.identifier {
                        Some(i) => i.0,
                        None => continue,
                    };
                    let pos = TypePosition::Argument;

                    fields.push(DictionaryField {
                        required: false,
                        name: rust_ident(&snake_case_ident(identifier)),
                        js_name: identifier.to_string(),
                        ty: idl_type::IdlType::Callback
                            .to_syn_type(pos)
                            .unwrap()
                            .unwrap(),
                    })
                }
                _ => {
                    log::warn!(
                        "skipping callback interface member on {}",
                        item.definition.identifier.0
                    );
                }
            }
        }

        Dictionary {
            name,
            js_name,
            fields,
            unstable: false,
        }
        .generate(options)
        .to_tokens(&mut program.tokens);
    }
}

/// Generates Rust source code with #[wasm_bindgen] annotations.
///
/// * Reads WebIDL files in `from`
/// * Generates Rust source code in the directory `to`
/// * `options.features` indicates whether everything is gated by features or
///   not
///
/// If features are enabled, returns a string that should be appended to
/// `Cargo.toml` which lists all the known features.
pub fn generate(from: &Path, to: &Path, options: Options) -> Result<String> {
    let generate_features = options.features;

    let source = read_source_from_path(&from.join("enabled"))?;
    let unstable_source = read_source_from_path(&from.join("unstable"))?;

    let features = parse_webidl(generate_features, source, unstable_source)?;

    if to.exists() {
        fs::remove_dir_all(&to).context("Removing features directory")?;
    }

    fs::create_dir_all(&to).context("Creating features directory")?;

    for (name, feature) in features.iter() {
        let out_file_path = to.join(format!("gen_{}.rs", name));

        fs::write(&out_file_path, &feature.code)?;

        rustfmt(&out_file_path, name)?;
    }

    let binding_file = features.keys().map(|name| {
        if generate_features {
            format!("#[cfg(feature = \"{name}\")] #[allow(non_snake_case)] mod gen_{name};\n#[cfg(feature = \"{name}\")] pub use gen_{name}::*;", name = name)
        } else {
            format!("#[allow(non_snake_case)] mod gen_{name};\npub use gen_{name}::*;", name = name)
        }
    }).collect::<Vec<_>>().join("\n\n");

    fs::write(to.join("mod.rs"), binding_file)?;

    rustfmt(&to.join("mod.rs"), "mod")?;

    return if generate_features {
        let features = features
            .iter()
            .map(|(name, feature)| {
                let features = feature
                    .required_features
                    .iter()
                    .map(|x| format!("\"{}\"", x))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} = [{}]", name, features)
            })
            .collect::<Vec<_>>()
            .join("\n");
        Ok(features)
    } else {
        Ok(String::new())
    };

    /// Read all WebIDL files in a directory into a single `SourceFile`
    fn read_source_from_path(dir: &Path) -> Result<SourceFile> {
        let entries = read_dir(dir).context("reading webidls directory")?;
        let mut source = SourceFile::default();
        for path in entries {
            if path.extension() != Some(OsStr::new("webidl")) {
                continue;
            }
            source = source
                .add_file(&path)
                .with_context(|| format!("reading contents of file \"{}\"", path.display()))?;
        }

        Ok(source)
    }

    fn rustfmt(path: &PathBuf, name: &str) -> Result<()> {
        // run rustfmt on the generated file - really handy for debugging
        let result = Command::new("rustfmt")
            .arg("--edition")
            .arg("2018")
            .arg(&path)
            .status()
            .context(format!("rustfmt on file {}", name))?;

        assert!(result.success(), "rustfmt on file {}", name);

        Ok(())
    }

    fn parse_webidl(
        generate_features: bool,
        enabled: SourceFile,
        unstable: SourceFile,
    ) -> Result<BTreeMap<String, Feature>> {
        let options = Options {
            features: generate_features,
        };

        match compile(&enabled.contents, &unstable.contents, options) {
            Ok(features) => Ok(features),
            Err(e) => {
                if let Some(err) = e.downcast_ref::<WebIDLParseError>() {
                    if let Some(pos) = enabled.resolve_offset(err.0) {
                        let ctx = format!(
                            "compiling WebIDL into wasm-bindgen bindings in file \
                             \"{}\", line {} column {}",
                            pos.filename,
                            pos.line + 1,
                            pos.col + 1
                        );
                        return Err(e.context(ctx));
                    } else {
                        return Err(e.context("compiling WebIDL into wasm-bindgen bindings"));
                    }
                }
                return Err(e.context("compiling WebIDL into wasm-bindgen bindings"));
            }
        }
    }
}
