use std::char;
use std::str::Chars;

use ast::OperationKind;
use backend::ast;
use backend::util::{ident_ty, ShortHash};
use backend::Diagnostic;
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use shared;
use syn::spanned::Spanned;
use syn::{self, AttributeArgs};

use crate::{d, ParseAttr};

/// Conversion trait with context.
///
/// Used to convert syn tokens into an AST, that we can then use to generate glue code. The context
/// (`Ctx`) is used to pass in the attributes from the `#[wasm_bindgen]`, if needed.
trait ConvertToAst<Ctx> {
    /// What we are converting to.
    type Target;
    /// Convert into our target.
    ///
    /// Since this is used in a procedural macro, use panic to fail.
    fn convert(self, context: Ctx) -> Result<Self::Target, Diagnostic>;
}

impl<'a> ConvertToAst<d::ItemStruct> for &'a mut syn::ItemStruct {
    type Target = ast::Struct;

    fn convert(self, attrs: d::ItemStruct) -> Result<Self::Target, Diagnostic> {
        if self.generics.params.len() > 0 {
            bail_span!(
                self.generics,
                "structs with #[wasm_bindgen] cannot have lifetime or \
                 type parameters currently"
            );
        }
        let mut fields = Vec::new();
        let js_name = attrs
            .js_name
            .as_ref()
            .map(|s| s.value())
            .unwrap_or(self.ident.to_string());
        let is_inspectable = attrs.inspectable.is_present();
        let getter_with_clone = attrs.getter_with_clone.is_present();

        for (i, field) in self.fields.iter_mut().enumerate() {
            match field.vis {
                syn::Visibility::Public(..) => {}
                _ => continue,
            }
            let (js_field_name, member) = match &field.ident {
                Some(ident) => (ident.to_string(), syn::Member::Named(ident.clone())),
                None => (i.to_string(), syn::Member::Unnamed(i.into())),
            };

            let attrs = field.parse_contained_attr()?;
            if attrs.skip.is_present() {
                continue;
            }

            let js_field_name = match &attrs.js_name {
                Some(name) => name.value(),
                None => js_field_name,
            };

            let comments = extract_doc_comments(&field.attrs);
            let getter = shared::struct_field_get(&js_name, &js_field_name);
            let setter = shared::struct_field_set(&js_name, &js_field_name);

            fields.push(ast::StructField {
                rust_name: member,
                js_name: js_field_name,
                struct_name: self.ident.clone(),
                readonly: attrs.readonly.is_present(),
                ty: field.ty.clone(),
                getter: Ident::new(&getter, Span::call_site()),
                setter: Ident::new(&setter, Span::call_site()),
                comments,
                generate_typescript: !attrs.skip_typescript.is_present(),
                getter_with_clone: getter_with_clone || attrs.getter_with_clone.is_present(),
            });
        }
        let generate_typescript = !attrs.skip_typescript.is_present();
        let comments: Vec<String> = extract_doc_comments(&self.attrs);
        Ok(ast::Struct {
            rust_name: self.ident.clone(),
            js_name,
            fields,
            comments,
            is_inspectable,
            generate_typescript,
        })
    }
}

fn get_ty(mut ty: &syn::Type) -> &syn::Type {
    while let syn::Type::Group(g) = ty {
        ty = &g.elem;
    }

    ty
}

fn get_expr(mut expr: &syn::Expr) -> &syn::Expr {
    while let syn::Expr::Group(g) = expr {
        expr = &g.expr;
    }

    expr
}

impl<'a> ConvertToAst<(d::ForeignItemFn, &'a ast::ImportModule)> for syn::ForeignItemFn {
    type Target = ast::ImportKind;

    fn convert(
        self,
        (opts, module): (d::ForeignItemFn, &'a ast::ImportModule),
    ) -> Result<Self::Target, Diagnostic> {
        let wasm = function_from_decl(
            &self.sig.ident,
            &opts,
            self.sig.clone(),
            self.attrs.clone(),
            self.vis.clone(),
            false,
            None,
        )?
        .0;
        let catch = opts.catch.is_present();
        let variadic = opts.variadic.is_present();
        let js_ret = if catch {
            // TODO: this assumes a whole bunch:
            //
            // * The outer type is actually a `Result`
            // * The error type is a `JsValue`
            // * The actual type is the first type parameter
            //
            // should probably fix this one day...
            extract_first_ty_param(wasm.ret.as_ref())?
        } else {
            wasm.ret.clone()
        };

        let operation_kind = OperationKind::from(&opts);

        let kind = if opts.method.is_present() {
            let class = wasm.arguments.get(0).ok_or_else(|| {
                err_span!(self, "imported methods must have at least one argument")
            })?;
            let class = match get_ty(&class.ty) {
                syn::Type::Reference(syn::TypeReference {
                    mutability: None,
                    elem,
                    ..
                }) => &**elem,
                _ => bail_span!(
                    class.ty,
                    "first argument of method must be a shared reference"
                ),
            };
            let class_name = match get_ty(class) {
                syn::Type::Path(syn::TypePath {
                    qself: None,
                    ref path,
                }) => path,
                _ => bail_span!(class, "first argument of method must be a path"),
            };
            let class_name = extract_path_ident(class_name)?;
            let class_name = opts
                .js_class
                .as_ref()
                .map(|p| p.value())
                .unwrap_or_else(|| class_name.to_string());

            let kind = ast::MethodKind::Operation(ast::Operation {
                is_static: false,
                kind: operation_kind,
            });

            ast::ImportFunctionKind::Method {
                class: class_name,
                ty: class.clone(),
                kind,
            }
        } else if let Some(cls) = &opts.static_method_of {
            let class = opts
                .js_class
                .as_ref()
                .map(|p| p.value())
                .unwrap_or_else(|| cls.to_string());
            let ty = ident_ty(cls.clone());

            let kind = ast::MethodKind::Operation(ast::Operation {
                is_static: true,
                kind: operation_kind,
            });

            ast::ImportFunctionKind::Method { class, ty, kind }
        } else if opts.constructor.is_present() {
            let class = match js_ret {
                Some(ref ty) => ty,
                _ => bail_span!(self, "constructor returns must be bare types"),
            };
            let class_name = match get_ty(class) {
                syn::Type::Path(syn::TypePath {
                    qself: None,
                    ref path,
                }) => path,
                _ => bail_span!(self, "return value of constructor must be a bare path"),
            };
            let class_name = extract_path_ident(class_name)?;
            let class_name = opts
                .js_class
                .as_ref()
                .map(|p| p.value())
                .unwrap_or_else(|| class_name.to_string());

            ast::ImportFunctionKind::Method {
                class: class_name.to_string(),
                ty: class.clone(),
                kind: ast::MethodKind::Constructor,
            }
        } else {
            ast::ImportFunctionKind::Normal
        };

        let shim = {
            let ns = match kind {
                ast::ImportFunctionKind::Normal => (0, "n"),
                ast::ImportFunctionKind::Method { ref class, .. } => (1, &class[..]),
            };
            let data = (ns, &self.sig.ident, module);
            format!(
                "__wbg_{}_{}",
                wasm.name
                    .chars()
                    .filter(|c| c.is_ascii_alphanumeric())
                    .collect::<String>(),
                ShortHash(data)
            )
        };
        if opts.r#final.is_present() && opts.structural.is_present() {
            let msg = "cannot specify both `structural` and `final`";
            return Err(Diagnostic::span_error(opts.r#final.span(), msg));
        }
        let assert_no_shim = opts.assert_no_shim.is_present();
        let ret = ast::ImportKind::Function(ast::ImportFunction {
            function: wasm,
            assert_no_shim,
            kind,
            js_ret,
            catch,
            variadic,
            structural: opts.structural.is_present() || !opts.r#final.is_present(),
            rust_name: self.sig.ident.clone(),
            shim: Ident::new(&shim, Span::call_site()),
            doc_comment: None,
        });

        Ok(ret)
    }
}

impl ConvertToAst<d::ForeignItemType> for syn::ForeignItemType {
    type Target = ast::ImportKind;

    fn convert(self, attrs: d::ForeignItemType) -> Result<Self::Target, Diagnostic> {
        let js_name = attrs
            .js_name
            .as_ref()
            .map_or_else(|| self.ident.to_string(), |s| s.value());
        let typescript_type = attrs.typescript_type.as_ref().map(|s| s.value());
        let is_type_of = attrs.is_type_of.clone();
        let shim = format!("__wbg_instanceof_{}_{}", self.ident, ShortHash(&self.ident));
        let extends = attrs.extends.clone();
        let vendor_prefixes = attrs.vendor_prefix.clone();
        let no_deref = attrs.no_deref.is_present();
        Ok(ast::ImportKind::Type(ast::ImportType {
            vis: self.vis,
            attrs: self.attrs,
            doc_comment: None,
            instanceof_shim: shim,
            is_type_of,
            rust_name: self.ident,
            typescript_type,
            js_name,
            extends,
            vendor_prefixes,
            no_deref,
        }))
    }
}

impl<'a> ConvertToAst<(d::ForeignItemStatic, &'a ast::ImportModule)> for syn::ForeignItemStatic {
    type Target = ast::ImportKind;

    fn convert(
        self,
        (opts, module): (d::ForeignItemStatic, &'a ast::ImportModule),
    ) -> Result<Self::Target, Diagnostic> {
        if self.mutability.is_some() {
            bail_span!(self.mutability, "cannot import mutable globals yet")
        }

        let default_name = self.ident.to_string();
        let js_name = opts
            .js_name
            .as_ref()
            .map(|p| p.value())
            .unwrap_or_else(|| default_name.to_string());
        let shim = format!(
            "__wbg_static_accessor_{}_{}",
            self.ident,
            ShortHash((&js_name, module, &self.ident)),
        );
        Ok(ast::ImportKind::Static(ast::ImportStatic {
            ty: *self.ty,
            vis: self.vis,
            rust_name: self.ident.clone(),
            js_name,
            shim: Ident::new(&shim, Span::call_site()),
        }))
    }
}

impl ConvertToAst<d::ItemFn> for syn::ItemFn {
    type Target = ast::Function;

    fn convert(self, attrs: d::ItemFn) -> Result<Self::Target, Diagnostic> {
        match self.vis {
            syn::Visibility::Public(_) => {}
            _ => bail_span!(self, "can only #[wasm_bindgen] public functions"),
        }
        if self.sig.constness.is_some() {
            bail_span!(
                self.sig.constness,
                "can only #[wasm_bindgen] non-const functions"
            );
        }
        if self.sig.unsafety.is_some() {
            bail_span!(self.sig.unsafety, "can only #[wasm_bindgen] safe functions");
        }

        let ret = function_from_decl(
            &self.sig.ident,
            &attrs,
            self.sig.clone(),
            self.attrs,
            self.vis,
            false,
            None,
        )?;
        Ok(ret.0)
    }
}

/// Construct a function (and gets the self type if appropriate) for our AST from a syn function.
fn function_from_decl<O>(
    decl_name: &syn::Ident,
    opts: &O,
    sig: syn::Signature,
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    allow_self: bool,
    self_ty: Option<&Ident>,
) -> Result<(ast::Function, Option<ast::MethodSelf>), Diagnostic>
where
    for<'a> &'a O: Into<OperationKind>,
    O: d::fields::JsName + d::fields::SkipTypescript,
{
    if sig.variadic.is_some() {
        bail_span!(sig.variadic, "can't #[wasm_bindgen] variadic functions");
    }
    if sig.generics.params.len() > 0 {
        bail_span!(
            sig.generics,
            "can't #[wasm_bindgen] functions with lifetime or type parameters",
        );
    }

    assert_no_lifetimes(&sig)?;

    let syn::Signature { inputs, output, .. } = sig;

    let replace_self = |t: syn::Type| {
        let self_ty = match self_ty {
            Some(i) => i,
            None => return t,
        };
        let path = match get_ty(&t) {
            syn::Type::Path(syn::TypePath { qself: None, path }) => path.clone(),
            other => return other.clone(),
        };
        let new_path = if path.segments.len() == 1 && path.segments[0].ident == "Self" {
            self_ty.clone().into()
        } else {
            path
        };
        syn::Type::Path(syn::TypePath {
            qself: None,
            path: new_path,
        })
    };

    let mut method_self = None;
    let arguments = inputs
        .into_iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(mut c) => {
                c.ty = Box::new(replace_self(*c.ty));
                Some(c)
            }
            syn::FnArg::Receiver(r) => {
                if !allow_self {
                    panic!("arguments cannot be `self`")
                }
                assert!(method_self.is_none());
                if r.reference.is_none() {
                    method_self = Some(ast::MethodSelf::ByValue);
                } else if r.mutability.is_some() {
                    method_self = Some(ast::MethodSelf::RefMutable);
                } else {
                    method_self = Some(ast::MethodSelf::RefShared);
                }
                None
            }
        })
        .collect::<Vec<_>>();

    let ret = match output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ty) => Some(replace_self(*ty)),
    };

    let (name, name_span, renamed_via_js_name) = if let Some(js_name) = opts.js_name() {
        let prefix = match opts.into() {
            OperationKind::Setter(_) => "set_",
            _ => "",
        };
        (
            format!("{}{}", prefix, js_name.value()),
            js_name.span(),
            true,
        )
    } else {
        (decl_name.to_string(), decl_name.span(), false)
    };
    Ok((
        ast::Function {
            arguments,
            name_span,
            name,
            renamed_via_js_name,
            ret,
            rust_attrs: attrs,
            rust_vis: vis,
            r#async: sig.asyncness.is_some(),
            generate_typescript: !opts.skip_typescript().is_present(),
        },
        method_self,
    ))
}

pub(crate) trait MacroParse<Ctx> {
    /// Parse the contents of an object into our AST, with a context if necessary.
    ///
    /// The context is used to have access to the attributes on `#[wasm_bindgen]`, and to allow
    /// writing to the output `TokenStream`.
    fn macro_parse(self, program: &mut ast::Program, context: Ctx) -> Result<(), Diagnostic>;
}

impl<'a> MacroParse<(AttributeArgs, &'a mut TokenStream)> for syn::Item {
    fn macro_parse(
        self,
        program: &mut ast::Program,
        (args, tokens): (AttributeArgs, &'a mut TokenStream),
    ) -> Result<(), Diagnostic> {
        match self {
            syn::Item::Fn(mut f) => {
                let no_mangle = f
                    .attrs
                    .iter()
                    .enumerate()
                    .filter_map(|(i, m)| m.parse_meta().ok().map(|m| (i, m)))
                    .find(|(_, m)| m.path().is_ident("no_mangle"));
                match no_mangle {
                    Some((i, _)) => {
                        f.attrs.remove(i);
                    }
                    _ => {}
                }
                let comments = extract_doc_comments(&f.attrs);
                f.to_tokens(tokens);
                let opts = syn::ItemFn::parse_attr_args(&args)?;
                if opts.start.is_present() {
                    if f.sig.generics.params.len() > 0 {
                        bail_span!(&f.sig.generics, "the start function cannot have generics",);
                    }
                    if f.sig.inputs.len() > 0 {
                        bail_span!(&f.sig.inputs, "the start function cannot have arguments",);
                    }
                }
                let method_kind = ast::MethodKind::Operation(ast::Operation {
                    is_static: true,
                    kind: OperationKind::from(&opts),
                });
                let rust_name = f.sig.ident.clone();
                let start = opts.start.is_present();
                program.exports.push(ast::Export {
                    comments,
                    function: f.convert(opts)?,
                    js_class: None,
                    method_kind,
                    method_self: None,
                    rust_class: None,
                    rust_name,
                    start,
                });
            }
            syn::Item::Struct(mut s) => {
                let opts = syn::ItemStruct::parse_attr_args(&args)?;
                program.structs.push((&mut s).convert(opts)?);
                s.to_tokens(tokens);
            }
            syn::Item::Impl(mut i) => {
                let opts = syn::ItemImpl::parse_attr_args(&args)?;
                (&mut i).macro_parse(program, opts)?;
                i.to_tokens(tokens);
            }
            syn::Item::ForeignMod(f) => {
                let opts = syn::ItemForeignMod::parse_attr_args(&args)?;
                f.macro_parse(program, opts)?;
            }
            syn::Item::Enum(e) => {
                let opts = syn::ItemEnum::parse_attr_args(&args)?;
                e.macro_parse(program, (tokens, opts))?;
            }
            syn::Item::Const(c) => {
                let opts = syn::ItemConst::parse_attr_args(&args)?;
                c.macro_parse(program, opts)?;
            }
            _ => {
                bail_span!(
                    self,
                    "#[wasm_bindgen] can only be applied to a function, \
                     struct, enum, impl, or extern block",
                );
            }
        }

        Ok(())
    }
}

impl<'a> MacroParse<d::ItemImpl> for &'a mut syn::ItemImpl {
    fn macro_parse(self, _program: &mut ast::Program, opts: d::ItemImpl) -> Result<(), Diagnostic> {
        if self.defaultness.is_some() {
            bail_span!(
                self.defaultness,
                "#[wasm_bindgen] default impls are not supported"
            );
        }
        if self.unsafety.is_some() {
            bail_span!(
                self.unsafety,
                "#[wasm_bindgen] unsafe impls are not supported"
            );
        }
        if let Some((_, path, _)) = &self.trait_ {
            bail_span!(path, "#[wasm_bindgen] trait impls are not supported");
        }
        if self.generics.params.len() > 0 {
            bail_span!(
                self.generics,
                "#[wasm_bindgen] generic impls aren't supported"
            );
        }
        let name = match get_ty(&self.self_ty) {
            syn::Type::Path(syn::TypePath {
                qself: None,
                ref path,
            }) => path,
            _ => bail_span!(
                self.self_ty,
                "unsupported self type in #[wasm_bindgen] impl"
            ),
        };
        let mut errors = Vec::new();
        for item in self.items.iter_mut() {
            if let Err(e) = prepare_for_impl_recursion(item, &name, &opts) {
                errors.push(e);
            }
        }
        Diagnostic::from_vec(errors)?;
        Ok(())
    }
}

// Prepare for recursion into an `impl` block. Here we want to attach an
// internal attribute, `__wasm_bindgen_class_marker`, with any metadata we need
// to pass from the impl to the impl item. Recursive macro expansion will then
// expand the `__wasm_bindgen_class_marker` attribute.
//
// Note that we currently do this because inner items may have things like cfgs
// on them, so we want to expand the impl first, let the insides get cfg'd, and
// then go for the rest.
fn prepare_for_impl_recursion(
    item: &mut syn::ImplItem,
    class: &syn::Path,
    impl_opts: &d::ItemImpl,
) -> Result<(), Diagnostic> {
    let method = match item {
        syn::ImplItem::Method(m) => m,
        syn::ImplItem::Const(_) => {
            bail_span!(
                &*item,
                "const definitions aren't supported with #[wasm_bindgen]"
            );
        }
        syn::ImplItem::Type(_) => bail_span!(
            &*item,
            "type definitions in impls aren't supported with #[wasm_bindgen]"
        ),
        syn::ImplItem::Macro(_) => {
            // In theory we want to allow this, but we have no way of expanding
            // the macro and then placing our magical attributes on the expanded
            // functions. As a result, just disallow it for now to hopefully
            // ward off buggy results from this macro.
            bail_span!(&*item, "macros in impls aren't supported");
        }
        syn::ImplItem::Verbatim(_) => panic!("unparsed impl item?"),
        other => bail_span!(other, "failed to parse this item as a known item"),
    };

    let ident = extract_path_ident(class)?;

    let js_class = impl_opts
        .js_class
        .as_ref()
        .map(|s| s.value())
        .unwrap_or(ident.to_string());

    method.attrs.insert(
        0,
        syn::Attribute {
            pound_token: Default::default(),
            style: syn::AttrStyle::Outer,
            bracket_token: Default::default(),
            path: syn::parse_quote! { wasm_bindgen::prelude::__wasm_bindgen_class_marker },
            tokens: quote::quote! { (#class = #js_class) }.into(),
        },
    );

    Ok(())
}

impl<'a, 'b> MacroParse<(&'a Ident, &'a str)> for &'b mut syn::ImplItemMethod {
    fn macro_parse(
        self,
        program: &mut ast::Program,
        (class, js_class): (&'a Ident, &'a str),
    ) -> Result<(), Diagnostic> {
        match self.vis {
            syn::Visibility::Public(_) => {}
            _ => return Ok(()),
        }
        if self.defaultness.is_some() {
            panic!("default methods are not supported");
        }
        if self.sig.constness.is_some() {
            bail_span!(
                self.sig.constness,
                "can only #[wasm_bindgen] non-const functions",
            );
        }
        if self.sig.unsafety.is_some() {
            bail_span!(self.sig.unsafety, "can only bindgen safe functions",);
        }

        let opts = self.parse_contained_attr()?;
        let comments = extract_doc_comments(&self.attrs);
        let (function, method_self) = function_from_decl(
            &self.sig.ident,
            &opts,
            self.sig.clone(),
            self.attrs.clone(),
            self.vis.clone(),
            true,
            Some(class),
        )?;
        let method_kind = if opts.constructor.is_present() {
            ast::MethodKind::Constructor
        } else {
            let is_static = method_self.is_none();
            let kind = OperationKind::from(&opts);
            ast::MethodKind::Operation(ast::Operation { is_static, kind })
        };
        program.exports.push(ast::Export {
            comments,
            function,
            js_class: Some(js_class.to_string()),
            method_kind,
            method_self,
            rust_class: Some(class.clone()),
            rust_name: self.sig.ident.clone(),
            start: false,
        });
        Ok(())
    }
}

fn import_enum(enum_: syn::ItemEnum, program: &mut ast::Program) -> Result<(), Diagnostic> {
    let mut variants = vec![];
    let mut variant_values = vec![];

    for v in enum_.variants.iter() {
        match v.fields {
            syn::Fields::Unit => (),
            _ => bail_span!(v.fields, "only C-Style enums allowed with #[wasm_bindgen]"),
        }

        let (_, expr) = match &v.discriminant {
            Some(pair) => pair,
            None => {
                bail_span!(v, "all variants must have a value");
            }
        };
        match get_expr(expr) {
            syn::Expr::Lit(syn::ExprLit {
                attrs: _,
                lit: syn::Lit::Str(str_lit),
            }) => {
                variants.push(v.ident.clone());
                variant_values.push(str_lit.value());
            }
            expr => bail_span!(
                expr,
                "enums with #[wasm_bindgen] cannot mix string and non-string values",
            ),
        }
    }

    program.imports.push(ast::Import {
        module: ast::ImportModule::None,
        js_namespace: None,
        kind: ast::ImportKind::Enum(ast::ImportEnum {
            vis: enum_.vis,
            name: enum_.ident,
            variants,
            variant_values,
            rust_attrs: enum_.attrs,
        }),
    });

    Ok(())
}

impl<'a> MacroParse<(&'a mut TokenStream, d::ItemEnum)> for syn::ItemEnum {
    fn macro_parse(
        self,
        program: &mut ast::Program,
        (tokens, opts): (&'a mut TokenStream, d::ItemEnum),
    ) -> Result<(), Diagnostic> {
        if self.variants.len() == 0 {
            bail_span!(self, "cannot export empty enums to JS");
        }
        let generate_typescript = !opts.skip_typescript.is_present();

        // Check if the first value is a string literal
        if let Some((_, expr)) = &self.variants[0].discriminant {
            match get_expr(expr) {
                syn::Expr::Lit(syn::ExprLit {
                    attrs: _,
                    lit: syn::Lit::Str(_),
                }) => {
                    return import_enum(self, program);
                }
                _ => {}
            }
        }
        let js_name = opts
            .js_name
            .as_ref()
            .map_or_else(|| self.ident.to_string(), |s| s.value());

        let has_discriminant = self.variants[0].discriminant.is_some();

        match self.vis {
            syn::Visibility::Public(_) => {}
            _ => bail_span!(self, "only public enums are allowed with #[wasm_bindgen]"),
        }

        let variants = self
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| {
                match v.fields {
                    syn::Fields::Unit => (),
                    _ => bail_span!(v.fields, "only C-Style enums allowed with #[wasm_bindgen]"),
                }

                // Require that everything either has a discriminant or doesn't.
                // We don't really want to get in the business of emulating how
                // rustc assigns values to enums.
                if v.discriminant.is_some() != has_discriminant {
                    bail_span!(
                        v,
                        "must either annotate discriminant of all variants or none"
                    );
                }

                let value = match &v.discriminant {
                    Some((_, expr)) => match get_expr(expr) {
                        syn::Expr::Lit(syn::ExprLit {
                            attrs: _,
                            lit: syn::Lit::Int(int_lit),
                        }) => match int_lit.base10_digits().parse::<u32>() {
                            Ok(v) => v,
                            Err(_) => {
                                bail_span!(
                                    int_lit,
                                    "enums with #[wasm_bindgen] can only support \
                                 numbers that can be represented as u32"
                                );
                            }
                        },
                        expr => bail_span!(
                            expr,
                            "enums with #[wasm_bindgen] may only have \
                             number literal values",
                        ),
                    },
                    None => i as u32,
                };

                let comments = extract_doc_comments(&v.attrs);
                Ok(ast::Variant {
                    name: v.ident.clone(),
                    value,
                    comments,
                })
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;

        let mut values = variants.iter().map(|v| v.value).collect::<Vec<_>>();
        values.sort();
        let hole = values
            .windows(2)
            .filter_map(|window| {
                if window[0] + 1 != window[1] {
                    Some(window[0] + 1)
                } else {
                    None
                }
            })
            .next()
            .unwrap_or(*values.last().unwrap() + 1);
        for value in values {
            assert!(hole != value);
        }

        let comments = extract_doc_comments(&self.attrs);

        self.to_tokens(tokens);

        program.enums.push(ast::Enum {
            rust_name: self.ident,
            js_name,
            variants,
            comments,
            hole,
            generate_typescript,
        });
        Ok(())
    }
}

impl MacroParse<d::ItemConst> for syn::ItemConst {
    fn macro_parse(
        self,
        program: &mut ast::Program,
        _opts: d::ItemConst,
    ) -> Result<(), Diagnostic> {
        match get_expr(&self.expr) {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(litstr),
                ..
            }) => {
                program.typescript_custom_sections.push(litstr.value());
            }
            expr => {
                bail_span!(expr, "Expected a string literal to be used with #[wasm_bindgen(typescript_custom_section)].");
            }
        }

        Ok(())
    }
}

impl MacroParse<d::ForeignMod> for syn::ItemForeignMod {
    fn macro_parse(
        self,
        program: &mut ast::Program,
        opts: d::ForeignMod,
    ) -> Result<(), Diagnostic> {
        let mut errors = Vec::new();
        match self.abi.name {
            Some(ref l) if l.value() == "C" => {}
            None => {}
            Some(ref other) => {
                errors.push(err_span!(
                    other,
                    "only foreign mods with the `C` ABI are allowed"
                ));
            }
        }

        // This expects only one branch to be taken as a result of validation in d::ForeignMod::from_attributes
        let module = if let Some(name) = &opts.module {
            ast::ImportModule::Named(name.value(), name.span())
        } else if let Some(name) = &opts.raw_module {
            ast::ImportModule::RawNamed(name.value(), name.span())
        } else if let Some(js) = &opts.inline_js {
            let i = program.inline_js.len();
            program.inline_js.push(js.value());
            ast::ImportModule::Inline(i, js.span())
        } else {
            ast::ImportModule::None
        };
        for item in self.items.into_iter() {
            if let Err(e) = item.macro_parse(program, module.clone()) {
                errors.push(e);
            }
        }
        Diagnostic::from_vec(errors)?;
        Ok(())
    }
}

impl MacroParse<ast::ImportModule> for syn::ForeignItem {
    fn macro_parse(
        self,
        program: &mut ast::Program,
        module: ast::ImportModule,
    ) -> Result<(), Diagnostic> {
        let (js_namespace, kind) = match self {
            syn::ForeignItem::Fn(mut f) => {
                let item_opts = f.parse_contained_attr()?;
                (
                    item_opts.js_namespace.as_ref().map(|v| v.to_strings()),
                    f.convert((item_opts, &module))?,
                )
            }
            syn::ForeignItem::Type(mut t) => {
                let item_opts = t.parse_contained_attr()?;
                (
                    item_opts.js_namespace.as_ref().map(|v| v.to_strings()),
                    t.convert(item_opts)?,
                )
            }
            syn::ForeignItem::Static(mut s) => {
                let item_opts = s.parse_contained_attr()?;
                (
                    item_opts.js_namespace.as_ref().map(|v| v.to_strings()),
                    s.convert((item_opts, &module))?,
                )
            }
            _ => panic!("only foreign functions/types allowed for now"),
        };

        program.imports.push(ast::Import {
            module,
            js_namespace,
            kind,
        });

        Ok(())
    }
}

/// Get the first type parameter of a generic type, errors on incorrect input.
fn extract_first_ty_param(ty: Option<&syn::Type>) -> Result<Option<syn::Type>, Diagnostic> {
    let t = match ty {
        Some(t) => t,
        None => return Ok(None),
    };
    let path = match *get_ty(&t) {
        syn::Type::Path(syn::TypePath {
            qself: None,
            ref path,
        }) => path,
        _ => bail_span!(t, "must be Result<...>"),
    };
    let seg = path
        .segments
        .last()
        .ok_or_else(|| err_span!(t, "must have at least one segment"))?;
    let generics = match seg.arguments {
        syn::PathArguments::AngleBracketed(ref t) => t,
        _ => bail_span!(t, "must be Result<...>"),
    };
    let generic = generics
        .args
        .first()
        .ok_or_else(|| err_span!(t, "must have at least one generic parameter"))?;
    let ty = match generic {
        syn::GenericArgument::Type(t) => t,
        other => bail_span!(other, "must be a type parameter"),
    };
    match get_ty(&ty) {
        syn::Type::Tuple(t) if t.elems.len() == 0 => return Ok(None),
        _ => {}
    }
    Ok(Some(ty.clone()))
}

/// Extract the documentation comments from a Vec of attributes
fn extract_doc_comments(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|a| {
            // if the path segments include an ident of "doc" we know this
            // this is a doc comment
            if a.path.segments.iter().any(|s| s.ident.to_string() == "doc") {
                Some(
                    // We want to filter out any Puncts so just grab the Literals
                    a.tokens.clone().into_iter().filter_map(|t| match t {
                        TokenTree::Literal(lit) => {
                            let quoted = lit.to_string();
                            Some(try_unescape(&quoted).unwrap_or_else(|| quoted))
                        }
                        _ => None,
                    }),
                )
            } else {
                None
            }
        })
        //Fold up the [[String]] iter we created into Vec<String>
        .fold(vec![], |mut acc, a| {
            acc.extend(a);
            acc
        })
}

// Unescapes a quoted string. char::escape_debug() was used to escape the text.
fn try_unescape(s: &str) -> Option<String> {
    if s.is_empty() {
        return Some(String::new());
    }
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    for i in 0.. {
        let c = match chars.next() {
            Some(c) => c,
            None => {
                if result.ends_with('"') {
                    result.pop();
                }
                return Some(result);
            }
        };
        if i == 0 && c == '"' {
            // ignore it
        } else if c == '\\' {
            let c = chars.next()?;
            match c {
                't' => result.push('\t'),
                'r' => result.push('\r'),
                'n' => result.push('\n'),
                '\\' | '\'' | '"' => result.push(c),
                'u' => {
                    if chars.next() != Some('{') {
                        return None;
                    }
                    let (c, next) = unescape_unicode(&mut chars)?;
                    result.push(c);
                    if next != '}' {
                        return None;
                    }
                }
                _ => return None,
            }
        } else {
            result.push(c);
        }
    }
    None
}

fn unescape_unicode(chars: &mut Chars) -> Option<(char, char)> {
    let mut value = 0;
    for i in 0..7 {
        let c = chars.next()?;
        let num = if c >= '0' && c <= '9' {
            c as u32 - '0' as u32
        } else if c >= 'a' && c <= 'f' {
            c as u32 - 'a' as u32 + 10
        } else if c >= 'A' && c <= 'F' {
            c as u32 - 'A' as u32 + 10
        } else {
            if i == 0 {
                return None;
            }
            let decoded = char::from_u32(value)?;
            return Some((decoded, c));
        };
        if i >= 6 {
            return None;
        }
        value = (value << 4) | num;
    }
    None
}

/// Check there are no lifetimes on the function.
fn assert_no_lifetimes(sig: &syn::Signature) -> Result<(), Diagnostic> {
    struct Walk {
        diagnostics: Vec<Diagnostic>,
    }

    impl<'ast> syn::visit::Visit<'ast> for Walk {
        fn visit_lifetime(&mut self, i: &'ast syn::Lifetime) {
            self.diagnostics.push(err_span!(
                &*i,
                "it is currently not sound to use lifetimes in function \
                 signatures"
            ));
        }
    }
    let mut walk = Walk {
        diagnostics: Vec::new(),
    };
    syn::visit::Visit::visit_signature(&mut walk, sig);
    Diagnostic::from_vec(walk.diagnostics)
}

/// Extracts the last ident from the path
fn extract_path_ident(path: &syn::Path) -> Result<Ident, Diagnostic> {
    for segment in path.segments.iter() {
        match segment.arguments {
            syn::PathArguments::None => {}
            _ => bail_span!(path, "paths with type parameters are not supported yet"),
        }
    }

    match path.segments.last() {
        Some(value) => Ok(value.ident.clone()),
        None => {
            bail_span!(path, "empty idents are not supported");
        }
    }
}
