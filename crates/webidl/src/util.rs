use std::iter::FromIterator;
use std::ptr;

use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use proc_macro2::{Ident, Span};
use syn;
use wasm_bindgen_backend::ast;
use wasm_bindgen_backend::util::{ident_ty, leading_colon_path_ty, raw_ident, rust_ident};
use weedle;
use weedle::attribute::{ExtendedAttribute, ExtendedAttributeList, IdentifierOrString};
use weedle::literal::{ConstValue, FloatLit, IntegerLit};

use crate::first_pass::{FirstPassRecord, OperationData, OperationId, Signature};
use crate::idl_type::{IdlType, ToIdlType};

/// For variadic operations an overload with a `js_sys::Array` argument is generated alongside with
/// `operation_name_0`, `operation_name_1`, `operation_name_2`, ..., `operation_name_n` overloads
/// which have the count of arguments for passing values to the variadic argument
/// in their names, where `n` is this constant.
const MAX_VARIADIC_ARGUMENTS_COUNT: usize = 7;

/// Take a type and create an immutable shared reference to that type.
pub(crate) fn shared_ref(ty: syn::Type, mutable: bool) -> syn::Type {
    syn::TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: if mutable {
            Some(syn::token::Mut::default())
        } else {
            None
        },
        elem: Box::new(ty),
    }
    .into()
}

/// Fix case of identifiers like `HTMLBRElement` or `texImage2D`
fn fix_ident(identifier: &str) -> String {
    identifier
        .replace("HTML", "HTML_")
        .replace("1D", "_1d")
        .replace("2D", "_2d")
        .replace("3D", "_3d")
}

/// Convert an identifier to camel case
pub fn camel_case_ident(identifier: &str) -> String {
    fix_ident(identifier).to_camel_case()
}

/// Convert an identifier to shouty snake case
pub fn shouty_snake_case_ident(identifier: &str) -> String {
    fix_ident(identifier).to_shouty_snake_case()
}

/// Convert an identifier to snake case
pub fn snake_case_ident(identifier: &str) -> String {
    fix_ident(identifier).to_snake_case()
}

// Returns a link to MDN
pub fn mdn_doc(class: &str, method: Option<&str>) -> String {
    let mut link = format!("https://developer.mozilla.org/en-US/docs/Web/API/{}", class);
    if let Some(method) = method {
        link.push_str(&format!("/{}", method));
    }
    format!("[MDN Documentation]({})", link).into()
}

// Array type is borrowed for arguments (`&mut [T]` or `&[T]`) and owned for return value (`Vec<T>`).
pub(crate) fn array(base_ty: &str, pos: TypePosition, immutable: bool) -> syn::Type {
    match pos {
        TypePosition::Argument => {
            shared_ref(
                slice_ty(ident_ty(raw_ident(base_ty))),
                /*mutable =*/ !immutable,
            )
        }
        TypePosition::Return => vec_ty(ident_ty(raw_ident(base_ty))),
    }
}

/// Map a webidl const value to the correct wasm-bindgen const value
pub fn webidl_const_v_to_backend_const_v(v: &ConstValue) -> ast::ConstValue {
    use std::f64::{INFINITY, NAN, NEG_INFINITY};

    match *v {
        ConstValue::Boolean(b) => ast::ConstValue::BooleanLiteral(b.0),
        ConstValue::Float(FloatLit::NegInfinity(_)) => ast::ConstValue::FloatLiteral(NEG_INFINITY),
        ConstValue::Float(FloatLit::Infinity(_)) => ast::ConstValue::FloatLiteral(INFINITY),
        ConstValue::Float(FloatLit::NaN(_)) => ast::ConstValue::FloatLiteral(NAN),
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
                    return ast::ConstValue::SignedIntegerLiteral(0);
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
                IntegerLit::Oct(h) => mklit(h.0, 8, 1),  // leading 0
                IntegerLit::Dec(h) => mklit(h.0, 10, 0),
            }
        }
        ConstValue::Null(_) => ast::ConstValue::Null,
    }
}

/// From `ident` and `Ty`, create `ident: Ty` for use in e.g. `fn(ident: Ty)`.
fn simple_fn_arg(ident: Ident, ty: syn::Type) -> syn::PatType {
    syn::PatType {
        pat: Box::new(syn::Pat::Ident(syn::PatIdent {
            attrs: Vec::new(),
            by_ref: None,
            ident,
            mutability: None,
            subpat: None,
        })),
        colon_token: Default::default(),
        ty: Box::new(ty),
        attrs: Vec::new(),
    }
}

/// Create `()`.
fn unit_ty() -> syn::Type {
    syn::Type::Tuple(syn::TypeTuple {
        paren_token: Default::default(),
        elems: syn::punctuated::Punctuated::new(),
    })
}

/// From `T` create `Result<T, wasm_bindgen::JsValue>`.
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
    }
    .into()
}

/// From `T` create `Vec<T>`.
pub(crate) fn vec_ty(t: syn::Type) -> syn::Type {
    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![syn::GenericArgument::Type(t)]),
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
    pub fn create_one_function<'a>(
        &self,
        js_name: &str,
        rust_name: &str,
        idl_arguments: impl Iterator<Item = (&'a str, &'a IdlType<'src>)>,
        ret: &IdlType<'src>,
        kind: ast::ImportFunctionKind,
        structural: bool,
        catch: bool,
        variadic: bool,
        doc_comment: Option<String>,
    ) -> Option<ast::ImportFunction>
    where
        'src: 'a,
    {
        // Convert all of the arguments from their IDL type to a `syn` type,
        // ready to pass to the backend.
        //
        // Note that for non-static methods we add a `&self` type placeholder,
        // but this type isn't actually used so it's just here for show mostly.
        let mut arguments = if let &ast::ImportFunctionKind::Method {
            ref ty,
            kind:
                ast::MethodKind::Operation(ast::Operation {
                    is_static: false, ..
                }),
            ..
        } = &kind
        {
            let mut res = Vec::with_capacity(idl_arguments.size_hint().0 + 1);
            res.push(simple_fn_arg(
                raw_ident("self_"),
                shared_ref(ty.clone(), false),
            ));
            res
        } else {
            Vec::with_capacity(idl_arguments.size_hint().0)
        };
        let idl_arguments: Vec<_> = idl_arguments.collect();
        let arguments_count = idl_arguments.len();
        for (i, (argument_name, idl_type)) in idl_arguments.into_iter().enumerate() {
            let syn_type = match idl_type.to_syn_type(TypePosition::Argument) {
                Some(t) => t,
                None => {
                    log::warn!(
                        "Unsupported argument type: {:?} on {:?}",
                        idl_type,
                        rust_name
                    );
                    return None;
                }
            };
            let syn_type = if variadic && i == arguments_count - 1 {
                let path = vec![rust_ident("js_sys"), rust_ident("Array")];
                shared_ref(leading_colon_path_ty(path), false)
            } else {
                syn_type
            };
            let argument_name = rust_ident(&argument_name.to_snake_case());
            arguments.push(simple_fn_arg(argument_name, syn_type));
        }

        // Convert the return type to a `syn` type, handling the `catch`
        // attribute here to use a `Result` in Rust.
        let ret = match ret {
            IdlType::Void => None,
            ret @ _ => match ret.to_syn_type(TypePosition::Return) {
                Some(ret) => Some(ret),
                None => {
                    log::warn!("Unsupported return type: {:?} on {:?}", ret, rust_name);
                    return None;
                }
            },
        };
        let js_ret = ret.clone();
        let ret = if catch {
            Some(ret.map_or_else(|| result_ty(unit_ty()), result_ty))
        } else {
            ret
        };

        Some(ast::ImportFunction {
            function: ast::Function {
                name: js_name.to_string(),
                name_span: Span::call_site(),
                renamed_via_js_name: false,
                arguments,
                ret: ret.clone(),
                rust_attrs: vec![],
                rust_vis: public(),
                r#async: false,
            },
            rust_name: rust_ident(rust_name),
            js_ret: js_ret.clone(),
            variadic,
            catch,
            structural,
            assert_no_shim: false,
            shim: {
                let ns = match kind {
                    ast::ImportFunctionKind::Normal => "",
                    ast::ImportFunctionKind::Method { ref class, .. } => class,
                };
                raw_ident(&format!("__widl_f_{}_{}", rust_name, ns))
            },
            kind,
            doc_comment,
        })
    }

    /// Create a wasm-bindgen getter method, if possible.
    pub fn create_getter(
        &self,
        name: &str,
        ty: &weedle::types::Type<'src>,
        self_name: &str,
        is_static: bool,
        attrs: &Option<ExtendedAttributeList>,
        container_attrs: Option<&ExtendedAttributeList>,
    ) -> Option<ast::ImportFunction> {
        let kind = ast::OperationKind::Getter(Some(raw_ident(name)));
        let kind = self.import_function_kind(self_name, is_static, kind);
        let ret = ty.to_idl_type(self);
        self.create_one_function(
            &name,
            &snake_case_ident(name),
            None.into_iter(),
            &ret,
            kind,
            is_structural(attrs.as_ref(), container_attrs),
            throws(attrs),
            false,
            Some(format!(
                "The `{}` getter\n\n{}",
                name,
                mdn_doc(self_name, Some(name))
            )),
        )
    }

    /// Create a wasm-bindgen setter method, if possible.
    pub fn create_setter(
        &self,
        name: &str,
        field_ty: &weedle::types::Type<'src>,
        self_name: &str,
        is_static: bool,
        attrs: &Option<ExtendedAttributeList>,
        container_attrs: Option<&ExtendedAttributeList>,
    ) -> Option<ast::ImportFunction> {
        let kind = ast::OperationKind::Setter(Some(raw_ident(name)));
        let kind = self.import_function_kind(self_name, is_static, kind);
        let field_ty = field_ty.to_idl_type(self);
        self.create_one_function(
            &name,
            &format!("set_{}", name).to_snake_case(),
            Some((name, &field_ty)).into_iter(),
            &IdlType::Void,
            kind,
            is_structural(attrs.as_ref(), container_attrs),
            throws(attrs),
            false,
            Some(format!(
                "The `{}` setter\n\n{}",
                name,
                mdn_doc(self_name, Some(name))
            )),
        )
    }

    pub fn import_function_kind(
        &self,
        self_name: &str,
        is_static: bool,
        operation_kind: ast::OperationKind,
    ) -> ast::ImportFunctionKind {
        let operation = ast::Operation {
            is_static,
            kind: operation_kind,
        };
        let ty = ident_ty(rust_ident(camel_case_ident(&self_name).as_str()));
        ast::ImportFunctionKind::Method {
            class: self_name.to_string(),
            ty,
            kind: ast::MethodKind::Operation(operation),
        }
    }

    pub fn create_imports(
        &self,
        container_attrs: Option<&ExtendedAttributeList<'src>>,
        kind: ast::ImportFunctionKind,
        id: &OperationId<'src>,
        data: &OperationData<'src>,
    ) -> Vec<ast::ImportFunction> {
        // First up, prune all signatures that reference unsupported arguments.
        // We won't consider these until said arguments are implemented.
        //
        // Note that we handle optional arguments as well. Optional arguments
        // should only appear at the end of argument lists and when we see one
        // we can simply push our signature so far onto the list for the
        // signature where that and all remaining optional arguments are
        // undefined.
        let mut signatures = Vec::new();
        'outer: for signature in data.signatures.iter() {
            let mut idl_args = Vec::with_capacity(signature.args.len());
            for (i, arg) in signature.args.iter().enumerate() {
                if arg.optional {
                    assert!(
                        signature.args[i..]
                            .iter()
                            .all(|arg| arg.optional || arg.variadic),
                        "Not optional or variadic argument after optional argument: {:?}",
                        signature.args,
                    );
                    signatures.push((signature, idl_args.clone()));
                }

                let idl_type = arg.ty.to_idl_type(self);
                let idl_type = self.maybe_adjust(idl_type, id);
                idl_args.push(idl_type);
            }
            signatures.push((signature, idl_args));
        }

        // Next expand all the signatures in `data` into all signatures that
        // we're going to generate. These signatures will be used to determine
        // the names for all the various functions.
        #[derive(Clone)]
        struct ExpandedSig<'a> {
            orig: &'a Signature<'a>,
            args: Vec<IdlType<'a>>,
        }

        let mut actual_signatures = Vec::new();
        for (signature, idl_args) in signatures.iter() {
            let start = actual_signatures.len();

            // Start off with an empty signature, this'll handle zero-argument
            // cases and otherwise the loop below will continue to add on to this.
            actual_signatures.push(ExpandedSig {
                orig: signature,
                args: Vec::with_capacity(signature.args.len()),
            });

            for (i, idl_type) in idl_args.iter().enumerate() {
                // small sanity check
                assert!(start < actual_signatures.len());
                for sig in actual_signatures[start..].iter() {
                    assert_eq!(sig.args.len(), i);
                }

                // The first element of the flattened type gets pushed directly
                // in-place, but all other flattened types will cause new
                // signatures to be created.
                let cur = actual_signatures.len();
                for (j, idl_type) in idl_type.flatten().into_iter().enumerate() {
                    for k in start..cur {
                        if j == 0 {
                            actual_signatures[k].args.push(idl_type.clone());
                        } else {
                            let mut sig = actual_signatures[k].clone();
                            assert_eq!(sig.args.len(), i + 1);
                            sig.args.truncate(i);
                            sig.args.push(idl_type.clone());
                            actual_signatures.push(sig);
                        }
                    }
                }
            }
        }

        let (name, force_structural, force_throws) = match id {
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
            OperationId::Constructor(_) => ("new", false, true),
            OperationId::Operation(Some(s)) => (*s, false, false),
            OperationId::Operation(None) => {
                log::warn!("unsupported unnamed operation");
                return Vec::new();
            }
            OperationId::IndexingGetter => ("get", true, false),
            OperationId::IndexingSetter => ("set", true, false),
            OperationId::IndexingDeleter => ("delete", true, false),
        };

        let mut ret = Vec::new();
        for signature in actual_signatures.iter() {
            // Ignore signatures with invalid return types
            //
            // TODO: overloads probably never change return types, so we should
            //       do this much earlier to avoid all the above work if
            //       possible.
            let ret_ty = signature.orig.ret.to_idl_type(self);

            let mut rust_name = snake_case_ident(name);
            let mut first = true;
            for (i, arg) in signature.args.iter().enumerate() {
                // Find out if any other known signature either has the same
                // name for this argument or a different type for this argument.
                let mut any_same_name = false;
                let mut any_different_type = false;
                let mut any_different = false;
                let arg_name = signature.orig.args[i].name;
                for other in actual_signatures.iter() {
                    if other.orig.args.get(i).map(|s| s.name) == Some(arg_name) {
                        if !ptr::eq(signature, other) {
                            any_same_name = true;
                        }
                    }
                    if let Some(other) = other.args.get(i) {
                        if other != arg {
                            any_different_type = true;
                            any_different = true;
                        }
                    } else {
                        any_different = true;
                    }
                }

                // If all signatures have the exact same type for this argument,
                // then there's nothing to disambiguate so we don't modify the
                // name.
                if !any_different {
                    continue;
                }
                if first {
                    rust_name.push_str("_with_");
                    first = false;
                } else {
                    rust_name.push_str("_and_");
                }

                // If this name of the argument for this signature is unique
                // then that's a bit more human readable so we include it in the
                // method name. Otherwise the type name should disambiguate
                // correctly.
                //
                // If any signature's argument has the same name as our argument
                // then we can't use that if the types are also the same because
                // otherwise it could be ambiguous.
                if any_same_name && any_different_type {
                    arg.push_snake_case_name(&mut rust_name);
                } else {
                    rust_name.push_str(&snake_case_ident(arg_name));
                }
            }
            let structural =
                force_structural || is_structural(signature.orig.attrs.as_ref(), container_attrs);
            let catch = force_throws || throws(&signature.orig.attrs);
            let variadic = signature.args.len() == signature.orig.args.len()
                && signature
                    .orig
                    .args
                    .last()
                    .map(|arg| arg.variadic)
                    .unwrap_or(false);
            ret.extend(
                self.create_one_function(
                    name,
                    &rust_name,
                    signature
                        .args
                        .iter()
                        .zip(&signature.orig.args)
                        .map(|(idl_type, orig_arg)| (orig_arg.name, idl_type)),
                    &ret_ty,
                    kind.clone(),
                    structural,
                    catch,
                    variadic,
                    None,
                ),
            );
            if !variadic {
                continue;
            }
            let last_idl_type = &signature.args[signature.args.len() - 1];
            let last_name = signature.orig.args[signature.args.len() - 1].name;
            for i in 0..=MAX_VARIADIC_ARGUMENTS_COUNT {
                ret.extend(
                    self.create_one_function(
                        name,
                        &format!("{}_{}", rust_name, i),
                        signature.args[..signature.args.len() - 1]
                            .iter()
                            .zip(&signature.orig.args)
                            .map(|(idl_type, orig_arg)| (orig_arg.name.to_string(), idl_type))
                            .chain((1..=i).map(|j| (format!("{}_{}", last_name, j), last_idl_type)))
                            .collect::<Vec<_>>()
                            .iter()
                            .map(|(name, idl_type)| (&name[..], idl_type.clone())),
                        &ret_ty,
                        kind.clone(),
                        structural,
                        catch,
                        false,
                        None,
                    ),
                );
            }
        }
        return ret;
    }

    /// When generating our web_sys APIs we default to setting slice references that
    /// get passed to JS as mutable in case they get mutated in JS.
    ///
    /// In certain cases we know for sure that the slice will not get mutated - for
    /// example when working with the WebGlRenderingContext APIs.
    ///
    /// Here we implement a whitelist for those cases. This whitelist is currently
    /// maintained by hand.
    ///
    /// When adding to this whitelist add tests to crates/web-sys/tests/wasm/whitelisted_immutable_slices.rs
    fn maybe_adjust<'a>(&self, mut idl_type: IdlType<'a>, id: &'a OperationId) -> IdlType<'a> {
        let op = match id {
            OperationId::Operation(Some(op)) => op,
            _ => return idl_type,
        };

        if self.immutable_slice_whitelist.contains(op) {
            flag_slices_immutable(&mut idl_type)
        }

        idl_type
    }
}

/// Search for an attribute by name in some webidl object's attributes.
fn has_named_attribute(list: Option<&ExtendedAttributeList>, attribute: &str) -> bool {
    let list = match list {
        Some(list) => list,
        None => return false,
    };
    list.body.list.iter().any(|attr| match attr {
        ExtendedAttribute::NoArgs(name) => (name.0).0 == attribute,
        _ => false,
    })
}

fn has_ident_attribute(list: Option<&ExtendedAttributeList>, ident: &str) -> bool {
    let list = match list {
        Some(list) => list,
        None => return false,
    };
    list.body.list.iter().any(|attr| match attr {
        ExtendedAttribute::Ident(id) => id.lhs_identifier.0 == ident,
        ExtendedAttribute::IdentList(id) => id.identifier.0 == ident,
        _ => false,
    })
}

/// ChromeOnly is for things that are only exposed to privileged code in Firefox.
pub fn is_chrome_only(ext_attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(ext_attrs.as_ref(), "ChromeOnly")
}

/// Whether a webidl object is marked as a no interface object.
pub fn is_no_interface_object(ext_attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(ext_attrs.as_ref(), "NoInterfaceObject")
}

pub fn get_rust_deprecated<'a>(ext_attrs: &Option<ExtendedAttributeList<'a>>) -> Option<&'a str> {
    ext_attrs
        .as_ref()?
        .body
        .list
        .iter()
        .filter_map(|attr| match attr {
            ExtendedAttribute::Ident(id) => Some(id),
            _ => None,
        })
        .filter(|attr| attr.lhs_identifier.0 == "RustDeprecated")
        .filter_map(|ident| match ident.rhs {
            IdentifierOrString::String(s) => Some(s),
            IdentifierOrString::Identifier(_) => None,
        })
        .next()
        .map(|s| s.0)
}

/// Whether a webidl object is marked as structural.
pub fn is_structural(
    item_attrs: Option<&ExtendedAttributeList>,
    container_attrs: Option<&ExtendedAttributeList>,
) -> bool {
    // Note that once host bindings is implemented we'll want to switch this
    // from `true` to `false`, and then we'll want to largely read information
    // from the WebIDL about whether to use structural bindings or not.
    true || has_named_attribute(item_attrs, "Unforgeable")
        || has_named_attribute(container_attrs, "Unforgeable")
        || has_ident_attribute(container_attrs, "Global")
}

/// Whether a webidl object is marked as throwing.
pub fn throws(attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(attrs.as_ref(), "Throws")
}

/// Create a syn `pub` token
pub fn public() -> syn::Visibility {
    syn::Visibility::Public(syn::VisPublic {
        pub_token: Default::default(),
    })
}

fn flag_slices_immutable(ty: &mut IdlType) {
    match ty {
        IdlType::Int8Array { immutable }
        | IdlType::Uint8Array { immutable }
        | IdlType::Uint8ClampedArray { immutable }
        | IdlType::Int16Array { immutable }
        | IdlType::Uint16Array { immutable }
        | IdlType::Int32Array { immutable }
        | IdlType::Uint32Array { immutable }
        | IdlType::Float32Array { immutable }
        | IdlType::Float64Array { immutable }
        | IdlType::ArrayBufferView { immutable }
        | IdlType::BufferSource { immutable } => *immutable = true,
        IdlType::Nullable(item) => flag_slices_immutable(item),
        IdlType::FrozenArray(item) => flag_slices_immutable(item),
        IdlType::Sequence(item) => flag_slices_immutable(item),
        IdlType::Promise(item) => flag_slices_immutable(item),
        IdlType::Record(item1, item2) => {
            flag_slices_immutable(item1);
            flag_slices_immutable(item2);
        }
        IdlType::Union(list) => {
            for item in list {
                flag_slices_immutable(item);
            }
        }
        // catch-all for everything else like Object
        _ => {}
    }
}
