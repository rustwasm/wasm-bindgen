use std::iter::FromIterator;
use std::collections::BTreeMap;

use backend;
use backend::util::{ident_ty, leading_colon_path_ty, raw_ident, rust_ident};
use heck::{CamelCase, SnakeCase};
use proc_macro2::Ident;
use syn;
use weedle;
use weedle::attribute::{ExtendedAttributeList, ExtendedAttribute};
use weedle::argument::Argument;
use weedle::literal::{ConstValue, FloatLit, IntegerLit};

use first_pass::{self, FirstPassRecord};
use idl_type::{IdlType, ToIdlType, flatten};

/// Take a type and create an immutable shared reference to that type.
pub(crate) fn shared_ref(ty: syn::Type) -> syn::Type {
    syn::TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: Box::new(ty),
    }.into()
}

/// Fix camelcase of identifiers like HTMLBRElement
pub fn camel_case_ident(identifier: &str) -> String {
  identifier.replace("HTML", "HTML_").to_camel_case()
}

// Returns a link to MDN
pub fn mdn_doc(class: &str, method: Option<&str>) -> String {
    let mut link = format!("https://developer.mozilla.org/en-US/docs/Web/API/{}", class);
    if let Some(method) = method {
        link.push_str(&format!("/{}", method));
    }
    format!("[Documentation]({})", link).into()
}

// Array type is borrowed for arguments (`&[T]`) and owned for return value (`Vec<T>`).
pub(crate) fn array(base_ty: &str, pos: TypePosition) -> syn::Type {
    match pos {
        TypePosition::Argument => {
            shared_ref(slice_ty(ident_ty(raw_ident(base_ty))))
        }
        TypePosition::Return => {
            vec_ty(ident_ty(raw_ident(base_ty)))
        }
    }
}

/// Map a webidl const value to the correct wasm-bindgen const value
pub fn webidl_const_v_to_backend_const_v(v: &ConstValue) -> backend::ast::ConstValue {
    use std::f64::{NEG_INFINITY, INFINITY, NAN};
    use backend::ast;

    match *v {
        ConstValue::Boolean(b) => ast::ConstValue::BooleanLiteral(b.0),
        ConstValue::Float(FloatLit::NegInfinity(_)) => {
            ast::ConstValue::FloatLiteral(NEG_INFINITY)
        }
        ConstValue::Float(FloatLit::Infinity(_)) => {
            ast::ConstValue::FloatLiteral(INFINITY)
        }
        ConstValue::Float(FloatLit::NaN(_)) => {
            ast::ConstValue::FloatLiteral(NAN)
        }
        ConstValue::Float(FloatLit::Value(s)) => {
            ast::ConstValue::FloatLiteral(s.0.parse().unwrap())
        }
        ConstValue::Integer(lit) => {
            let mklit = |orig_text: &str, base: u32, offset: usize| {
                let (negative, text) = if orig_text.starts_with("-") {
                    (true, &orig_text[1..])
                } else {
                    (false, orig_text)
                };
                if text == "0" {
                    return ast::ConstValue::SignedIntegerLiteral(0)
                }
                let text = &text[offset..];
                let n = u64::from_str_radix(text, base)
                    .unwrap_or_else(|_| panic!("literal too big: {}", orig_text));
                if negative {
                    let n = if n > (i64::min_value() as u64).wrapping_neg() {
                        panic!("literal too big: {}", orig_text)
                    } else {
                        n.wrapping_neg() as i64
                    };
                    ast::ConstValue::SignedIntegerLiteral(n)
                } else {
                    ast::ConstValue::UnsignedIntegerLiteral(n)
                }
            };
            match lit {
                IntegerLit::Hex(h) => mklit(h.0, 16, 2), // leading 0x
                IntegerLit::Oct(h) => mklit(h.0, 8, 1), // leading 0
                IntegerLit::Dec(h) => mklit(h.0, 10, 0),
            }
        }
        ConstValue::Null(_) => ast::ConstValue::Null,
    }
}

/// From `ident` and `Ty`, create `ident: Ty` for use in e.g. `fn(ident: Ty)`.
fn simple_fn_arg(ident: Ident, ty: syn::Type) -> syn::ArgCaptured {
    syn::ArgCaptured {
        pat: syn::Pat::Ident(syn::PatIdent {
            by_ref: None,
            mutability: None,
            ident,
            subpat: None,
        }),
        colon_token: Default::default(),
        ty,
    }
}

/// Create `()`.
fn unit_ty() -> syn::Type {
    syn::Type::Tuple(syn::TypeTuple {
        paren_token: Default::default(),
        elems: syn::punctuated::Punctuated::new(),
    })
}

/// From `T` create `Result<T, ::wasm_bindgen::JsValue>`.
fn result_ty(t: syn::Type) -> syn::Type {
    let js_value = leading_colon_path_ty(vec![rust_ident("wasm_bindgen"), rust_ident("JsValue")]);

    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![
            syn::GenericArgument::Type(t),
            syn::GenericArgument::Type(js_value),
        ]),
        gt_token: Default::default(),
    });

    let ident = raw_ident("Result");
    let seg = syn::PathSegment { ident, arguments };
    let path: syn::Path = seg.into();
    let ty = syn::TypePath { qself: None, path };
    ty.into()
}

/// From `T` create `[T]`.
pub(crate) fn slice_ty(t: syn::Type) -> syn::Type {
    syn::TypeSlice {
        bracket_token: Default::default(),
        elem: Box::new(t),
    }.into()
}

/// From `T` create `Vec<T>`.
pub(crate) fn vec_ty(t: syn::Type) -> syn::Type {
    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![
            syn::GenericArgument::Type(t),
        ]),
        gt_token: Default::default(),
    });

    let ident = raw_ident("Vec");
    let seg = syn::PathSegment { ident, arguments };
    let path: syn::Path = seg.into();
    let ty = syn::TypePath { qself: None, path };
    ty.into()
}

/// From `T` create `Option<T>`
pub(crate) fn option_ty(t: syn::Type) -> syn::Type {
    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![syn::GenericArgument::Type(t)]),
        gt_token: Default::default(),
    });

    let ident = raw_ident("Option");
    let seg = syn::PathSegment { ident, arguments };
    let path: syn::Path = seg.into();
    let ty = syn::TypePath { qself: None, path };
    ty.into()
}

/// Possible positions for a type in a function signature.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypePosition {
    Argument,
    Return,
}

impl<'src> FirstPassRecord<'src> {
    /// Create a wasm-bindgen function, if possible.
    pub fn create_function(
        &self,
        name: &str,
        overloaded: bool,
        same_argument_names: bool,
        arguments: &[(&str, IdlType<'src>, bool)],
        ret: IdlType<'src>,
        kind: backend::ast::ImportFunctionKind,
        structural: bool,
        catch: bool,
        doc_comment: Option<String>,
    ) -> Vec<backend::ast::ImportFunction> {
        let rust_name = if overloaded && !arguments.is_empty() {
            let mut argument_type_names = String::new();
            for argument in arguments {
                if argument_type_names.len() > 0 {
                    argument_type_names.push_str("_and_");
                }
                if same_argument_names {
                    argument.1.push_type_name(&mut argument_type_names);
                } else {
                    argument_type_names.push_str(&argument.0.to_snake_case());
                }
            }
            if name == "new" {
                "with_".to_owned() + &argument_type_names
            } else {
                name.to_snake_case() + "_with_" + &argument_type_names
            }
        } else {
            name.to_snake_case()
        };

        let ret = match ret {
            IdlType::Void => None,
            ret @ _ => {
                match ret.to_syn_type(TypePosition::Return) {
                    None => {
                        warn!(
                            "Unsupported return type: {:?} on {:?}",
                            ret,
                            rust_name
                        );
                        return Vec::new();
                    },
                    Some(ret) => Some(ret),
                }
            },
        };

        let js_ret = ret.clone();

        let ret = if catch {
            Some(ret.map_or_else(|| result_ty(unit_ty()), result_ty))
        } else {
            ret
        };

        let converted_arguments = arguments
            .iter()
            .cloned()
            .map(|(_name, idl_type, optional)| (idl_type, optional))
            .collect::<Vec<_>>();
        let possibilities = flatten(&converted_arguments);
        let mut arguments_count_multiple = BTreeMap::new();
        for idl_types in &possibilities {
            arguments_count_multiple
                .entry(idl_types.len())
                .and_modify(|variants_count| { *variants_count = true; })
                .or_insert(false);
        }
        let mut import_functions = Vec::new();
        'outer: for idl_types in &possibilities {
            let rust_name = if possibilities.len() > 1 {
                let mut rust_name = rust_name.clone();
                let mut first = true;
                let iter = arguments.iter().zip(idl_types).enumerate();
                for (i, ((argument_name, _, _), idl_type)) in iter {
                    if possibilities.iter().all(|p| p.get(i) == Some(idl_type)) {
                        continue
                    }
                    if first {
                        rust_name.push_str("_with_");
                        first = false;
                    } else {
                        rust_name.push_str("_and_");
                    }
                    if arguments_count_multiple[&idl_types.len()] {
                        idl_type.push_type_name(&mut rust_name);
                    } else {
                        rust_name.push_str(&argument_name.to_snake_case());
                    }
                }
                rust_name
            } else {
                rust_name.clone()
            };
            let rust_name = rust_ident(&rust_name);
            let shim = {
                let ns = match kind {
                    backend::ast::ImportFunctionKind::ScopedMethod { .. } |
                    backend::ast::ImportFunctionKind::Normal => "",
                    backend::ast::ImportFunctionKind::Method { ref class, .. } => class,
                };

                raw_ident(&format!("__widl_f_{}_{}", rust_name, ns))
            };

            let mut args_captured = if let &backend::ast::ImportFunctionKind::Method {
                ref ty,
                kind: backend::ast::MethodKind::Operation(
                    backend::ast::Operation {
                        is_static: false, ..
                    }
                ),
                ..
            } = &kind {
                let mut res = Vec::with_capacity(idl_types.len() + 1);
                res.push(simple_fn_arg(raw_ident("self_"), shared_ref(ty.clone())));
                res
            } else {
                Vec::with_capacity(idl_types.len())
            };
            for ((argument_name, _, _), idl_type) in arguments.iter().zip(idl_types) {
                let syn_type = if let Some(syn_type) = idl_type.to_syn_type(TypePosition::Argument) {
                    syn_type
                } else {
                    warn!(
                        "Unsupported argument type: {:?} on {:?}",
                        idl_type,
                        rust_name
                    );
                    continue 'outer;
                };
                let argument_name = rust_ident(&argument_name.to_snake_case());
                args_captured.push(simple_fn_arg(argument_name, syn_type));
            }

            import_functions.push(backend::ast::ImportFunction {
                function: backend::ast::Function {
                    name: name.to_string(),
                    arguments: args_captured,
                    ret: ret.clone(),
                    rust_attrs: vec![],
                    rust_vis: public(),
                },
                rust_name,
                js_ret: js_ret.clone(),
                catch,
                variadic: false,
                structural,
                kind: kind.clone(),
                shim,
                doc_comment: doc_comment.clone(),
            })
        }
        import_functions
    }

    /// Convert arguments to ones suitable crating function
    pub(crate) fn convert_arguments(
        &self,
        arguments: &[weedle::argument::Argument<'src>],
    ) -> Option<Vec<(&str, IdlType<'src>, bool)>> {
        let mut converted_arguments = Vec::with_capacity(arguments.len());
        for argument in arguments {
            let name = match argument {
                Argument::Single(single) => single.identifier.0,
                Argument::Variadic(variadic) => variadic.identifier.0,
            };
            let ty = match argument {
                Argument::Single(single) => &single.type_.type_,
                Argument::Variadic(variadic) => &variadic.type_,
            };
            let idl_type = ty.to_idl_type(self)?;
            let optional = match argument {
                Argument::Single(single) => single.optional.is_some(),
                Argument::Variadic(_variadic) => false,
            };
            converted_arguments.push((name, idl_type, optional));
        }
        Some(converted_arguments)
    }

    /// Create a wasm-bindgen method, if possible.
    pub fn create_basic_method(
        &self,
        arguments: &[Argument],
        operation_id: first_pass::OperationId,
        return_type: &weedle::types::ReturnType,
        self_name: &str,
        is_static: bool,
        structural: bool,
        catch: bool,
        global: bool,
    ) -> Vec<backend::ast::ImportFunction> {
        let (overloaded, same_argument_names) = self.get_operation_overloading(
            arguments,
            &operation_id,
            self_name,
            false,
        );

        let name = match &operation_id {
            first_pass::OperationId::Constructor => panic!("constructors are unsupported"),
            first_pass::OperationId::Operation(name) => match name {
                None => {
                    warn!("Unsupported unnamed operation: {:?}", operation_id);
                    return Vec::new();
                }
                Some(name) => name,
            },
            first_pass::OperationId::IndexingGetter => "get",
            first_pass::OperationId::IndexingSetter => "set",
            first_pass::OperationId::IndexingDeleter => "delete",
        };
        let operation_kind = match &operation_id {
            first_pass::OperationId::Constructor => panic!("constructors are unsupported"),
            first_pass::OperationId::Operation(_) => backend::ast::OperationKind::Regular,
            first_pass::OperationId::IndexingGetter => backend::ast::OperationKind::IndexingGetter,
            first_pass::OperationId::IndexingSetter => backend::ast::OperationKind::IndexingSetter,
            first_pass::OperationId::IndexingDeleter => backend::ast::OperationKind::IndexingDeleter,
        };
        let operation = backend::ast::Operation { is_static, kind: operation_kind };
        let ty = ident_ty(rust_ident(camel_case_ident(&self_name).as_str()));
        let kind = if global {
            backend::ast::ImportFunctionKind::ScopedMethod {
                ty,
                operation,
            }
        } else {
            backend::ast::ImportFunctionKind::Method {
                class: self_name.to_string(),
                ty,
                kind: backend::ast::MethodKind::Operation(operation),
            }
        };

        let ret = match return_type.to_idl_type(self) {
            None => return Vec::new(),
            Some(idl_type) => idl_type,
        };

        let doc_comment = match &operation_id {
            first_pass::OperationId::Constructor => panic!("constructors are unsupported"),
            first_pass::OperationId::Operation(_) => Some(
                format!(
                    "The `{}()` method\n\n{}",
                    name,
                    mdn_doc(self_name, Some(&name))
                )
            ),
            first_pass::OperationId::IndexingGetter => Some("The indexing getter\n\n".to_string()),
            first_pass::OperationId::IndexingSetter => Some("The indexing setter\n\n".to_string()),
            first_pass::OperationId::IndexingDeleter => Some("The indexing deleter\n\n".to_string()),
        };

        let arguments = match self.convert_arguments(arguments) {
            None => return Vec::new(),
            Some(arguments) => arguments
        };

        self.create_function(
            &name,
            overloaded,
            same_argument_names,
            &arguments,
            ret,
            kind,
            structural,
            catch,
            doc_comment,
        )
    }

    /// Whether operation is overloaded and
    /// whether there overloads with same argument names for given argument types
    pub fn get_operation_overloading(
        &self,
        arguments: &[Argument],
        operation_id: &first_pass::OperationId,
        self_name: &str,
        namespace: bool,
    ) -> (bool, bool) {
        fn get_operation_data<'src>(
            record: &'src FirstPassRecord,
            operation_id: &'src ::first_pass::OperationId,
            self_name: &str,
            mixin_name: &str,
        ) -> Option<&'src ::first_pass::OperationData<'src>> {
            if let Some(mixin_data) = record.mixins.get(mixin_name) {
                if let Some(operation_data) = mixin_data.operations.get(operation_id) {
                    return Some(operation_data);
                }
            }
            if let Some(mixin_names) = record.includes.get(mixin_name) {
                for mixin_name in mixin_names {
                    if let Some(operation_data) = get_operation_data(record, operation_id, self_name, mixin_name) {
                        return Some(operation_data);
                    }
                }
            }
            None
        }

        let operation_data = if !namespace {
            self
                .interfaces
                .get(self_name)
                .and_then(|interface_data| interface_data.operations.get(operation_id))
                .unwrap_or_else(||
                    get_operation_data(self, operation_id, self_name, self_name)
                        .expect(&format!("not found operation {:?} in interface {}", operation_id, self_name))
                )
        } else {
            self
                .namespaces
                .get(self_name)
                .and_then(|interface_data| interface_data.operations.get(operation_id))
                .expect(&format!("not found operation {:?} in namespace {}", operation_id, self_name))
        };

        let mut names = Vec::with_capacity(arguments.len());
        for argument in arguments {
            match argument {
                Argument::Single(single) => names.push(single.identifier.0),
                Argument::Variadic(variadic) => names.push(variadic.identifier.0),
            }
        }
        (
            operation_data.overloaded,
            *operation_data
                .argument_names_same
                .get(&names)
                .unwrap_or(&false)
        )
    }

    /// Create a wasm-bindgen operation (free function with no `self` type), if possible.
    pub fn create_namespace_operation(
        &self,
        arguments: &[weedle::argument::Argument],
        operation_name: Option<&str>,
        return_type: &weedle::types::ReturnType,
        self_name: &str,
        catch: bool,
    ) -> Vec<backend::ast::ImportFunction> {
        let (overloaded, same_argument_names) = self.get_operation_overloading(
            arguments,
            &first_pass::OperationId::Operation(operation_name),
            self_name,
            true,
        );

        let name = match operation_name {
            Some(name) => name.to_string(),
            None => {
                warn!("Unsupported unnamed operation: on {:?}", self_name);
                return Vec::new();
            }
        };

        let ret = match return_type.to_idl_type(self) {
            None => return Vec::new(),
            Some(idl_type) => idl_type,
        };

        let doc_comment = Some(
            format!(
                "The `{}.{}()` function\n\n{}",
                self_name,
                name,
                mdn_doc(self_name, Some(&name))
            )
        );

        let arguments = match self.convert_arguments(arguments) {
            None => return Vec::new(),
            Some(arguments) => arguments
        };

        self.create_function(
            &name,
            overloaded,
            same_argument_names,
            &arguments,
            ret,
            backend::ast::ImportFunctionKind::Normal,
            false,
            catch,
            doc_comment,
        )
    }

    /// Create a wasm-bindgen getter method, if possible.
    pub fn create_getter(
        &self,
        name: &str,
        ty: &weedle::types::Type,
        self_name: &str,
        is_static: bool,
        is_structural: bool,
        catch: bool,
        global: bool,
    ) -> Vec<backend::ast::ImportFunction> {
        let ret = match ty.to_idl_type(self) {
            None => return Vec::new(),
            Some(idl_type) => idl_type,
        };
        let operation = backend::ast::Operation {
            is_static,
            kind: backend::ast::OperationKind::Getter(Some(raw_ident(name))),
        };
        let ty = ident_ty(rust_ident(camel_case_ident(&self_name).as_str()));

        let kind = if global {
            backend::ast::ImportFunctionKind::ScopedMethod {
                ty,
                operation,
            }
        } else {
            backend::ast::ImportFunctionKind::Method {
                class: self_name.to_string(),
                ty,
                kind: backend::ast::MethodKind::Operation(operation),
            }
        };
        let doc_comment = Some(format!("The `{}` getter\n\n{}", name, mdn_doc(self_name, Some(name))));

        self.create_function(name, false, false, &[], ret, kind, is_structural, catch, doc_comment)
    }

    /// Create a wasm-bindgen setter method, if possible.
    pub fn create_setter(
        &self,
        name: &str,
        field_ty: weedle::types::Type,
        self_name: &str,
        is_static: bool,
        is_structural: bool,
        catch: bool,
        global: bool,
    ) -> Vec<backend::ast::ImportFunction> {
        let operation = backend::ast::Operation {
            is_static,
            kind: backend::ast::OperationKind::Setter(Some(raw_ident(name))),
        };
        let ty = ident_ty(rust_ident(camel_case_ident(&self_name).as_str()));

        let kind = if global {
            backend::ast::ImportFunctionKind::ScopedMethod {
                ty,
                operation,
            }
        } else {
            backend::ast::ImportFunctionKind::Method {
                class: self_name.to_string(),
                ty,
                kind: backend::ast::MethodKind::Operation(operation),
            }
        };
        let doc_comment = Some(format!("The `{}` setter\n\n{}", name, mdn_doc(self_name, Some(name))));

        self.create_function(
            &format!("set_{}", name),
            false,
            false,
            &[(
                name,
                match field_ty.to_idl_type(self) {
                    None => return Vec::new(),
                    Some(idl_type) => idl_type,
                },
                false,
            )],
            IdlType::Void,
            kind,
            is_structural,
            catch,
            doc_comment,
        )
    }
}

/// Search for an attribute by name in some webidl object's attributes.
fn has_named_attribute(list: &Option<ExtendedAttributeList>, attribute: &str) -> bool {
    let list = match list {
        Some(list) => list,
        None => return false,
    };
    list.body.list.iter().any(|attr| match attr {
        ExtendedAttribute::NoArgs(name) => (name.0).0 == attribute,
        _ => false,
    })
}

/// ChromeOnly is for things that are only exposed to privileged code in Firefox.
pub fn is_chrome_only(ext_attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(ext_attrs, "ChromeOnly")
}

/// Whether a webidl object is marked as a no interface object.
pub fn is_no_interface_object(ext_attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(ext_attrs, "NoInterfaceObject")
}

/// Whether a webidl object is marked as structural.
pub fn is_structural(attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(attrs, "Unforgeable")
}

/// Whether a webidl object is marked as throwing.
pub fn throws(attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(attrs, "Throws")
}

/// Create a syn `pub` token
pub fn public() -> syn::Visibility {
    syn::Visibility::Public(syn::VisPublic {
        pub_token: Default::default(),
    })
}
