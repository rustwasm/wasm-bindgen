/*!
# `wasm_bindgen_webidl`

Converts WebIDL into wasm-bindgen's internal AST form, so that bindings can be
emitted for the types and methods described in the WebIDL.
 */

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/wasm-bindgen-webidl/0.2")]

mod error;
mod first_pass;
mod idl_type;
mod util;

use crate::first_pass::{CallbackInterfaceData, OperationData};
use crate::first_pass::{FirstPass, FirstPassRecord, InterfaceData, OperationId};
use crate::idl_type::ToIdlType;
use crate::util::{
    camel_case_ident, mdn_doc, public, shouty_snake_case_ident, snake_case_ident,
    webidl_const_v_to_backend_const_v, TypePosition,
};
use failure::format_err;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fmt::Display;
use std::fs;
use std::iter::FromIterator;
use wasm_bindgen_backend::ast;
use wasm_bindgen_backend::defined::ImportedTypeReferences;
use wasm_bindgen_backend::defined::{ImportedTypeDefinitions, RemoveUndefinedImports};
use wasm_bindgen_backend::util::{ident_ty, raw_ident, rust_ident, wrap_import_function};
use wasm_bindgen_backend::TryToTokens;
use weedle::attribute::ExtendedAttributeList;
use weedle::dictionary::DictionaryMember;
use weedle::interface::InterfaceMember;

pub use crate::error::{Error, ErrorKind, Result};

struct Program {
    main: ast::Program,
    submodules: Vec<(String, ast::Program)>,
}

/// Parse a string of WebIDL source text into a wasm-bindgen AST.
fn parse(webidl_source: &str, allowed_types: Option<&[&str]>) -> Result<Program> {
    let definitions = match weedle::parse(webidl_source) {
        Ok(def) => def,
        Err(e) => {
            return Err(match &e {
                weedle::Err::Incomplete(needed) => format_err!("needed {:?} more bytes", needed)
                    .context(ErrorKind::ParsingWebIDLSource)
                    .into(),
                weedle::Err::Error(cx) | weedle::Err::Failure(cx) => {
                    // Note that #[allow] here is a workaround for Geal/nom#843
                    // because the `Context` type here comes from `nom` and if
                    // something else in our crate graph enables the
                    // `verbose-errors` feature then we need to still compiled
                    // against the changed enum definition.
                    #[allow(unreachable_patterns)]
                    let remaining = match cx {
                        weedle::Context::Code(remaining, _) => remaining.len(),
                        _ => 0,
                    };
                    let pos = webidl_source.len() - remaining;
                    format_err!("failed to parse WebIDL")
                        .context(ErrorKind::ParsingWebIDLSourcePos(pos))
                        .into()
                }
            });
        }
    };

    let mut first_pass_record: FirstPassRecord = Default::default();
    first_pass_record.builtin_idents = builtin_idents();
    first_pass_record.immutable_slice_whitelist = immutable_slice_whitelist();

    definitions.first_pass(&mut first_pass_record, ())?;
    let mut program = Default::default();
    let mut submodules = Vec::new();

    let allowed_types = allowed_types.map(|list| list.iter().cloned().collect::<HashSet<_>>());
    let filter = |name: &str| match &allowed_types {
        Some(set) => set.contains(name),
        None => true,
    };

    for (name, e) in first_pass_record.enums.iter() {
        if filter(&camel_case_ident(name)) {
            first_pass_record.append_enum(&mut program, e);
        }
    }
    for (name, d) in first_pass_record.dictionaries.iter() {
        if filter(&camel_case_ident(name)) {
            first_pass_record.append_dictionary(&mut program, d);
        }
    }
    for (name, n) in first_pass_record.namespaces.iter() {
        if filter(&snake_case_ident(name)) {
            let prog = first_pass_record.append_ns(name, n);
            submodules.push((snake_case_ident(name).to_string(), prog));
        }
    }
    for (name, d) in first_pass_record.interfaces.iter() {
        if filter(&camel_case_ident(name)) {
            first_pass_record.append_interface(&mut program, name, d);
        }
    }
    for (name, d) in first_pass_record.callback_interfaces.iter() {
        if filter(&camel_case_ident(name)) {
            first_pass_record.append_callback_interface(&mut program, d);
        }
    }

    // Prune out `extends` annotations that aren't defined as these shouldn't
    // prevent the type from being usable entirely. They're just there for
    // `AsRef` and such implementations.
    for import in program.imports.iter_mut() {
        if let ast::ImportKind::Type(t) = &mut import.kind {
            t.extends.retain(|n| {
                let ident = &n.segments.last().unwrap().ident;
                first_pass_record.builtin_idents.contains(ident) || filter(&ident.to_string())
            });
        }
    }

    Ok(Program {
        main: program,
        submodules: submodules,
    })
}

/// Compile the given WebIDL source text into Rust source text containing
/// `wasm-bindgen` bindings to the things described in the WebIDL.
pub fn compile(webidl_source: &str, allowed_types: Option<&[&str]>) -> Result<String> {
    let ast = parse(webidl_source, allowed_types)?;
    Ok(compile_ast(ast))
}

fn builtin_idents() -> BTreeSet<Ident> {
    BTreeSet::from_iter(
        vec![
            "str",
            "char",
            "bool",
            "JsValue",
            "u8",
            "i8",
            "u16",
            "i16",
            "u32",
            "i32",
            "u64",
            "i64",
            "usize",
            "isize",
            "f32",
            "f64",
            "Result",
            "String",
            "Vec",
            "Option",
            "Array",
            "ArrayBuffer",
            "Object",
            "Promise",
            "Function",
            "Clamped",
        ]
        .into_iter()
        .map(|id| proc_macro2::Ident::new(id, proc_macro2::Span::call_site())),
    )
}

fn immutable_slice_whitelist() -> BTreeSet<&'static str> {
    BTreeSet::from_iter(vec![
        // WebGlRenderingContext, WebGl2RenderingContext
        "uniform1fv",
        "uniform2fv",
        "uniform3fv",
        "uniform4fv",
        "uniform1iv",
        "uniform2iv",
        "uniform3iv",
        "uniform4iv",
        "uniformMatrix2fv",
        "uniformMatrix3fv",
        "uniformMatrix4fv",
        "vertexAttrib1fv",
        "vertexAttrib2fv",
        "vertexAttrib3fv",
        "vertexAttrib4fv",
        "bufferData",
        "bufferSubData",
        "texImage2D",
        "texSubImage2D",
        "compressedTexImage2D",
        // WebGl2RenderingContext
        "uniform1uiv",
        "uniform2uiv",
        "uniform3uiv",
        "uniform4uiv",
        "texImage3D",
        "texSubImage3D",
        "compressedTexImage3D",
        "clearBufferfv",
        "clearBufferiv",
        "clearBufferuiv",
        // TODO: Add another type's functions here. Leave a comment header with the type name
    ])
}

/// Run codegen on the AST to generate rust code.
fn compile_ast(mut ast: Program) -> String {
    // Iteratively prune all entries from the AST which reference undefined
    // fields. Each pass may remove definitions of types and so we need to
    // reexecute this pass to see if we need to keep removing types until we
    // reach a steady state.
    let builtin = builtin_idents();
    let mut all_definitions = BTreeSet::new();
    let track = env::var_os("__WASM_BINDGEN_DUMP_FEATURES");
    loop {
        let mut defined = builtin.clone();
        {
            let mut cb = |id: &Ident| {
                defined.insert(id.clone());
                if track.is_some() {
                    all_definitions.insert(id.clone());
                }
            };
            ast.main.imported_type_definitions(&mut cb);
            for (name, m) in ast.submodules.iter() {
                cb(&Ident::new(name, Span::call_site()));
                m.imported_type_references(&mut cb);
            }
        }
        let changed = ast
            .main
            .remove_undefined_imports(&|id| defined.contains(id))
            || ast
                .submodules
                .iter_mut()
                .any(|(_, m)| m.remove_undefined_imports(&|id| defined.contains(id)));
        if !changed {
            break;
        }
    }
    if let Some(path) = track {
        let contents = all_definitions
            .into_iter()
            .filter(|def| !builtin.contains(def))
            .map(|s| format!("{} = []", s))
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(path, contents).unwrap();
    }

    let mut tokens = proc_macro2::TokenStream::new();
    if let Err(e) = ast.main.try_to_tokens(&mut tokens) {
        e.panic();
    }
    for (name, m) in ast.submodules.iter() {
        let mut m_tokens = proc_macro2::TokenStream::new();
        if let Err(e) = m.try_to_tokens(&mut m_tokens) {
            e.panic();
        }

        let name = Ident::new(name, Span::call_site());

        (quote! {
            pub mod #name { #m_tokens }
        })
        .to_tokens(&mut tokens);
    }
    tokens.to_string()
}

impl<'src> FirstPassRecord<'src> {
    fn append_enum(&self, program: &mut ast::Program, enum_: &'src weedle::EnumDefinition<'src>) {
        let variants = &enum_.values.body.list;
        program.imports.push(ast::Import {
            module: ast::ImportModule::None,
            js_namespace: None,
            kind: ast::ImportKind::Enum(ast::ImportEnum {
                vis: public(),
                name: rust_ident(camel_case_ident(enum_.identifier.0).as_str()),
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
                rust_attrs: vec![syn::parse_quote!(#[derive(Copy, Clone, PartialEq, Debug)])],
            }),
        });
    }

    // tons more data for what's going on here at
    // https://www.w3.org/TR/WebIDL-1/#idl-dictionaries
    fn append_dictionary(
        &self,
        program: &mut ast::Program,
        data: &first_pass::DictionaryData<'src>,
    ) {
        let def = match data.definition {
            Some(def) => def,
            None => return,
        };
        let mut fields = Vec::new();
        if !self.append_dictionary_members(def.identifier.0, &mut fields) {
            return;
        }
        let name = rust_ident(&camel_case_ident(def.identifier.0));
        let extra_feature = name.to_string();
        for field in fields.iter_mut() {
            let mut doc_comment = Some(format!(
                "Configure the `{}` field of this object\n",
                field.js_name,
            ));
            self.append_required_features_doc(&*field, &mut doc_comment, &[&extra_feature]);
            field.doc_comment = doc_comment;
        }

        let mut doc_comment = format!("The `{}` dictionary\n", def.identifier.0);
        if let Some(s) = self.required_doc_string(vec![name.clone()]) {
            doc_comment.push_str(&s);
        }
        let mut dict = ast::Dictionary {
            name,
            fields,
            ctor: true,
            doc_comment: Some(doc_comment),
            ctor_doc_comment: None,
        };
        let mut ctor_doc_comment = Some(format!("Construct a new `{}`\n", def.identifier.0));
        self.append_required_features_doc(&dict, &mut ctor_doc_comment, &[&extra_feature]);
        dict.ctor_doc_comment = ctor_doc_comment;

        program.dictionaries.push(dict);
    }

    fn append_dictionary_members(
        &self,
        dict: &'src str,
        dst: &mut Vec<ast::DictionaryField>,
    ) -> bool {
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

    fn dictionary_field(
        &self,
        field: &'src DictionaryMember<'src>,
    ) -> Option<ast::DictionaryField> {
        // use argument position now as we're just binding setters
        let ty = field
            .type_
            .to_idl_type(self)
            .to_syn_type(TypePosition::Argument)?;

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
        ty.imported_type_references(&mut |i| {
            any_64bit = any_64bit || i == "u64" || i == "i64";
        });
        if any_64bit {
            return None;
        }

        Some(ast::DictionaryField {
            required: field.required.is_some(),
            rust_name: rust_ident(&snake_case_ident(field.identifier.0)),
            js_name: field.identifier.0.to_string(),
            ty,
            doc_comment: None,
        })
    }

    fn append_ns(
        &'src self,
        name: &'src str,
        ns: &'src first_pass::NamespaceData<'src>,
    ) -> ast::Program {
        let mut ret = Default::default();

        for (id, data) in ns.operations.iter() {
            self.append_ns_member(&mut ret, name, id, data);
        }

        return ret;
    }

    fn append_ns_member(
        &self,
        module: &mut ast::Program,
        self_name: &'src str,
        id: &OperationId<'src>,
        data: &OperationData<'src>,
    ) {
        let name = match id {
            OperationId::Operation(Some(name)) => name,
            OperationId::Constructor(_)
            | OperationId::Operation(None)
            | OperationId::IndexingGetter
            | OperationId::IndexingSetter
            | OperationId::IndexingDeleter => {
                log::warn!("Unsupported unnamed operation: on {:?}", self_name);
                return;
            }
        };
        let doc_comment = format!(
            "The `{}.{}()` function\n\n{}",
            self_name,
            name,
            mdn_doc(self_name, Some(&name))
        );

        let kind = ast::ImportFunctionKind::Normal;
        let extra = snake_case_ident(self_name);
        let extra = &[&extra[..]];
        for mut import_function in self.create_imports(None, kind, id, data) {
            let mut doc = Some(doc_comment.clone());
            self.append_required_features_doc(&import_function, &mut doc, extra);
            import_function.doc_comment = doc;
            module.imports.push(ast::Import {
                module: ast::ImportModule::None,
                js_namespace: Some(raw_ident(self_name)),
                kind: ast::ImportKind::Function(import_function),
            });
        }
    }

    fn append_const(
        &self,
        program: &mut ast::Program,
        self_name: &'src str,
        member: &'src weedle::interface::ConstMember<'src>,
    ) {
        let idl_type = member.const_type.to_idl_type(self);
        let ty = match idl_type.to_syn_type(TypePosition::Return) {
            Some(ty) => ty,
            None => {
                log::warn!(
                    "Cannot convert const type to syn type: {:?} in {:?} on {:?}",
                    idl_type,
                    member,
                    self_name
                );
                return;
            }
        };

        program.consts.push(ast::Const {
            vis: public(),
            name: rust_ident(shouty_snake_case_ident(member.identifier.0).as_str()),
            class: Some(rust_ident(camel_case_ident(&self_name).as_str())),
            ty,
            value: webidl_const_v_to_backend_const_v(&member.const_value),
        });
    }

    fn append_interface(
        &self,
        program: &mut ast::Program,
        name: &'src str,
        data: &InterfaceData<'src>,
    ) {
        let mut doc_comment = Some(format!("The `{}` object\n\n{}", name, mdn_doc(name, None),));

        let mut attrs = Vec::new();
        attrs.push(syn::parse_quote!( #[derive(Debug, Clone, PartialEq, Eq)] ));
        self.add_deprecated(data, &mut attrs);
        let mut import_type = ast::ImportType {
            vis: public(),
            rust_name: rust_ident(camel_case_ident(name).as_str()),
            js_name: name.to_string(),
            attrs,
            doc_comment: None,
            instanceof_shim: format!("__widl_instanceof_{}", name),
            is_type_of: if data.has_interface {
                None
            } else {
                Some(syn::parse_quote! { |_| false })
            },
            extends: Vec::new(),
            vendor_prefixes: Vec::new(),
        };

        // whitelist a few names that have known polyfills
        match name {
            "AudioContext" => {
                import_type
                    .vendor_prefixes
                    .push(Ident::new("webkit", Span::call_site()));
            }
            _ => {}
        }
        let extra = camel_case_ident(name);
        let extra = &[&extra[..]];
        self.append_required_features_doc(&import_type, &mut doc_comment, extra);
        import_type.extends = self
            .all_superclasses(name)
            .map(|name| Ident::new(&name, Span::call_site()).into())
            .chain(Some(Ident::new("Object", Span::call_site()).into()))
            .collect();
        import_type.doc_comment = doc_comment;

        program.imports.push(ast::Import {
            module: ast::ImportModule::None,
            js_namespace: None,
            kind: ast::ImportKind::Type(import_type),
        });

        for (id, op_data) in data.operations.iter() {
            self.member_operation(program, name, data, id, op_data);
        }
        for member in data.consts.iter() {
            self.append_const(program, name, member);
        }
        for member in data.attributes.iter() {
            self.member_attribute(
                program,
                name,
                data,
                member.modifier,
                member.readonly.is_some(),
                &member.type_,
                member.identifier.0,
                &member.attributes,
                data.definition_attributes,
            );
        }

        for mixin_data in self.all_mixins(name) {
            for (id, op_data) in mixin_data.operations.iter() {
                self.member_operation(program, name, data, id, op_data);
            }
            for member in &mixin_data.consts {
                self.append_const(program, name, member);
            }
            for member in &mixin_data.attributes {
                self.member_attribute(
                    program,
                    name,
                    data,
                    if let Some(s) = member.stringifier {
                        Some(weedle::interface::StringifierOrInheritOrStatic::Stringifier(s))
                    } else {
                        None
                    },
                    member.readonly.is_some(),
                    &member.type_,
                    member.identifier.0,
                    &member.attributes,
                    data.definition_attributes,
                );
            }
        }
    }

    fn member_attribute(
        &self,
        program: &mut ast::Program,
        self_name: &'src str,
        data: &InterfaceData<'src>,
        modifier: Option<weedle::interface::StringifierOrInheritOrStatic>,
        readonly: bool,
        type_: &'src weedle::types::AttributedType<'src>,
        identifier: &'src str,
        attrs: &'src Option<ExtendedAttributeList<'src>>,
        container_attrs: Option<&'src ExtendedAttributeList<'src>>,
    ) {
        use weedle::interface::StringifierOrInheritOrStatic::*;

        let is_static = match modifier {
            Some(Stringifier(_)) => unreachable!(), // filtered out earlier
            Some(Inherit(_)) => false,
            Some(Static(_)) => true,
            None => false,
        };

        for mut import_function in self.create_getter(
            identifier,
            &type_.type_,
            self_name,
            is_static,
            attrs,
            container_attrs,
        ) {
            let mut doc = import_function.doc_comment.take();
            self.append_required_features_doc(&import_function, &mut doc, &[]);
            import_function.doc_comment = doc;
            program.imports.push(wrap_import_function(import_function));
        }

        if !readonly {
            for mut import_function in self.create_setter(
                identifier,
                &type_.type_,
                self_name,
                is_static,
                attrs,
                container_attrs,
            ) {
                let mut doc = import_function.doc_comment.take();
                self.append_required_features_doc(&import_function, &mut doc, &[]);
                import_function.doc_comment = doc;
                self.add_deprecated(data, &mut import_function.function.rust_attrs);
                program.imports.push(wrap_import_function(import_function));
            }
        }
    }

    fn member_operation(
        &self,
        program: &mut ast::Program,
        self_name: &str,
        data: &InterfaceData<'src>,
        id: &OperationId<'src>,
        op_data: &OperationData<'src>,
    ) {
        let import_function_kind =
            |opkind| self.import_function_kind(self_name, op_data.is_static, opkind);
        let kind = match id {
            OperationId::Constructor(ctor_name) => {
                let self_ty = ident_ty(rust_ident(&camel_case_ident(self_name)));
                ast::ImportFunctionKind::Method {
                    class: ctor_name.0.to_string(),
                    ty: self_ty.clone(),
                    kind: ast::MethodKind::Constructor,
                }
            }
            OperationId::Operation(_) => import_function_kind(ast::OperationKind::Regular),
            OperationId::IndexingGetter => import_function_kind(ast::OperationKind::IndexingGetter),
            OperationId::IndexingSetter => import_function_kind(ast::OperationKind::IndexingSetter),
            OperationId::IndexingDeleter => {
                import_function_kind(ast::OperationKind::IndexingDeleter)
            }
        };
        let doc = match id {
            OperationId::Operation(None) => Some(String::new()),
            OperationId::Constructor(_) => Some(format!(
                "The `new {}(..)` constructor, creating a new \
                 instance of `{0}`\n\n{}",
                self_name,
                mdn_doc(self_name, Some(self_name))
            )),
            OperationId::Operation(Some(name)) => Some(format!(
                "The `{}()` method\n\n{}",
                name,
                mdn_doc(self_name, Some(name))
            )),
            OperationId::IndexingGetter => Some(format!("The indexing getter\n\n")),
            OperationId::IndexingSetter => Some(format!("The indexing setter\n\n")),
            OperationId::IndexingDeleter => Some(format!("The indexing deleter\n\n")),
        };
        let attrs = data.definition_attributes;
        for mut method in self.create_imports(attrs, kind, id, op_data) {
            let mut doc = doc.clone();
            self.append_required_features_doc(&method, &mut doc, &[]);
            method.doc_comment = doc;
            self.add_deprecated(data, &mut method.function.rust_attrs);
            program.imports.push(wrap_import_function(method));
        }
    }

    fn add_deprecated(&self, data: &InterfaceData<'src>, dst: &mut Vec<syn::Attribute>) {
        let msg = match &data.deprecated {
            Some(s) => s,
            None => return,
        };
        dst.push(syn::parse_quote!( #[deprecated(note = #msg)] ));
    }

    fn append_required_features_doc(
        &self,
        item: impl ImportedTypeReferences,
        doc: &mut Option<String>,
        extra: &[&str],
    ) {
        let doc = match doc {
            Some(doc) => doc,
            None => return,
        };
        let mut required = extra
            .iter()
            .map(|s| Ident::new(s, Span::call_site()))
            .collect::<BTreeSet<_>>();
        item.imported_type_references(&mut |f| {
            if !self.builtin_idents.contains(f) {
                required.insert(f.clone());
            }
        });
        if required.len() == 0 {
            return;
        }
        if let Some(extra) = self.required_doc_string(required) {
            doc.push_str(&extra);
        }
    }

    fn required_doc_string<T: Display>(
        &self,
        features: impl IntoIterator<Item = T>,
    ) -> Option<String> {
        let features = features.into_iter().collect::<Vec<_>>();
        if features.len() == 0 {
            return None;
        }
        let list = features
            .iter()
            .map(|ident| format!("`{}`", ident))
            .collect::<Vec<_>>()
            .join(", ");
        Some(format!(
            "\n\n*This API requires the following crate features \
             to be activated: {}*",
            list,
        ))
    }

    fn append_callback_interface(
        &self,
        program: &mut ast::Program,
        item: &CallbackInterfaceData<'src>,
    ) {
        let mut fields = Vec::new();
        for member in item.definition.members.body.iter() {
            match member {
                InterfaceMember::Operation(op) => {
                    let identifier = match op.identifier {
                        Some(i) => i.0,
                        None => continue,
                    };
                    let pos = TypePosition::Argument;
                    fields.push(ast::DictionaryField {
                        required: false,
                        rust_name: rust_ident(&snake_case_ident(identifier)),
                        js_name: identifier.to_string(),
                        ty: idl_type::IdlType::Callback.to_syn_type(pos).unwrap(),
                        doc_comment: None,
                    });
                }
                _ => {
                    log::warn!(
                        "skipping callback interface member on {}",
                        item.definition.identifier.0
                    );
                }
            }
        }

        program.dictionaries.push(ast::Dictionary {
            name: rust_ident(&camel_case_ident(item.definition.identifier.0)),
            fields,
            ctor: true,
            doc_comment: None,
            ctor_doc_comment: None,
        });
    }
}
