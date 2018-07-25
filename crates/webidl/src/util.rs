use std::iter::{self, FromIterator};

use backend;
use backend::util::{ident_ty, leading_colon_path_ty, raw_ident, rust_ident, simple_path_ty};
use heck::{CamelCase, SnakeCase};
use proc_macro2::Ident;
use syn;
use webidl;
use webidl::ast::ExtendedAttribute;

use first_pass::FirstPassRecord;

/// Take a type and create an immutable shared reference to that type.
fn shared_ref(ty: syn::Type) -> syn::Type {
    syn::TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: Box::new(ty),
    }.into()
}

/// For a webidl const type node, get the corresponding syn type node.
pub fn webidl_const_ty_to_syn_ty(ty: &webidl::ast::ConstType) -> syn::Type {
    use webidl::ast::ConstType::*;

    // similar to webidl_ty_to_syn_ty
    match ty {
        Boolean => ident_ty(raw_ident("bool")),
        Byte => ident_ty(raw_ident("i8")),
        Octet => ident_ty(raw_ident("u8")),
        RestrictedDouble | UnrestrictedDouble => ident_ty(raw_ident("f64")),
        RestrictedFloat | UnrestrictedFloat => ident_ty(raw_ident("f32")),
        SignedLong => ident_ty(raw_ident("i32")),
        SignedLongLong => ident_ty(raw_ident("i64")),
        SignedShort => ident_ty(raw_ident("i16")),
        UnsignedLong => ident_ty(raw_ident("u32")),
        UnsignedLongLong => ident_ty(raw_ident("u64")),
        UnsignedShort => ident_ty(raw_ident("u16")),
        Identifier(ref id) => ident_ty(rust_ident(id)),
    }
}

/// Map a webidl const value to the correct wasm-bindgen const value
pub fn webidl_const_v_to_backend_const_v(v: &webidl::ast::ConstValue) -> backend::ast::ConstValue {
    match *v {
        webidl::ast::ConstValue::BooleanLiteral(b) => backend::ast::ConstValue::BooleanLiteral(b),
        webidl::ast::ConstValue::FloatLiteral(f) => backend::ast::ConstValue::FloatLiteral(f),
        webidl::ast::ConstValue::SignedIntegerLiteral(i) => backend::ast::ConstValue::SignedIntegerLiteral(i),
        webidl::ast::ConstValue::UnsignedIntegerLiteral(u) => backend::ast::ConstValue::UnsignedIntegerLiteral(u),
        webidl::ast::ConstValue::Null => backend::ast::ConstValue::Null,
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
fn slice_ty(t: syn::Type) -> syn::Type {
    syn::TypeSlice {
        bracket_token: Default::default(),
        elem: Box::new(t),
    }.into()
}

/// From `T` create `Vec<T>`.
fn vec_ty(t: syn::Type) -> syn::Type {
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

/// Possible positions for a type in a function signature.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypePosition {
    Argument,
    Return,
}

impl<'a> FirstPassRecord<'a> {
    /// Use information from the first pass to work out the correct Rust type to use for
    /// a given WebIDL type.
    pub fn webidl_ty_to_syn_ty(
        &self,
        ty: &webidl::ast::Type,
        pos: TypePosition,
    ) -> Option<syn::Type> {
        // Array type is borrowed for arguments (`&[T]`) and owned for return value (`Vec<T>`).
        let array = |base_ty: &str| {
            match pos {
                TypePosition::Argument => {
                    shared_ref(slice_ty(ident_ty(raw_ident(base_ty))))
                }
                TypePosition::Return => {
                    vec_ty(ident_ty(raw_ident(base_ty)))
                }
            }
        };

        let base_ty = match ty.kind {
            // `any` becomes `::wasm_bindgen::JsValue`.
            webidl::ast::TypeKind::Any => {
                simple_path_ty(vec![rust_ident("wasm_bindgen"), rust_ident("JsValue")])
            }

            // A reference to a type by name becomes the same thing in the
            // bindings.
            webidl::ast::TypeKind::Identifier(ref id) => {
                let ty = ident_ty(rust_ident(id.to_camel_case().as_str()));
                if self.interfaces.contains(id) {
                    if pos == TypePosition::Argument {
                        shared_ref(ty)
                    } else {
                        ty
                    }
                } else if self.dictionaries.contains(id) {
                    ty
                } else if self.enums.contains(id) {
                    ty
                } else {
                    warn!("unrecognized type {}", id);
                    ty
                }
            }

            // Scalars.
            webidl::ast::TypeKind::Boolean => ident_ty(raw_ident("bool")),
            webidl::ast::TypeKind::Byte => ident_ty(raw_ident("i8")),
            webidl::ast::TypeKind::Octet => ident_ty(raw_ident("u8")),
            webidl::ast::TypeKind::RestrictedDouble | webidl::ast::TypeKind::UnrestrictedDouble => {
                ident_ty(raw_ident("f64"))
            }
            webidl::ast::TypeKind::RestrictedFloat | webidl::ast::TypeKind::UnrestrictedFloat => {
                ident_ty(raw_ident("f32"))
            }
            webidl::ast::TypeKind::SignedLong => ident_ty(raw_ident("i32")),
            webidl::ast::TypeKind::SignedLongLong => ident_ty(raw_ident("i64")),
            webidl::ast::TypeKind::SignedShort => ident_ty(raw_ident("i16")),
            webidl::ast::TypeKind::UnsignedLong => ident_ty(raw_ident("u32")),
            webidl::ast::TypeKind::UnsignedLongLong => ident_ty(raw_ident("u64")),
            webidl::ast::TypeKind::UnsignedShort => ident_ty(raw_ident("u16")),

            webidl::ast::TypeKind::Float32Array => array("f32"),
            webidl::ast::TypeKind::Float64Array => array("f64"),
            webidl::ast::TypeKind::Int8Array => array("i8"),
            webidl::ast::TypeKind::Int16Array => array("i16"),
            webidl::ast::TypeKind::Int32Array => array("i32"),
            webidl::ast::TypeKind::Uint8Array => array("u8"),
            webidl::ast::TypeKind::Uint8ClampedArray => array("u8"),
            webidl::ast::TypeKind::Uint16Array => array("u16"),
            webidl::ast::TypeKind::Uint32Array => array("u32"),

            // strings -> `&str` for arguments and `String` for return
            //
            // Note that DOMString mostly makes sense here, ByteString maps to
            // String in JS [1], along with USVString
            //
            // [1]: https://developer.mozilla.org/en-US/docs/Web/API/ByteString
            // [2]: https://developer.mozilla.org/en-US/docs/Web/API/USVString
            webidl::ast::TypeKind::DOMString
            | webidl::ast::TypeKind::ByteString
            | webidl::ast::TypeKind::USVString => {
                match pos {
                    TypePosition::Argument => shared_ref(ident_ty(raw_ident("str"))),
                    TypePosition::Return => ident_ty(raw_ident("String")),
                }
            }

            // Support for these types is not yet implemented, so skip
            // generating any bindings for this function.
            webidl::ast::TypeKind::ArrayBuffer
            | webidl::ast::TypeKind::DataView
            | webidl::ast::TypeKind::Error
            | webidl::ast::TypeKind::FrozenArray(_)
            | webidl::ast::TypeKind::Object
            | webidl::ast::TypeKind::Promise(_)
            | webidl::ast::TypeKind::Record(..)
            | webidl::ast::TypeKind::Sequence(_)
            | webidl::ast::TypeKind::Symbol
            | webidl::ast::TypeKind::Union(_) => {
                return None;
            }
        };

        // Map nullable to an option.
        if ty.nullable {
            let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Default::default(),
                args: FromIterator::from_iter(vec![
                    syn::GenericArgument::Type(base_ty),
                ]),
                gt_token: Default::default(),
            });

            let ident = raw_ident("Option");
            let seg = syn::PathSegment { ident, arguments };
            let path: syn::Path = seg.into();
            let ty = syn::TypePath { qself: None, path };
            Some(ty.into())
        } else {
            Some(base_ty)
        }
    }

    /// Use the first pass to convert webidl function arguments to rust arguments.
    ///
    /// `kind` is whether the function is a method, in which case we would need a `self`
    /// parameter.
    fn webidl_arguments_to_syn_arg_captured<'b, I>(
        &self,
        arguments: I,
        kind: &backend::ast::ImportFunctionKind,
    ) -> Option<Vec<syn::ArgCaptured>>
    where
        I: Iterator<Item = (&'b str, &'b webidl::ast::Type, bool)>,
    {
        let estimate = arguments.size_hint();
        let len = estimate.1.unwrap_or(estimate.0);
        let mut res = if let backend::ast::ImportFunctionKind::Method {
            ty,
            kind:
                backend::ast::MethodKind::Operation(backend::ast::Operation {
                    is_static: false, ..
                }),
            ..
        } = kind
        {
            let mut res = Vec::with_capacity(len + 1);
            res.push(simple_fn_arg(raw_ident("self_"), shared_ref(ty.clone())));
            res
        } else {
            Vec::with_capacity(len)
        };

        for (name, ty, variadic) in arguments {
            if variadic {
                warn!("Variadic arguments are not supported yet",);
                return None;
            }

            match self.webidl_ty_to_syn_ty(ty, TypePosition::Argument) {
                None => {
                    warn!("Argument's type is not yet supported: {:?}", ty);
                    return None;
                }
                Some(ty) => res.push(simple_fn_arg(rust_ident(&name.to_snake_case()), ty)),
            }
        }

        Some(res)
    }

    /// Create a wasm-bindgen function, if possible.
    pub fn create_function<'b, I>(
        &self,
        name: &str,
        arguments: I,
        mut ret: Option<syn::Type>,
        kind: backend::ast::ImportFunctionKind,
        structural: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction>
    where
        I: Iterator<Item = (&'b str, &'b webidl::ast::Type, bool)>,
    {
        let rust_name = rust_ident(&name.to_snake_case());
        let name = raw_ident(name);

        let arguments = self.webidl_arguments_to_syn_arg_captured(arguments, &kind)?;

        let js_ret = ret.clone();

        if catch {
            ret = Some(ret.map_or_else(|| result_ty(unit_ty()), result_ty))
        }

        let shim = {
            let ns = match kind {
                backend::ast::ImportFunctionKind::Normal => "",
                backend::ast::ImportFunctionKind::Method { ref class, .. } => class,
            };

            raw_ident(&format!("__widl_f_{}_{}", rust_name, ns))
        };

        Some(backend::ast::ImportFunction {
            function: backend::ast::Function {
                name,
                arguments,
                ret,
                rust_attrs: vec![],
                rust_vis: public(),
            },
            rust_name,
            js_ret,
            catch,
            structural,
            kind,
            shim,
        })
    }

    /// Create a wasm-bindgen method, if possible.
    pub fn create_basic_method(
        &self,
        arguments: &[webidl::ast::Argument],
        name: Option<&String>,
        return_type: &webidl::ast::ReturnType,
        self_name: &str,
        is_static: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction> {
        let name = match name {
            None => {
                warn!("Operations without a name are unsupported");
                return None;
            }
            Some(ref name) => name,
        };

        let kind = backend::ast::ImportFunctionKind::Method {
            class: self_name.to_string(),
            ty: ident_ty(rust_ident(self_name.to_camel_case().as_str())),
            kind: backend::ast::MethodKind::Operation(backend::ast::Operation {
                is_static,
                kind: backend::ast::OperationKind::Regular,
            }),
        };

        let ret = match return_type {
            webidl::ast::ReturnType::Void => None,
            webidl::ast::ReturnType::NonVoid(ty) => {
                match self.webidl_ty_to_syn_ty(ty, TypePosition::Return) {
                    None => {
                        warn!("Operation's return type is not yet supported: {:?}", ty);
                        return None;
                    }
                    Some(ty) => Some(ty),
                }
            }
        };

        self.create_function(
            &name,
            arguments
                .iter()
                .map(|arg| (&*arg.name, &*arg.type_, arg.variadic)),
            ret,
            kind,
            false,
            catch,
        )
    }

    /// Create a wasm-bindgen getter method, if possible.
    pub fn create_getter(
        &self,
        name: &str,
        ty: &webidl::ast::Type,
        self_name: &str,
        is_static: bool,
        is_structural: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction> {
        let ret = match self.webidl_ty_to_syn_ty(ty, TypePosition::Return) {
            None => {
                warn!("Attribute's type does not yet support reading: {:?}", ty);
                return None;
            }
            Some(ty) => Some(ty),
        };

        let kind = backend::ast::ImportFunctionKind::Method {
            class: self_name.to_string(),
            ty: ident_ty(rust_ident(self_name.to_camel_case().as_str())),
            kind: backend::ast::MethodKind::Operation(backend::ast::Operation {
                is_static,
                kind: backend::ast::OperationKind::Getter(Some(raw_ident(name))),
            }),
        };

        self.create_function(name, iter::empty(), ret, kind, is_structural, catch)
    }

    /// Create a wasm-bindgen setter method, if possible.
    pub fn create_setter(
        &self,
        name: &str,
        ty: &webidl::ast::Type,
        self_name: &str,
        is_static: bool,
        is_structural: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction> {
        let kind = backend::ast::ImportFunctionKind::Method {
            class: self_name.to_string(),
            ty: ident_ty(rust_ident(self_name.to_camel_case().as_str())),
            kind: backend::ast::MethodKind::Operation(backend::ast::Operation {
                is_static,
                kind: backend::ast::OperationKind::Setter(Some(raw_ident(name))),
            }),
        };

        self.create_function(
            &format!("set_{}", name),
            iter::once((name, ty, false)),
            None,
            kind,
            is_structural,
            catch,
        )
    }
}

/// Search for an attribute by name in some webidl object's attributes.
fn has_named_attribute(ext_attrs: &[Box<ExtendedAttribute>], attribute: &str) -> bool {
    ext_attrs.iter().any(|attr| match &**attr {
        ExtendedAttribute::NoArguments(webidl::ast::Other::Identifier(name)) => {
            name == attribute
        }
        _ => false,
    })
}

/// ChromeOnly is for things that are only exposed to privileged code in Firefox.
pub fn is_chrome_only(ext_attrs: &[Box<ExtendedAttribute>]) -> bool {
    has_named_attribute(ext_attrs, "ChromeOnly")
}

/// Whether a webidl object is marked as a no interface object.
pub fn is_no_interface_object(ext_attrs: &[Box<ExtendedAttribute>]) -> bool {
    has_named_attribute(ext_attrs, "NoInterfaceObject")
}

/// Whether a webidl object is marked as structural.
pub fn is_structural(attrs: &[Box<ExtendedAttribute>]) -> bool {
    attrs.iter().any(|attr| match &**attr {
        ExtendedAttribute::NoArguments(webidl::ast::Other::Identifier(name)) => {
            name == "Unforgeable"
        }
        _ => false,
    })
}

/// Whether a webidl object is marked as throwing.
pub fn throws(attrs: &[Box<ExtendedAttribute>]) -> bool {
    attrs.iter().any(|attr| match &**attr {
        ExtendedAttribute::NoArguments(webidl::ast::Other::Identifier(name)) => name == "Throws",
        _ => false,
    })
}

/// Create a syn `pub` token
pub fn public() -> syn::Visibility {
    syn::Visibility::Public(syn::VisPublic {
        pub_token: Default::default(),
    })
}
