use std::iter::FromIterator;

use backend;
use backend::util::{leading_colon_path_ty, raw_ident, rust_ident};
use proc_macro2::Ident;
use syn;
use weedle;

use first_pass::FirstPassRecord;
use type_conversion::ToSynType;

pub(crate) fn public() -> syn::Visibility {
    syn::Visibility::Public(syn::VisPublic {
        pub_token: Default::default(),
    })
}

pub(crate) fn shared_ref(ty: syn::Type) -> syn::Type {
    syn::TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: Box::new(ty),
    }.into()
}

pub(crate) fn unit_ty() -> syn::Type {
    syn::Type::Tuple(syn::TypeTuple {
        paren_token: Default::default(),
        elems: syn::punctuated::Punctuated::new(),
    })
}

pub(crate) fn js_value() -> syn::Type {
    leading_colon_path_ty(vec![rust_ident("wasm_bindgen"), rust_ident("JsValue")])
}

pub(crate) fn rust_type_name(identifier: &weedle::common::Identifier) -> String {
    use heck::CamelCase;
    identifier.name.to_camel_case()
}

pub(crate) fn rust_type_ident(identifier: &weedle::common::Identifier) -> Ident {
    rust_ident(&rust_type_name(identifier))
}

pub(crate) fn rust_variable_name(identifier: &weedle::common::Identifier) -> String {
    use heck::SnakeCase;
    identifier.name.to_snake_case()
}

pub(crate) fn rust_variable_ident(identifier: &weedle::common::Identifier) -> Ident {
    rust_ident(&rust_variable_name(identifier))
}

pub(crate) fn rust_const_name(identifier: &weedle::common::Identifier) -> String {
    use heck::ShoutySnakeCase;
    identifier.name.to_shouty_snake_case()
}

pub(crate) fn rust_const_ident(identifier: &weedle::common::Identifier) -> Ident {
    rust_ident(&rust_const_name(identifier))
}

pub fn weedle_const_v_to_backend_const_v(
    value: &weedle::literal::ConstValue,
) -> backend::ast::ConstValue {
    use weedle::{
        literal::{BooleanLit, ConstValue::*, DecI64, FloatLit::*, HexI64, IntegerLit::*, OctI64},
        term,
    };

    match value {
        Boolean(BooleanLit(boolean)) => backend::ast::ConstValue::Boolean(*boolean),
        Float(Value(value)) => {
            backend::ast::ConstValue::Float(backend::ast::FloatValue::Literal(value.to_string()))
        }
        Float(NegInfinity(term::NegInfinity)) => {
            backend::ast::ConstValue::Float(backend::ast::FloatValue::NegInfinity)
        }
        Float(Infinity(term::Infinity)) => {
            backend::ast::ConstValue::Float(backend::ast::FloatValue::Infinity)
        }
        Float(NaN(term::NaN)) => backend::ast::ConstValue::Float(backend::ast::FloatValue::NaN),
        Integer(Dec(DecI64(dec_i64))) => backend::ast::ConstValue::Integer(
            backend::ast::IntegerValue::Literal(dec_i64.to_string()),
        ),
        Integer(Hex(HexI64(hex_i64))) => backend::ast::ConstValue::Integer(
            backend::ast::IntegerValue::Literal(hex_i64.to_string()),
        ),
        Integer(Oct(OctI64(oct_i64))) => {
            let literal = if oct_i64 == "0" || oct_i64 == "-0" {
                oct_i64.to_string()
            } else if oct_i64.starts_with('-') {
                format!("-0o{}", &oct_i64[2..])
            } else {
                format!("0o{}", &oct_i64[1..])
            };
            backend::ast::ConstValue::Integer(backend::ast::IntegerValue::Literal(literal))
        }
        Null(weedle::term::Null) => backend::ast::ConstValue::Null,
    }
}

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

fn result_ty(t: syn::Type) -> syn::Type {
    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![
            syn::GenericArgument::Type(t),
            syn::GenericArgument::Type(js_value()),
        ]),
        gt_token: Default::default(),
    });

    let ident = raw_ident("Result");
    let seg = syn::PathSegment { ident, arguments };
    let path: syn::Path = seg.into();
    let ty = syn::TypePath { qself: None, path };
    ty.into()
}

impl<'a> FirstPassRecord<'a> {
    fn weedle_arguments_to_syn_arg_captured<'b, I>(
        &self,
        arguments: I,
        kind: &backend::ast::ImportFunctionKind,
    ) -> Option<Vec<syn::ArgCaptured>>
    where
        I: IntoIterator<Item = &'b weedle::argument::Argument>,
    {
        let arguments = arguments.into_iter();
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

        for arg in arguments {
            use weedle::argument::Argument::*;
            match arg {
                Single(arg) => {
                    // TODO: deal with arg.{attributes, optional, default}
                    match arg.type_.to_syn_type(self, false) {
                        None => return None,
                        Some(ty) => {
                            res.push(simple_fn_arg(rust_variable_ident(&arg.identifier), ty))
                        }
                    }
                }
                Variadic(_) => {
                    warn!("Variadic arguments are not supported yet");
                    return None;
                }
            }
        }

        Some(res)
    }

    pub fn create_function<'b, I, T>(
        &self,
        identifier: &weedle::common::Identifier,
        arguments: I,
        ret: Option<T>,
        kind: backend::ast::ImportFunctionKind,
        structural: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction>
    where
        I: IntoIterator<Item = &'b weedle::argument::Argument>,
        T: ToSynType,
    {
        let rust_name = rust_variable_ident(identifier);
        let name = raw_ident(&identifier.name);

        let arguments = self.weedle_arguments_to_syn_arg_captured(arguments, &kind)?;
        let mut ret = match ret {
            Some(ret) => Some(ret.to_syn_type(self, true)?),
            None => None,
        };

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

    pub fn create_method<T>(
        &self,
        identifier: Option<&weedle::common::Identifier>,
        arguments: &[weedle::argument::Argument],
        return_type: Option<T>,
        self_identifier: &weedle::common::Identifier,
        is_static: bool,
        kind: backend::ast::OperationKind,
        is_structural: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction>
    where
        T: ToSynType,
    {
        let identifier = match identifier {
            None => {
                warn!("Operations without a name are unsupported");
                return None;
            }
            Some(identifier) => identifier,
        };

        let kind = backend::ast::ImportFunctionKind::Method {
            class: self_identifier.name.to_string(),
            ty: self_identifier.to_syn_type(self, true)?,
            kind: backend::ast::MethodKind::Operation(backend::ast::Operation { is_static, kind }),
        };

        self.create_function(
            identifier,
            arguments,
            return_type,
            kind,
            is_structural,
            catch,
        )
    }

    pub fn create_regular_method(
        &self,
        identifier: &Option<weedle::common::Identifier>,
        arguments: &[weedle::argument::Argument],
        return_type: &weedle::types::ReturnType,
        self_identifier: &weedle::common::Identifier,
        is_static: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction> {
        let return_type = match return_type {
            weedle::types::ReturnType::Void(weedle::term::Void) => None,
            weedle::types::ReturnType::Type(type_) => Some(type_),
        };
        self.create_method(
            identifier.as_ref(),
            arguments,
            return_type,
            self_identifier,
            is_static,
            backend::ast::OperationKind::Regular,
            false,
            catch,
        )
    }

    pub fn create_getter(
        &self,
        identifier: &weedle::common::Identifier,
        ty: &weedle::types::AttributedType,
        self_identifier: &weedle::common::Identifier,
        is_static: bool,
        is_structural: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction> {
        self.create_method(
            Some(identifier),
            &[],
            Some(ty),
            self_identifier,
            is_static,
            backend::ast::OperationKind::Getter(Some(raw_ident(&identifier.name))),
            is_structural,
            catch,
        )
    }

    pub fn create_setter(
        &self,
        identifier: &weedle::common::Identifier,
        ty: &weedle::types::AttributedType,
        self_identifier: &weedle::common::Identifier,
        is_static: bool,
        is_structural: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction> {
        enum AlwaysVoidReturn {}

        impl ToSynType for AlwaysVoidReturn {
            fn to_syn_type(&self, _: &FirstPassRecord<'_>, _: bool) -> Option<syn::Type> {
                match *self {}
            }
        }

        const RETURN_TYPE: Option<AlwaysVoidReturn> = None;

        self.create_method(
            Some(&weedle::common::Identifier {
                name: format!("set_{}", identifier.name),
            }),
            &[weedle::argument::Argument::Single(
                weedle::argument::SingleArgument {
                    attributes: None,
                    optional: None,
                    type_: ty.clone(),
                    identifier: identifier.clone(),
                    default: None,
                },
            )],
            RETURN_TYPE,
            self_identifier,
            is_static,
            backend::ast::OperationKind::Setter(Some(raw_ident(&identifier.name))),
            is_structural,
            catch,
        )
    }
}

mod extended_attribute_helpers {
    use weedle::attribute::{ExtendedAttribute, ExtendedAttributeList};

    fn search<F>(attrs: &Option<ExtendedAttributeList>, f: F) -> bool
    where
        F: FnMut(&ExtendedAttribute) -> bool,
    {
        attrs
            .as_ref()
            .map(|attrs| attrs.body.list.iter().any(f))
            .unwrap_or(false)
    }

    fn has_named_attribute(attrs: &Option<ExtendedAttributeList>, attribute: &str) -> bool {
        search(attrs, |attr| match attr {
            ExtendedAttribute::NoArgs(attr) => attr.identifier.name == attribute,
            _ => false,
        })
    }

    /// ChromeOnly is for things that are only exposed to priveleged code in Firefox.
    pub(crate) fn is_chrome_only(attrs: &Option<ExtendedAttributeList>) -> bool {
        has_named_attribute(attrs, "ChromeOnly")
    }

    pub fn is_no_interface_object(attrs: &Option<ExtendedAttributeList>) -> bool {
        has_named_attribute(attrs, "NoInterfaceObject")
    }

    pub(crate) fn is_structural(attrs: &Option<ExtendedAttributeList>) -> bool {
        has_named_attribute(attrs, "Unforgeable")
    }

    pub(crate) fn throws(attrs: &Option<ExtendedAttributeList>) -> bool {
        has_named_attribute(attrs, "Throws")
    }
}

pub(crate) use self::extended_attribute_helpers::*;
