use std::iter::{self, FromIterator};

use backend;
use backend::util::{ident_ty, leading_colon_path_ty, raw_ident, rust_ident};
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

fn type_kind_to_const_type(type_kind: &webidl::ast::TypeKind) -> webidl::ast::ConstType {
    match type_kind {
        webidl::ast::TypeKind::Boolean => webidl::ast::ConstType::Boolean,
        webidl::ast::TypeKind::Byte => webidl::ast::ConstType::Byte,
        webidl::ast::TypeKind::Identifier(identifier) => webidl::ast::ConstType::Identifier(identifier.clone()),
        webidl::ast::TypeKind::Octet => webidl::ast::ConstType::Octet,
        webidl::ast::TypeKind::RestrictedDouble => webidl::ast::ConstType::RestrictedDouble,
        webidl::ast::TypeKind::RestrictedFloat => webidl::ast::ConstType::RestrictedFloat,
        webidl::ast::TypeKind::SignedLong => webidl::ast::ConstType::SignedLong,
        webidl::ast::TypeKind::SignedLongLong => webidl::ast::ConstType::SignedLongLong,
        webidl::ast::TypeKind::SignedShort => webidl::ast::ConstType::SignedShort,
        webidl::ast::TypeKind::UnrestrictedDouble => webidl::ast::ConstType::UnrestrictedDouble,
        webidl::ast::TypeKind::UnrestrictedFloat => webidl::ast::ConstType::UnrestrictedFloat,
        webidl::ast::TypeKind::UnsignedLong => webidl::ast::ConstType::UnsignedLong,
        webidl::ast::TypeKind::UnsignedLongLong => webidl::ast::ConstType::UnsignedLongLong,
        webidl::ast::TypeKind::UnsignedShort => webidl::ast::ConstType::UnsignedShort,
        _ => panic!("can not convert TypeKind to ConstType: {:#?}", type_kind),
    }
}

/// Implemented on an AST type node to apply typedefs.
pub(crate) trait ApplyTypedefs {
    fn apply_typedefs<'a>(&self, record: &FirstPassRecord<'a>) -> Self;
}

impl ApplyTypedefs for webidl::ast::Type {
    fn apply_typedefs<'a>(&self, record: &FirstPassRecord<'a>) -> Self {
        webidl::ast::Type {
            extended_attributes: self.extended_attributes.clone(),
            kind: self.kind.apply_typedefs(record),
            nullable: self.nullable,
        }
    }
}

impl ApplyTypedefs for webidl::ast::ReturnType {
    fn apply_typedefs<'a>(&self, record: &FirstPassRecord<'a>) -> Self {
        match self {
            webidl::ast::ReturnType::NonVoid(ty) =>
                webidl::ast::ReturnType::NonVoid(Box::new(ty.apply_typedefs(record))),
            _ => self.clone(),
        }
    }
}

impl ApplyTypedefs for webidl::ast::StringType {
    fn apply_typedefs<'a>(&self, _: &FirstPassRecord<'a>) -> Self {
        *self
    }
}

impl ApplyTypedefs for webidl::ast::ConstType {
    fn apply_typedefs<'a>(&self, record: &FirstPassRecord<'a>) -> Self {
        match self {
            webidl::ast::ConstType::Identifier(identifier) =>
                record
                    .typedefs
                    .get(identifier)
                    .map(|ty| type_kind_to_const_type(&ty.kind))
                    .unwrap_or(self.clone()),
            _ => self.clone(),
        }
    }
}

impl ApplyTypedefs for webidl::ast::TypeKind {
    fn apply_typedefs<'a>(&self, record: &FirstPassRecord<'a>) -> Self {
        match self {
            webidl::ast::TypeKind::FrozenArray(ty) =>
                webidl::ast::TypeKind::FrozenArray(Box::new(ty.apply_typedefs(record))),
            webidl::ast::TypeKind::Identifier(identifier) => record
                .typedefs
                .get(identifier)
                .map(|ty| ty.kind.clone())
                .unwrap_or(self.clone()),
            webidl::ast::TypeKind::Promise(ty) =>
                webidl::ast::TypeKind::Promise(ty.apply_typedefs(record)),
            webidl::ast::TypeKind::Record(string_type, ty) => webidl::ast::TypeKind::Record(
                string_type.apply_typedefs(record),
                Box::new(ty.apply_typedefs(record)),
            ),
            webidl::ast::TypeKind::Union(types) => webidl::ast::TypeKind::Union(
                types
                    .iter()
                    .map(|ty| Box::new(ty.apply_typedefs(record)))
                    .collect(),
            ),
            _ => self.clone(),
        }
    }
}

impl ApplyTypedefs for webidl::ast::Argument {
    fn apply_typedefs<'a>(&self, record: &FirstPassRecord<'a>) -> Self {
        webidl::ast::Argument {
            extended_attributes: self.extended_attributes.clone(),
            default: self.default.clone(),
            name: self.name.clone(),
            optional: self.optional,
            type_: Box::new(self.type_.apply_typedefs(record)),
            variadic: self.variadic,
        }
    }
}

/// Implemented on an AST type node to generate a snake case name.
trait TypeToString {
    fn type_to_string(&self) -> String;
}

impl TypeToString for webidl::ast::Type {
    fn type_to_string(&self) -> String {
        if self.nullable {
            "opt_".to_owned() + &self.kind.type_to_string()
        } else {
            self.kind.type_to_string()
        }
    }
}

impl TypeToString for webidl::ast::ReturnType {
    fn type_to_string(&self) -> String {
        match self {
            webidl::ast::ReturnType::NonVoid(ty) => (*ty).type_to_string(),
            webidl::ast::ReturnType::Void => "void".to_owned(),
        }
    }
}

impl TypeToString for webidl::ast::StringType {
    fn type_to_string(&self) -> String {
        match self {
            webidl::ast::StringType::ByteString => "byte_str".to_owned(),
            webidl::ast::StringType::DOMString => "dom_str".to_owned(),
            webidl::ast::StringType::USVString => "usv_str".to_owned(),
        }
    }
}

impl TypeToString for webidl::ast::TypeKind {
    fn type_to_string(&self) -> String {
        match self {
            webidl::ast::TypeKind::Any => "any".to_owned(),
            webidl::ast::TypeKind::ArrayBuffer => "array_buffer".to_owned(),
            webidl::ast::TypeKind::Boolean => "bool".to_owned(),
            webidl::ast::TypeKind::Byte => "i8".to_owned(),
            webidl::ast::TypeKind::ByteString => "byte_str".to_owned(),
            webidl::ast::TypeKind::DOMString => "dom_str".to_owned(),
            webidl::ast::TypeKind::DataView => "data_view".to_owned(),
            webidl::ast::TypeKind::Error => "error".to_owned(),
            webidl::ast::TypeKind::Float32Array => "f32_array".to_owned(),
            webidl::ast::TypeKind::Float64Array => "f64_array".to_owned(),
            webidl::ast::TypeKind::FrozenArray(ty) => "frozen_array_of_".to_owned() + &ty.type_to_string(),
            webidl::ast::TypeKind::Identifier(identifier) => identifier.to_snake_case(),
            webidl::ast::TypeKind::Int16Array => "i16_array".to_owned(),
            webidl::ast::TypeKind::Int32Array => "i32_array".to_owned(),
            webidl::ast::TypeKind::Int8Array => "i8_array".to_owned(),
            webidl::ast::TypeKind::Octet => "u8".to_owned(),
            webidl::ast::TypeKind::Object => "object".to_owned(),
            webidl::ast::TypeKind::Promise(ty) => "promise_of_".to_owned() + &(*ty).type_to_string(),
            webidl::ast::TypeKind::Record(string_type, ty) => format!(
                "record_from_{}_to_{}",
                string_type.type_to_string(),
                (*ty).type_to_string()
            ),
            webidl::ast::TypeKind::RestrictedDouble => "restricted_f64".to_owned(),
            webidl::ast::TypeKind::RestrictedFloat => "restricted_f32".to_owned(),
            webidl::ast::TypeKind::Sequence(ty) => "sequence_of_".to_owned() + &ty.type_to_string(),
            webidl::ast::TypeKind::SignedLong => "i32".to_owned(),
            webidl::ast::TypeKind::SignedLongLong => "i64".to_owned(),
            webidl::ast::TypeKind::SignedShort => "i16".to_owned(),
            webidl::ast::TypeKind::Symbol => "symbol".to_owned(),
            webidl::ast::TypeKind::USVString => "usv_str".to_owned(),
            webidl::ast::TypeKind::Uint16Array => "u16_array".to_owned(),
            webidl::ast::TypeKind::Uint32Array => "u32_array".to_owned(),
            webidl::ast::TypeKind::Uint8Array => "u8_array".to_owned(),
            webidl::ast::TypeKind::Uint8ClampedArray => "u8_clamped_array".to_owned(),
            webidl::ast::TypeKind::Union(types) => "union_of_".to_owned() + &types
                .iter()
                .map(|ty| (*ty).type_to_string())
                .collect::<Vec<_>>()
                .join("_and_"),
            webidl::ast::TypeKind::UnrestrictedDouble => "unrestricted_f64".to_owned(),
            webidl::ast::TypeKind::UnrestrictedFloat => "unrestricted_f32".to_owned(),
            webidl::ast::TypeKind::UnsignedLong => "u32".to_owned(),
            webidl::ast::TypeKind::UnsignedLongLong => "u64".to_owned(),
            webidl::ast::TypeKind::UnsignedShort => "u16".to_owned(),
        }
    }
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
                leading_colon_path_ty(vec![rust_ident("wasm_bindgen"), rust_ident("JsValue")])
            }

            // A reference to a type by name becomes the same thing in the
            // bindings.
            webidl::ast::TypeKind::Identifier(ref id) => {
                let ty = ident_ty(rust_ident(camel_case_ident(&id).as_str()));
                if self.interfaces.contains_key(id) {
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

            // This seems like a "naively correct" mapping, but the online docs
            // are a bit scary in this regard...
            //
            // https://heycam.github.io/webidl/#es-buffer-source-types
            webidl::ast::TypeKind::ArrayBuffer => {
                leading_colon_path_ty(vec![rust_ident("js_sys"), rust_ident("ArrayBuffer")])
            }

            // The WebIDL `object` maps to the ECMAScript `Object`
            //
            // https://heycam.github.io/webidl/#es-object
            webidl::ast::TypeKind::Object => {
                leading_colon_path_ty(vec![rust_ident("js_sys"), rust_ident("Object")])
            }

            // Support for these types is not yet implemented, so skip
            // generating any bindings for this function.
            | webidl::ast::TypeKind::DataView
            | webidl::ast::TypeKind::Error
            | webidl::ast::TypeKind::FrozenArray(_)
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
        overloaded: bool,
        same_argument_names: bool,
        arguments: I,
        mut ret: Option<syn::Type>,
        kind: backend::ast::ImportFunctionKind,
        structural: bool,
        catch: bool,
        doc_comment: Option<String>,
    ) -> Option<backend::ast::ImportFunction>
    where
        I: Iterator<Item = (&'b str, &'b webidl::ast::Type, bool)>,
    {
        let arguments: Vec<_> = arguments.collect();
        let rust_name = rust_ident(
            &if overloaded && !arguments.is_empty() {
                let argument_type_names = arguments
                    .iter()
                    .map(|&(name, ty, variadic)| {
                        if same_argument_names {
                            if variadic {
                                "variadic_".to_owned() + &ty.type_to_string()
                            } else {
                                ty.type_to_string()
                            }
                        } else {
                            name.to_snake_case()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("_and_");
                if name == "new" {
                    "with_".to_owned() + &argument_type_names
                } else {
                    name.to_snake_case() + "_with_" + &argument_type_names
                }
            } else {
                name.to_snake_case()
            }
        );
        let name = name.to_string();

        let arguments = self.webidl_arguments_to_syn_arg_captured(arguments.into_iter(), &kind)?;

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
            doc_comment,
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
        let (overloaded, same_argument_names) = self.get_operation_overloading(
            arguments,
            ::first_pass::OperationId::Operation(name.cloned()),
            self_name,
        );

        let name = match name {
            None => {
                warn!("Operations without a name are unsupported");
                return None;
            }
            Some(ref name) => name,
        };

        let kind = backend::ast::ImportFunctionKind::Method {
            class: self_name.to_string(),
            ty: ident_ty(rust_ident(camel_case_ident(&self_name).as_str())),
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
        let doc_comment = Some(format!("The `{}()` method\n\n{}", name, mdn_doc(self_name, Some(name))));

        self.create_function(
            &name,
            overloaded,
            same_argument_names,
            arguments
                .iter()
                .map(|arg| (&*arg.name, &*arg.type_, arg.variadic)),
            ret,
            kind,
            self
                .interfaces
                .get(self_name)
                .map(|interface_data| interface_data.global)
                .unwrap_or(false),
            catch,
            doc_comment,
        )
    }

    /// Whether operation is overloaded and
    /// whether there overloads with same argument names for given argument types
    pub fn get_operation_overloading(
        &self,
        arguments: &[webidl::ast::Argument],
        id: ::first_pass::OperationId,
        self_name: &str,
    ) -> (bool, bool) {
        self
            .interfaces
            .get(self_name)
            .map(|interface_data| {
                interface_data
                    .operations
                    .get(&id)
                    .map(|operation_data|
                        (
                            operation_data.overloaded,
                            *operation_data
                                .argument_names_same
                                .get(
                                    &arguments
                                        .iter()
                                        .map(|argument| argument.name.clone())
                                        .collect::<Vec<_>>()
                                )
                                .unwrap_or(&false)
                        )
                    )
                    .unwrap_or((false, false))
            })
            .unwrap_or((false, false))
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
            ty: ident_ty(rust_ident(camel_case_ident(&self_name).as_str())),
            kind: backend::ast::MethodKind::Operation(backend::ast::Operation {
                is_static,
                kind: backend::ast::OperationKind::Getter(Some(raw_ident(name))),
            }),
        };
        let doc_comment = Some(format!("The `{}` getter\n\n{}", name, mdn_doc(self_name, Some(name))));

        self.create_function(name, false, false, iter::empty(), ret, kind, is_structural, catch, doc_comment)
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
            ty: ident_ty(rust_ident(camel_case_ident(&self_name).as_str())),
            kind: backend::ast::MethodKind::Operation(backend::ast::Operation {
                is_static,
                kind: backend::ast::OperationKind::Setter(Some(raw_ident(name))),
            }),
        };
        let doc_comment = Some(format!("The `{}` setter\n\n{}", name, mdn_doc(self_name, Some(name))));

        self.create_function(
            &format!("set_{}", name),
            false,
            false,
            iter::once((name, ty, false)),
            None,
            kind,
            is_structural,
            catch,
            doc_comment,
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
