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

use backend::ast;
use backend::TryToTokens;
use backend::defined::{ImportedTypeDefinitions, RemoveUndefinedImports};
use backend::defined::ImportedTypeReferences;
use backend::util::{ident_ty, rust_ident, raw_ident, wrap_import_function};
use failure::ResultExt;
use heck::{ShoutySnakeCase, SnakeCase};
use proc_macro2::{Ident, Span};
use weedle::argument::Argument;
use weedle::attribute::{ExtendedAttributeList};
use weedle::dictionary::DictionaryMember;

use first_pass::{FirstPass, FirstPassRecord, OperationId, InterfaceData};
use first_pass::OperationData2;
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
            });
        }
    };

    let mut first_pass_record = Default::default();
    definitions.first_pass(&mut first_pass_record, ())?;
    let mut program = Default::default();

    for e in first_pass_record.enums.values() {
        first_pass_record.append_enum(&mut program, e);
    }
    for d in first_pass_record.dictionaries.values() {
        first_pass_record.append_dictionary(&mut program, d);
    }
    for (name, n) in first_pass_record.namespaces.iter() {
        first_pass_record.append_ns(&mut program, name, n);
    }
    for (name, d) in first_pass_record.interfaces.iter() {
        first_pass_record.append_interface(&mut program, name, d);
    }

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
    // Iteratively prune all entries from the AST which reference undefined
    // fields. Each pass may remove definitions of types and so we need to
    // reexecute this pass to see if we need to keep removing types until we
    // reach a steady state.
    let builtin = BTreeSet::from_iter(
        vec![
            "str", "char", "bool", "JsValue", "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64",
            "usize", "isize", "f32", "f64", "Result", "String", "Vec", "Option",
            "ArrayBuffer", "Object", "Promise",
        ].into_iter()
            .map(|id| proc_macro2::Ident::new(id, proc_macro2::Span::call_site())),
    );
    loop {
        let mut defined = builtin.clone();
        ast.imported_type_definitions(&mut |id| {
            defined.insert(id.clone());
        });
        if !ast.remove_undefined_imports(&|id| defined.contains(id)) {
            break
        }
    }

    let mut tokens = proc_macro2::TokenStream::new();
    if let Err(e) = ast.try_to_tokens(&mut tokens) {
        e.panic();
    }
    tokens.to_string()
}

impl<'src> FirstPassRecord<'src> {
    fn append_enum(
        &self,
        program: &mut backend::ast::Program,
        enum_: &'src weedle::EnumDefinition<'src>
    ) {
        let variants = &enum_.values.body.list;
        program.imports.push(backend::ast::Import {
            module: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Enum(backend::ast::ImportEnum {
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
                rust_attrs: vec![parse_quote!(#[derive(Copy, Clone, PartialEq, Debug)])],
            }),
        });
    }

    // tons more data for what's going on here at
    // https://www.w3.org/TR/WebIDL-1/#idl-dictionaries
    fn append_dictionary(
        &self,
        program: &mut backend::ast::Program,
        data: &first_pass::DictionaryData<'src>,
    ) {
        let def = match data.definition {
            Some(def) => def,
            None => return,
        };
        let mut fields = Vec::new();
        if !self.append_dictionary_members(def.identifier.0, &mut fields) {
            return
        }

        program.dictionaries.push(ast::Dictionary {
            name: rust_ident(&camel_case_ident(def.identifier.0)),
            fields,
        });
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
                return false
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
                    warn!(
                        "unsupported dictionary field {:?}",
                        (dict, member.identifier.0),
                    );
                    // If this is required then we can't support the
                    // dictionary at all, but if it's not required we can
                    // avoid generating bindings for the field and keep
                    // going otherwise.
                    if member.required.is_some() {
                        return false
                    }
                }
            }
        }
        // Note that this sort isn't *quite* right in that it is sorting
        // based on snake case instead of the original casing which could
        // produce inconsistent results, but should work well enough for
        // now!
        dst[start..].sort_by_key(|f| f.name.clone());

        return true
    }

    fn dictionary_field(
        &self,
        field: &'src DictionaryMember<'src>,
    ) -> Option<ast::DictionaryField> {
        // use argument position now as we're just binding setters
        let ty = field.type_.to_idl_type(self)?.to_syn_type(TypePosition::Argument)?;

        // Slice types aren't supported because they don't implement
        // `Into<JsValue>`
        if let syn::Type::Reference(ty) = &ty {
            match &*ty.elem {
                syn::Type::Slice(_) => return None,
                _ => {}
            }
        }

        // Similarly i64/u64 aren't supported because they don't
        // implement `Into<JsValue>`
        let mut any_64bit = false;
        ty.imported_type_references(&mut |i| {
            any_64bit = any_64bit || i == "u64" || i == "i64";
        });
        if any_64bit {
            return None
        }

        Some(ast::DictionaryField {
            required: field.required.is_some(),
            name: rust_ident(&field.identifier.0.to_snake_case()),
            ty,
        })
    }

    fn append_ns(
        &'src self,
        program: &mut backend::ast::Program,
        name: &'src str,
        ns: &'src first_pass::NamespaceData<'src>,
    ) {
        let mut module = backend::ast::Module {
            vis: public(),
            name: rust_ident(name.to_snake_case().as_str()),
            imports: Default::default(),
        };

        for (id, data) in ns.operations2.iter() {
            self.append_ns_member(&mut module, name, id, data);
        }

        program.modules.push(module);
    }

    fn append_ns_member(
        &self,
        module: &mut backend::ast::Module,
        self_name: &'src str,
        id: &OperationId<'src>,
        data: &OperationData2<'src>,
    ) {
        let name = match id {
            OperationId::Operation(Some(name)) => name,
            OperationId::Constructor(_) |
            OperationId::Operation(None) |
            OperationId::IndexingGetter |
            OperationId::IndexingSetter |
            OperationId::IndexingDeleter => {
                warn!("Unsupported unnamed operation: on {:?}", self_name);
                return
            }
        };
        let doc_comment = format!(
            "The `{}.{}()` function\n\n{}",
            self_name,
            name,
            mdn_doc(self_name, Some(&name))
        );

        let kind = backend::ast::ImportFunctionKind::Normal;
        for mut import_function in self.create_imports(kind, id, data) {
            import_function.doc_comment = Some(doc_comment.clone());
            module.imports.push(
                backend::ast::Import {
                    module: None,
                    js_namespace: Some(raw_ident(self_name)),
                    kind: backend::ast::ImportKind::Function(import_function),
                }
            );
        }
    }

    fn append_const(
        &self,
        program: &mut backend::ast::Program,
        self_name: &'src str,
        member: &'src weedle::interface::ConstMember<'src>,
    ) {
        let idl_type = match member.const_type.to_idl_type(self) {
            Some(idl_type) => idl_type,
            None => return,
        };

        let ty = match idl_type.to_syn_type(TypePosition::Return) {
            Some(ty) => ty,
            None => {
                warn!(
                    "Cannot convert const type to syn type: {:?} in {:?} on {:?}",
                    idl_type,
                    member,
                    self_name
                );
                return
            },
        };

        program.consts.push(backend::ast::Const {
            vis: public(),
            name: rust_ident(member.identifier.0.to_shouty_snake_case().as_str()),
            class: Some(rust_ident(camel_case_ident(&self_name).as_str())),
            ty,
            value: webidl_const_v_to_backend_const_v(&member.const_value),
        });
    }

    fn append_interface(
        &self,
        program: &mut backend::ast::Program,
        name: &'src str,
        data: &InterfaceData<'src>,
    ) {
        let doc_comment = Some(format!(
            "The `{}` object\n\n{}",
            name,
            mdn_doc(name, None),
        ));

        program.imports.push(backend::ast::Import {
            module: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Type(backend::ast::ImportType {
                vis: public(),
                rust_name: rust_ident(camel_case_ident(name).as_str()),
                js_name: name.to_string(),
                attrs: Vec::new(),
                doc_comment,
                instanceof_shim: format!("__widl_instanceof_{}", name),
                extends: self.all_superclasses(name)
                    .map(|name| Ident::new(&name, Span::call_site()))
                    .collect(),
            }),
        });

        // for (ctor_name, args) in data.constructors.iter() {
        //     self.append_constructor(program, name, ctor_name, args);
        // }
        for (id, op_data) in data.operations2.iter() {
            // if let OperationId::Constructor = id {
            //     continue // TODO
            // }
            self.member_operation2(program, name, data, id, op_data);
        }
        for member in data.consts.iter() {
            self.append_const(program, name, member);
        }
        for member in data.attributes.iter() {
            self.member_attribute(
                program,
                name,
                &member.attributes,
                member.modifier,
                member.readonly.is_some(),
                &member.type_,
                member.identifier.0,
            );
        }

        for mixin_data in self.all_mixins(name) {
            for (id, op_data) in mixin_data.operations2.iter() {
                self.member_operation2(program, name, data, id, op_data);
            }
            for member in &mixin_data.consts {
                self.append_const(program, name, member);
            }
            for member in &mixin_data.attributes {
                self.member_attribute(
                    program,
                    name,
                    &member.attributes,
                    if let Some(s) = member.stringifier {
                        Some(weedle::interface::StringifierOrInheritOrStatic::Stringifier(s))
                    } else {
                        None
                    },
                    member.readonly.is_some(),
                    &member.type_,
                    member.identifier.0,
                );
            }
        }
    }

    // fn append_constructor(
    //     &self,
    //     program: &mut backend::ast::Program,
    //     iface_name: &'src str,
    //     ctor_name: &'src str,
    //     args: &[Argument<'src>],
    // ) {
    //     let (overloaded, same_argument_names) = self.get_operation_overloading(
    //         args,
    //         &::first_pass::OperationId::Constructor,
    //         iface_name,
    //         false,
    //     );
    //
    //     let self_ty = ident_ty(rust_ident(camel_case_ident(iface_name).as_str()));
    //
    //     let kind = backend::ast::ImportFunctionKind::Method {
    //         class: ctor_name.to_string(),
    //         ty: self_ty.clone(),
    //         kind: backend::ast::MethodKind::Constructor,
    //     };
    //
    //     let structural = false;
    //
    //     // Constructors aren't annotated with `[Throws]` extended attributes
    //     // (how could they be, since they themselves are extended
    //     // attributes?) so we must conservatively assume that they can
    //     // always throw.
    //     //
    //     // From https://heycam.github.io/webidl/#Constructor (emphasis
    //     // mine):
    //     //
    //     // > The prose definition of a constructor must either return an IDL
    //     // > value of a type corresponding to the interface the
    //     // > `[Constructor]` extended attribute appears on, **or throw an
    //     // > exception**.
    //     let throws = true;
    //
    //     for import_function in self.create_function(
    //         "new",
    //         overloaded,
    //         same_argument_names,
    //         &match self.convert_arguments(args) {
    //             Some(arguments) => arguments,
    //             None => return,
    //         },
    //         IdlType::Interface(iface_name),
    //         kind,
    //         structural,
    //         throws,
    //         None,
    //     ) {
    //         program.imports.push(wrap_import_function(import_function));
    //     }
    // }

    fn member_attribute(
        &self,
        program: &mut backend::ast::Program,
        self_name: &'src str,
        attrs: &'src Option<ExtendedAttributeList>,
        modifier: Option<weedle::interface::StringifierOrInheritOrStatic>,
        readonly: bool,
        type_: &'src weedle::types::AttributedType<'src>,
        identifier: &'src str,
    ) {
        use weedle::interface::StringifierOrInheritOrStatic::*;

        let is_static = match modifier {
            Some(Stringifier(_)) => unimplemented!(), // filtered out earlier
            Some(Inherit(_)) => false,
            Some(Static(_)) => true,
            None => false,
        };

        let is_structural = util::is_structural(attrs);
        let throws = util::throws(attrs);
        let global = self
            .interfaces
            .get(self_name)
            .map(|interface_data| interface_data.global)
            .unwrap_or(false);

        for import_function in self.create_getter(
            identifier,
            &type_.type_,
            self_name,
            is_static,
            is_structural,
            throws,
            global,
        ) {
            program.imports.push(wrap_import_function(import_function));
        }

        if !readonly {
            for import_function in self.create_setter(
                identifier,
                type_.type_.clone(),
                self_name,
                is_static,
                is_structural,
                throws,
                global,
            ) {
                program.imports.push(wrap_import_function(import_function));
            }
        }
    }

    fn member_operation2(
        &self,
        program: &mut backend::ast::Program,
        self_name: &str,
        data: &InterfaceData<'src>,
        id: &OperationId<'src>,
        op_data: &OperationData2<'src>,
    ) {
        let import_function_kind = |opkind| {
            self.import_function_kind(self_name, data.global, op_data.is_static, opkind)
        };
        let kind = match id {
            OperationId::Constructor(ctor_name) => {
                let self_ty = ident_ty(rust_ident(&camel_case_ident(self_name)));
                backend::ast::ImportFunctionKind::Method {
                    class: ctor_name.0.to_string(),
                    ty: self_ty.clone(),
                    kind: backend::ast::MethodKind::Constructor,
                }
            }
            OperationId::Operation(_) => {
                import_function_kind(backend::ast::OperationKind::Regular)
            }
            OperationId::IndexingGetter => {
                import_function_kind(backend::ast::OperationKind::IndexingGetter)
            }
            OperationId::IndexingSetter => {
                import_function_kind(backend::ast::OperationKind::IndexingSetter)
            }
            OperationId::IndexingDeleter => {
                import_function_kind(backend::ast::OperationKind::IndexingDeleter)
            }
        };
        for method in self.create_imports(kind, id, op_data) {
            program.imports.push(wrap_import_function(method));
        }
    }
}
