use std::iter::FromIterator;
use std::ptr;

use backend;
use backend::util::{ident_ty, leading_colon_path_ty, raw_ident, rust_ident};
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use proc_macro2::Ident;
use syn;
use weedle;
use weedle::attribute::{ExtendedAttributeList, ExtendedAttribute};
use weedle::literal::{ConstValue, FloatLit, IntegerLit};

use first_pass::{FirstPassRecord, OperationId, OperationData, Signature};
use idl_type::{IdlType, ToIdlType};

/// Take a type and create an immutable shared reference to that type.
pub(crate) fn shared_ref(ty: syn::Type) -> syn::Type {
    syn::TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: Box::new(ty),
    }.into()
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
    pub fn create_one_function<'a>(
        &self,
        js_name: &str,
        rust_name: &str,
        idl_arguments: impl Iterator<Item = (&'a str, &'a IdlType<'src>)>,
        ret: &IdlType<'src>,
        kind: backend::ast::ImportFunctionKind,
        structural: bool,
        catch: bool,
        doc_comment: Option<String>,
    ) -> Option<backend::ast::ImportFunction> where 'src: 'a {
        // Convert all of the arguments from their IDL type to a `syn` type,
        // ready to pass to the backend.
        //
        // Note that for non-static methods we add a `&self` type placeholder,
        // but this type isn't actually used so it's just here for show mostly.
        let mut arguments = if let &backend::ast::ImportFunctionKind::Method {
            ref ty,
            kind: backend::ast::MethodKind::Operation(
                backend::ast::Operation {
                    is_static: false, ..
                }
            ),
            ..
        } = &kind {
            let mut res = Vec::with_capacity(idl_arguments.size_hint().0 + 1);
            res.push(simple_fn_arg(raw_ident("self_"), shared_ref(ty.clone())));
            res
        } else {
            Vec::with_capacity(idl_arguments.size_hint().0)
        };
        for (argument_name, idl_type) in idl_arguments {
            let syn_type = match idl_type.to_syn_type(TypePosition::Argument) {
                Some(t) => t,
                None => {
                    warn!(
                        "Unsupported argument type: {:?} on {:?}",
                        idl_type,
                        rust_name
                    );
                    return None
                }
            };
            let argument_name = rust_ident(&argument_name.to_snake_case());
            arguments.push(simple_fn_arg(argument_name, syn_type));
        }

        // Convert the return type to a `syn` type, handling the `catch`
        // attribute here to use a `Result` in Rust.
        let ret = match ret {
            IdlType::Void => None,
            ret @ _ => {
                match ret.to_syn_type(TypePosition::Return) {
                    Some(ret) => Some(ret),
                    None => {
                        warn!(
                            "Unsupported return type: {:?} on {:?}",
                            ret,
                            rust_name
                        );
                        return None
                    }
                }
            },
        };
        let js_ret = ret.clone();
        let ret = if catch {
            Some(ret.map_or_else(|| result_ty(unit_ty()), result_ty))
        } else {
            ret
        };

        Some(backend::ast::ImportFunction {
            function: backend::ast::Function {
                name: js_name.to_string(),
                arguments,
                ret: ret.clone(),
                rust_attrs: vec![],
                rust_vis: public(),
            },
            rust_name: rust_ident(rust_name),
            js_ret: js_ret.clone(),
            variadic: false,
            catch,
            structural,
            shim:{
                let ns = match kind {
                    backend::ast::ImportFunctionKind::ScopedMethod { .. } |
                    backend::ast::ImportFunctionKind::Normal => "",
                    backend::ast::ImportFunctionKind::Method { ref class, .. } => class,
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
        global: bool,
        attrs: &Option<ExtendedAttributeList>,
        container_attrs: Option<&ExtendedAttributeList>,
    ) -> Option<backend::ast::ImportFunction> {
        let kind = backend::ast::OperationKind::Getter(Some(raw_ident(name)));
        let kind = self.import_function_kind(self_name, global, is_static, kind);
        let ret = ty.to_idl_type(self)?;
        self.create_one_function(
            &name,
            &snake_case_ident(name),
            None.into_iter(),
            &ret,
            kind,
            is_structural(attrs.as_ref(), container_attrs),
            throws(attrs),
            Some(format!("The `{}` getter\n\n{}", name, mdn_doc(self_name, Some(name))))
        )
    }

    /// Create a wasm-bindgen setter method, if possible.
    pub fn create_setter(
        &self,
        name: &str,
        field_ty: &weedle::types::Type<'src>,
        self_name: &str,
        is_static: bool,
        global: bool,
        attrs: &Option<ExtendedAttributeList>,
        container_attrs: Option<&ExtendedAttributeList>,
    ) -> Option<backend::ast::ImportFunction> {
        let kind = backend::ast::OperationKind::Setter(Some(raw_ident(name)));
        let kind = self.import_function_kind(self_name, global, is_static, kind);
        let field_ty = field_ty.to_idl_type(self)?;
        self.create_one_function(
            &name,
            &format!("set_{}", name).to_snake_case(),
            Some((name, &field_ty)).into_iter(),
            &IdlType::Void,
            kind,
            is_structural(attrs.as_ref(), container_attrs),
            throws(attrs),
            Some(format!("The `{}` setter\n\n{}", name, mdn_doc(self_name, Some(name))))
        )
    }

    pub fn import_function_kind(
        &self,
        self_name: &str,
        global: bool,
        is_static: bool,
        operation_kind: backend::ast::OperationKind,
    ) -> backend::ast::ImportFunctionKind {
        let operation = backend::ast::Operation {
            is_static,
            kind: operation_kind,
        };
        let ty = ident_ty(rust_ident(camel_case_ident(&self_name).as_str()));
        if global {
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
        }
    }

    pub fn create_imports(
        &self,
        container_attrs: Option<&ExtendedAttributeList<'src>>,
        kind: backend::ast::ImportFunctionKind,
        id: &OperationId<'src>,
        data: &OperationData<'src>,
    )
        -> Vec<backend::ast::ImportFunction>
    {
        // First up, prune all signatures that reference unsupported arguments.
        // We won't consider these until said arguments are implemented.
        //
        // Note that we handle optional arguments as well. Optional arguments
        // should only appear at the end of argument lists and when we see one
        // we can simply push our signature so far onto the list for the
        // signature where that and all remaining optional arguments are
        // undefined.
        let mut signatures = Vec::new();
        'outer:
        for signature in data.signatures.iter() {
            let mut idl_args = Vec::with_capacity(signature.args.len());
            for (i, arg) in signature.args.iter().enumerate() {
                if arg.optional {
                    assert!(signature.args[i..].iter().all(|a| a.optional));
                    signatures.push((signature, idl_args.clone()));
                }
                match arg.ty.to_idl_type(self) {
                    Some(t) => idl_args.push(t),
                    None => continue 'outer,
                }
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
            let mut start = actual_signatures.len();

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
                warn!("unsupported unnamed operation");
                return Vec::new()
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
            let ret_ty = match signature.orig.ret.to_idl_type(self) {
                Some(ty) => ty,
                None => continue,
            };

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
                    continue
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
            ret.extend(self.create_one_function(
                name,
                &rust_name,
                signature.args.iter()
                    .zip(&signature.orig.args)
                    .map(|(ty, orig_arg)| (orig_arg.name, ty)),
                &ret_ty,
                kind.clone(),
                force_structural || is_structural(signature.orig.attrs.as_ref(), container_attrs),
                force_throws || throws(&signature.orig.attrs),
                None,
            ));
        }
        return ret;
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

/// ChromeOnly is for things that are only exposed to privileged code in Firefox.
pub fn is_chrome_only(ext_attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(ext_attrs.as_ref(), "ChromeOnly")
}

/// Whether a webidl object is marked as a no interface object.
pub fn is_no_interface_object(ext_attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(ext_attrs.as_ref(), "NoInterfaceObject")
}

/// Whether a webidl object is marked as structural.
pub fn is_structural(
    item_attrs: Option<&ExtendedAttributeList>,
    container_attrs: Option<&ExtendedAttributeList>,
) -> bool {
    has_named_attribute(item_attrs, "Unforgeable") ||
        has_named_attribute(container_attrs, "Unforgeable")
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
