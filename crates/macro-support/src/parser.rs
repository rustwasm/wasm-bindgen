use backend::ast;
use backend::util::{ident_ty, ShortHash};
use backend::Diagnostic;
use proc_macro2::{Delimiter, Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use shared;
use syn;
use syn::parse::{Parse, ParseStream, Result as SynResult};

/// Parsed attributes from a `#[wasm_bindgen(..)]`.
#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Default)]
pub struct BindgenAttrs {
    /// List of parsed attributes
    pub attrs: Vec<BindgenAttr>,
}

impl BindgenAttrs {
    /// Find and parse the wasm_bindgen attributes.
    fn find(attrs: &mut Vec<syn::Attribute>) -> Result<BindgenAttrs, Diagnostic> {
        let pos = attrs
            .iter()
            .enumerate()
            .find(|&(_, ref m)| m.path.segments[0].ident == "wasm_bindgen")
            .map(|a| a.0);
        let pos = match pos {
            Some(i) => i,
            None => return Ok(BindgenAttrs::default()),
        };
        let attr = attrs.remove(pos);
        let mut tts = attr.tts.clone().into_iter();
        let group = match tts.next() {
            Some(TokenTree::Group(d)) => d,
            Some(_) => bail_span!(attr, "malformed #[wasm_bindgen] attribute"),
            None => return Ok(BindgenAttrs::default()),
        };
        if tts.next().is_some() {
            bail_span!(attr, "malformed #[wasm_bindgen] attribute");
        }
        if group.delimiter() != Delimiter::Parenthesis {
            bail_span!(attr, "malformed #[wasm_bindgen] attribute");
        }
        Ok(syn::parse2(group.stream())?)
    }

    /// Get the first module attribute
    fn module(&self) -> Option<&str> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::Module(s) => Some(&s[..]),
                _ => None,
            })
            .next()
    }

    /// Whether the catch attribute is present
    fn catch(&self) -> bool {
        self.attrs.iter().any(|a| match a {
            BindgenAttr::Catch => true,
            _ => false,
        })
    }

    /// Whether the constructor attribute is present
    fn constructor(&self) -> bool {
        self.attrs.iter().any(|a| match a {
            BindgenAttr::Constructor => true,
            _ => false,
        })
    }

    /// Get the first static_method_of attribute
    fn static_method_of(&self) -> Option<&Ident> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::StaticMethodOf(c) => Some(c),
                _ => None,
            })
            .next()
    }

    /// Whether the method attributes is present
    fn method(&self) -> bool {
        self.attrs.iter().any(|a| match a {
            BindgenAttr::Method => true,
            _ => false,
        })
    }

    /// Get the first js_namespace attribute
    fn js_namespace(&self) -> Option<&Ident> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::JsNamespace(s) => Some(s),
                _ => None,
            })
            .next()
    }

    /// Get the first getter attribute
    fn getter(&self) -> Option<Option<Ident>> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::Getter(g) => Some(g.clone()),
                _ => None,
            })
            .next()
    }

    /// Get the first setter attribute
    fn setter(&self) -> Option<Option<Ident>> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::Setter(s) => Some(s.clone()),
                _ => None,
            })
            .next()
    }

    /// Whether the indexing getter attributes is present
    fn indexing_getter(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::IndexingGetter => true,
            _ => false,
        })
    }

    /// Whether the indexing setter attributes is present
    fn indexing_setter(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::IndexingSetter => true,
            _ => false,
        })
    }

    /// Whether the indexing deleter attributes is present
    fn indexing_deleter(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::IndexingDeleter => true,
            _ => false,
        })
    }

    /// Whether the structural attributes is present
    fn structural(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::Structural => true,
            _ => false,
        })
    }

    /// Whether the `final` attribute is present
    fn final_(&self) -> Option<&Ident> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::Final(i) => Some(i),
                _ => None,
            })
            .next()
    }

    /// Whether the readonly attributes is present
    fn readonly(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::Readonly => true,
            _ => false,
        })
    }

    /// Get the first js_name attribute
    fn js_name(&self) -> Option<(&str, Span)> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::JsName(s, span) => Some((&s[..], *span)),
                _ => None,
            })
            .next()
    }

    /// Get the first js_class attribute
    fn js_class(&self) -> Option<&str> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::JsClass(s) => Some(&s[..]),
                _ => None,
            })
            .next()
    }

    /// Return the list of classes that a type extends
    fn extends(&self) -> impl Iterator<Item = &syn::Path> {
        self.attrs.iter().filter_map(|a| match a {
            BindgenAttr::Extends(s) => Some(s),
            _ => None,
        })
    }

    fn vendor_prefixes(&self) -> impl Iterator<Item = &Ident> {
        self.attrs.iter().filter_map(|a| match a {
            BindgenAttr::VendorPrefix(s) => Some(s),
            _ => None,
        })
    }

    /// Whether the variadic attributes is present
    fn variadic(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::Variadic => true,
            _ => false,
        })
    }

    fn typescript_custom_section(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::TypescriptCustomSection => true,
            _ => false,
        })
    }
}

impl Parse for BindgenAttrs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.is_empty() {
            return Ok(BindgenAttrs { attrs: Vec::new() });
        }

        let opts = syn::punctuated::Punctuated::<_, syn::token::Comma>::parse_terminated(input)?;
        Ok(BindgenAttrs {
            attrs: opts.into_iter().collect(),
        })
    }
}

/// The possible attributes in the `#[wasm_bindgen]`.
#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub enum BindgenAttr {
    Catch,
    Constructor,
    Method,
    StaticMethodOf(Ident),
    JsNamespace(Ident),
    Module(String),
    Getter(Option<Ident>),
    Setter(Option<Ident>),
    IndexingGetter,
    IndexingSetter,
    IndexingDeleter,
    Structural,
    Final(Ident),
    Readonly,
    JsName(String, Span),
    JsClass(String),
    Extends(syn::Path),
    VendorPrefix(Ident),
    Variadic,
    TypescriptCustomSection,
}

impl Parse for BindgenAttr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let original = input.fork();
        let attr: AnyIdent = input.parse()?;
        let attr = attr.0;
        if attr == "catch" {
            return Ok(BindgenAttr::Catch);
        }
        if attr == "constructor" {
            return Ok(BindgenAttr::Constructor);
        }
        if attr == "method" {
            return Ok(BindgenAttr::Method);
        }
        if attr == "indexing_getter" {
            return Ok(BindgenAttr::IndexingGetter);
        }
        if attr == "indexing_setter" {
            return Ok(BindgenAttr::IndexingSetter);
        }
        if attr == "indexing_deleter" {
            return Ok(BindgenAttr::IndexingDeleter);
        }
        if attr == "structural" {
            return Ok(BindgenAttr::Structural);
        }
        if attr == "final" {
            return Ok(BindgenAttr::Final(attr));
        }
        if attr == "readonly" {
            return Ok(BindgenAttr::Readonly);
        }
        if attr == "variadic" {
            return Ok(BindgenAttr::Variadic);
        }
        if attr == "static_method_of" {
            input.parse::<Token![=]>()?;
            return Ok(BindgenAttr::StaticMethodOf(input.parse::<AnyIdent>()?.0));
        }
        if attr == "getter" {
            if input.parse::<Token![=]>().is_ok() {
                return Ok(BindgenAttr::Getter(Some(input.parse::<AnyIdent>()?.0)));
            } else {
                return Ok(BindgenAttr::Getter(None));
            }
        }
        if attr == "setter" {
            if input.parse::<Token![=]>().is_ok() {
                return Ok(BindgenAttr::Setter(Some(input.parse::<AnyIdent>()?.0)));
            } else {
                return Ok(BindgenAttr::Setter(None));
            }
        }
        if attr == "js_namespace" {
            input.parse::<Token![=]>()?;
            return Ok(BindgenAttr::JsNamespace(input.parse::<AnyIdent>()?.0));
        }
        if attr == "extends" {
            input.parse::<Token![=]>()?;
            return Ok(BindgenAttr::Extends(input.parse()?));
        }
        if attr == "vendor_prefix" {
            input.parse::<Token![=]>()?;
            return Ok(BindgenAttr::VendorPrefix(input.parse::<AnyIdent>()?.0));
        }
        if attr == "module" {
            input.parse::<Token![=]>()?;
            return Ok(BindgenAttr::Module(input.parse::<syn::LitStr>()?.value()));
        }
        if attr == "js_class" {
            input.parse::<Token![=]>()?;
            let val = match input.parse::<syn::LitStr>() {
                Ok(str) => str.value(),
                Err(_) => input.parse::<AnyIdent>()?.0.to_string(),
            };
            return Ok(BindgenAttr::JsClass(val));
        }
        if attr == "js_name" {
            input.parse::<Token![=]>()?;
            let (val, span) = match input.parse::<syn::LitStr>() {
                Ok(str) => (str.value(), str.span()),
                Err(_) => {
                    let ident = input.parse::<AnyIdent>()?.0;
                    (ident.to_string(), ident.span())
                }
            };
            return Ok(BindgenAttr::JsName(val, span));
        }
        if attr == "typescript_custom_section" {
            return Ok(BindgenAttr::TypescriptCustomSection);
        }

        Err(original.error("unknown attribute"))
    }
}

struct AnyIdent(Ident);

impl Parse for AnyIdent {
    fn parse(input: ParseStream) -> SynResult<Self> {
        input.step(|cursor| match cursor.ident() {
            Some((ident, remaining)) => Ok((AnyIdent(ident), remaining)),
            None => Err(cursor.error("expected an identifier")),
        })
    }
}

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

impl<'a> ConvertToAst<BindgenAttrs> for &'a mut syn::ItemStruct {
    type Target = ast::Struct;

    fn convert(self, opts: BindgenAttrs) -> Result<Self::Target, Diagnostic> {
        if self.generics.params.len() > 0 {
            bail_span!(
                self.generics,
                "structs with #[wasm_bindgen] cannot have lifetime or \
                 type parameters currently"
            );
        }
        let mut fields = Vec::new();
        let js_name = opts
            .js_name()
            .map(|s| s.0.to_string())
            .unwrap_or(self.ident.to_string());
        if let syn::Fields::Named(names) = &mut self.fields {
            for field in names.named.iter_mut() {
                match field.vis {
                    syn::Visibility::Public(..) => {}
                    _ => continue,
                }
                let name = match &field.ident {
                    Some(n) => n,
                    None => continue,
                };
                let name_str = name.to_string();
                let getter = shared::struct_field_get(&js_name, &name_str);
                let setter = shared::struct_field_set(&js_name, &name_str);
                let opts = BindgenAttrs::find(&mut field.attrs)?;
                assert_not_variadic(&opts, &field)?;
                let comments = extract_doc_comments(&field.attrs);
                fields.push(ast::StructField {
                    name: name.clone(),
                    struct_name: self.ident.clone(),
                    readonly: opts.readonly(),
                    ty: field.ty.clone(),
                    getter: Ident::new(&getter, Span::call_site()),
                    setter: Ident::new(&setter, Span::call_site()),
                    comments,
                });
            }
        }
        let comments: Vec<String> = extract_doc_comments(&self.attrs);
        Ok(ast::Struct {
            rust_name: self.ident.clone(),
            js_name,
            fields,
            comments,
        })
    }
}

impl<'a> ConvertToAst<(BindgenAttrs, &'a Option<String>)> for syn::ForeignItemFn {
    type Target = ast::ImportKind;

    fn convert(
        self,
        (opts, module): (BindgenAttrs, &'a Option<String>),
    ) -> Result<Self::Target, Diagnostic> {
        let wasm = function_from_decl(
            &self.ident,
            &opts,
            self.decl.clone(),
            self.attrs.clone(),
            self.vis.clone(),
            false,
            None,
        )?
        .0;
        let catch = opts.catch();
        let variadic = opts.variadic();
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

        let mut operation_kind = ast::OperationKind::Regular;
        if let Some(g) = opts.getter() {
            operation_kind = ast::OperationKind::Getter(g);
        }
        if let Some(s) = opts.setter() {
            operation_kind = ast::OperationKind::Setter(s);
        }
        if opts.indexing_getter() {
            operation_kind = ast::OperationKind::IndexingGetter;
        }
        if opts.indexing_setter() {
            operation_kind = ast::OperationKind::IndexingSetter;
        }
        if opts.indexing_deleter() {
            operation_kind = ast::OperationKind::IndexingDeleter;
        }

        let kind = if opts.method() {
            let class = wasm.arguments.get(0).ok_or_else(|| {
                err_span!(self, "imported methods must have at least one argument")
            })?;
            let class = match class.ty {
                syn::Type::Reference(syn::TypeReference {
                    mutability: None,
                    ref elem,
                    ..
                }) => &**elem,
                _ => bail_span!(
                    class.ty,
                    "first argument of method must be a shared reference"
                ),
            };
            let class_name = match *class {
                syn::Type::Path(syn::TypePath {
                    qself: None,
                    ref path,
                }) => path,
                _ => bail_span!(class, "first argument of method must be a path"),
            };
            let class_name = extract_path_ident(class_name)?;
            let class_name = opts
                .js_class()
                .map(Into::into)
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
        } else if let Some(cls) = opts.static_method_of() {
            let class = opts
                .js_class()
                .map(Into::into)
                .unwrap_or_else(|| cls.to_string());
            let ty = ident_ty(cls.clone());

            let kind = ast::MethodKind::Operation(ast::Operation {
                is_static: true,
                kind: operation_kind,
            });

            ast::ImportFunctionKind::Method { class, ty, kind }
        } else if opts.constructor() {
            let class = match js_ret {
                Some(ref ty) => ty,
                _ => bail_span!(self, "constructor returns must be bare types"),
            };
            let class_name = match *class {
                syn::Type::Path(syn::TypePath {
                    qself: None,
                    ref path,
                }) => path,
                _ => bail_span!(self, "return value of constructor must be a bare path"),
            };
            let class_name = extract_path_ident(class_name)?;
            let class_name = opts
                .js_class()
                .map(Into::into)
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
            let data = (ns, &self.ident, module);
            format!(
                "__wbg_{}_{}",
                wasm.name
                    .chars()
                    .filter(|c| c.is_ascii_alphanumeric())
                    .collect::<String>(),
                ShortHash(data)
            )
        };
        if let Some(ident) = opts.final_() {
            if opts.structural() {
                bail_span!(ident, "cannot specify both `structural` and `final`");
            }
        }
        Ok(ast::ImportKind::Function(ast::ImportFunction {
            function: wasm,
            kind,
            js_ret,
            catch,
            variadic,
            structural: opts.structural() || opts.final_().is_none(),
            rust_name: self.ident.clone(),
            shim: Ident::new(&shim, Span::call_site()),
            doc_comment: None,
        }))
    }
}

impl ConvertToAst<BindgenAttrs> for syn::ForeignItemType {
    type Target = ast::ImportKind;

    fn convert(self, attrs: BindgenAttrs) -> Result<Self::Target, Diagnostic> {
        assert_not_variadic(&attrs, &self)?;
        let js_name = attrs
            .js_name()
            .map(|s| s.0)
            .map_or_else(|| self.ident.to_string(), |s| s.to_string());
        let shim = format!("__wbg_instanceof_{}_{}", self.ident, ShortHash(&self.ident));
        Ok(ast::ImportKind::Type(ast::ImportType {
            vis: self.vis,
            attrs: self.attrs,
            doc_comment: None,
            instanceof_shim: shim,
            rust_name: self.ident,
            js_name,
            extends: attrs.extends().cloned().collect(),
            vendor_prefixes: attrs.vendor_prefixes().cloned().collect(),
        }))
    }
}

impl<'a> ConvertToAst<(BindgenAttrs, &'a Option<String>)> for syn::ForeignItemStatic {
    type Target = ast::ImportKind;

    fn convert(
        self,
        (opts, module): (BindgenAttrs, &'a Option<String>),
    ) -> Result<Self::Target, Diagnostic> {
        if self.mutability.is_some() {
            bail_span!(self.mutability, "cannot import mutable globals yet")
        }
        assert_not_variadic(&opts, &self)?;
        let default_name = self.ident.to_string();
        let js_name = opts.js_name().map(|p| p.0).unwrap_or(&default_name);
        let shim = format!(
            "__wbg_static_accessor_{}_{}",
            self.ident,
            ShortHash((&js_name, module, &self.ident)),
        );
        Ok(ast::ImportKind::Static(ast::ImportStatic {
            ty: *self.ty,
            vis: self.vis,
            rust_name: self.ident.clone(),
            js_name: js_name.to_string(),
            shim: Ident::new(&shim, Span::call_site()),
        }))
    }
}

impl ConvertToAst<BindgenAttrs> for syn::ItemFn {
    type Target = ast::Function;

    fn convert(self, attrs: BindgenAttrs) -> Result<Self::Target, Diagnostic> {
        match self.vis {
            syn::Visibility::Public(_) => {}
            _ => bail_span!(self, "can only #[wasm_bindgen] public functions"),
        }
        if self.constness.is_some() {
            bail_span!(
                self.constness,
                "can only #[wasm_bindgen] non-const functions"
            );
        }
        if self.unsafety.is_some() {
            bail_span!(self.unsafety, "can only #[wasm_bindgen] safe functions");
        }
        assert_not_variadic(&attrs, &self)?;

        Ok(function_from_decl(
            &self.ident,
            &attrs,
            self.decl,
            self.attrs,
            self.vis,
            false,
            None,
        )?
        .0)
    }
}

/// Construct a function (and gets the self type if appropriate) for our AST from a syn function.
fn function_from_decl(
    decl_name: &syn::Ident,
    opts: &BindgenAttrs,
    decl: Box<syn::FnDecl>,
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    allow_self: bool,
    self_ty: Option<&Ident>,
) -> Result<(ast::Function, Option<ast::MethodSelf>), Diagnostic> {
    if decl.variadic.is_some() {
        bail_span!(decl.variadic, "can't #[wasm_bindgen] variadic functions");
    }
    if decl.generics.params.len() > 0 {
        bail_span!(
            decl.generics,
            "can't #[wasm_bindgen] functions with lifetime or type parameters",
        );
    }

    assert_no_lifetimes(&decl)?;

    let syn::FnDecl { inputs, output, .. } = { *decl };

    let replace_self = |t: syn::Type| {
        let self_ty = match self_ty {
            Some(i) => i,
            None => return t,
        };
        let path = match t {
            syn::Type::Path(syn::TypePath { qself: None, path }) => path,
            other => return other,
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
            syn::FnArg::Captured(mut c) => {
                c.ty = replace_self(c.ty);
                Some(c)
            }
            syn::FnArg::SelfValue(_) => {
                assert!(method_self.is_none());
                method_self = Some(ast::MethodSelf::ByValue);
                None
            }
            syn::FnArg::SelfRef(ref a) if allow_self => {
                assert!(method_self.is_none());
                if a.mutability.is_some() {
                    method_self = Some(ast::MethodSelf::RefMutable);
                } else {
                    method_self = Some(ast::MethodSelf::RefShared);
                }
                None
            }
            _ => panic!("arguments cannot be `self` or ignored"),
        })
        .collect::<Vec<_>>();

    let ret = match output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ty) => Some(replace_self(*ty)),
    };

    let js_name = opts.js_name();
    Ok((
        ast::Function {
            name: js_name
                .map(|s| s.0.to_string())
                .unwrap_or(decl_name.to_string()),
            name_span: js_name.map(|s| s.1).unwrap_or(decl_name.span()),
            renamed_via_js_name: js_name.is_some(),
            arguments,
            ret,
            rust_vis: vis,
            rust_attrs: attrs,
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

impl<'a> MacroParse<(Option<BindgenAttrs>, &'a mut TokenStream)> for syn::Item {
    fn macro_parse(
        self,
        program: &mut ast::Program,
        (opts, tokens): (Option<BindgenAttrs>, &'a mut TokenStream),
    ) -> Result<(), Diagnostic> {
        match self {
            syn::Item::Fn(mut f) => {
                let no_mangle = f
                    .attrs
                    .iter()
                    .enumerate()
                    .filter_map(|(i, m)| m.interpret_meta().map(|m| (i, m)))
                    .find(|&(_, ref m)| m.name() == "no_mangle");
                match no_mangle {
                    Some((i, _)) => {
                        f.attrs.remove(i);
                    }
                    _ => {}
                }
                let comments = extract_doc_comments(&f.attrs);
                f.to_tokens(tokens);
                let opts = opts.unwrap_or_default();
                program.exports.push(ast::Export {
                    rust_class: None,
                    js_class: None,
                    method_self: None,
                    is_constructor: false,
                    comments,
                    rust_name: f.ident.clone(),
                    function: f.convert(opts)?,
                });
            }
            syn::Item::Struct(mut s) => {
                let opts = opts.unwrap_or_default();
                program.structs.push((&mut s).convert(opts)?);
                s.to_tokens(tokens);
            }
            syn::Item::Impl(mut i) => {
                let opts = opts.unwrap_or_default();
                (&mut i).macro_parse(program, opts)?;
                i.to_tokens(tokens);
            }
            syn::Item::ForeignMod(mut f) => {
                let opts = match opts {
                    Some(opts) => opts,
                    None => BindgenAttrs::find(&mut f.attrs)?,
                };
                f.macro_parse(program, opts)?;
            }
            syn::Item::Enum(e) => {
                e.to_tokens(tokens);
                e.macro_parse(program, ())?;
            }
            syn::Item::Const(mut c) => {
                let opts = match opts {
                    Some(opts) => opts,
                    None => BindgenAttrs::find(&mut c.attrs)?,
                };
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

impl<'a> MacroParse<BindgenAttrs> for &'a mut syn::ItemImpl {
    fn macro_parse(self, program: &mut ast::Program, opts: BindgenAttrs) -> Result<(), Diagnostic> {
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
        let name = match *self.self_ty {
            syn::Type::Path(syn::TypePath {
                qself: None,
                ref path,
            }) => extract_path_ident(path)?,
            _ => bail_span!(
                self.self_ty,
                "unsupported self type in #[wasm_bindgen] impl"
            ),
        };
        let mut errors = Vec::new();
        for item in self.items.iter_mut() {
            if let Err(e) = (&name, item).macro_parse(program, &opts) {
                errors.push(e);
            }
        }
        Diagnostic::from_vec(errors)
    }
}

impl<'a, 'b> MacroParse<&'a BindgenAttrs> for (&'a Ident, &'b mut syn::ImplItem) {
    fn macro_parse(
        self,
        program: &mut ast::Program,
        impl_opts: &'a BindgenAttrs,
    ) -> Result<(), Diagnostic> {
        let (class, item) = self;
        let method = match item {
            syn::ImplItem::Method(ref mut m) => m,
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
            syn::ImplItem::Existential(_) => bail_span!(
                &*item,
                "existentials in impls aren't supported with #[wasm_bindgen]"
            ),
            syn::ImplItem::Macro(_) => {
                bail_span!(&*item, "macros in impls aren't supported");
            }
            syn::ImplItem::Verbatim(_) => panic!("unparsed impl item?"),
        };
        match method.vis {
            syn::Visibility::Public(_) => {}
            _ => return Ok(()),
        }
        if method.defaultness.is_some() {
            panic!("default methods are not supported");
        }
        if method.sig.constness.is_some() {
            bail_span!(
                method.sig.constness,
                "can only #[wasm_bindgen] non-const functions",
            );
        }
        if method.sig.unsafety.is_some() {
            bail_span!(method.sig.unsafety, "can only bindgen safe functions",);
        }

        let opts = BindgenAttrs::find(&mut method.attrs)?;
        let comments = extract_doc_comments(&method.attrs);
        let is_constructor = opts.constructor();
        let (function, method_self) = function_from_decl(
            &method.sig.ident,
            &opts,
            Box::new(method.sig.decl.clone()),
            method.attrs.clone(),
            method.vis.clone(),
            true,
            Some(class),
        )?;
        let js_class = impl_opts
            .js_class()
            .map(|s| s.to_string())
            .unwrap_or(class.to_string());

        program.exports.push(ast::Export {
            rust_class: Some(class.clone()),
            js_class: Some(js_class),
            method_self,
            is_constructor,
            function,
            comments,
            rust_name: method.sig.ident.clone(),
        });
        Ok(())
    }
}

impl MacroParse<()> for syn::ItemEnum {
    fn macro_parse(self, program: &mut ast::Program, (): ()) -> Result<(), Diagnostic> {
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
                let value = match v.discriminant {
                    Some((
                        _,
                        syn::Expr::Lit(syn::ExprLit {
                            attrs: _,
                            lit: syn::Lit::Int(ref int_lit),
                        }),
                    )) => {
                        if int_lit.value() > <u32>::max_value() as u64 {
                            bail_span!(
                                int_lit,
                                "enums with #[wasm_bindgen] can only support \
                                 numbers that can be represented as u32"
                            );
                        }
                        int_lit.value() as u32
                    }
                    None => i as u32,
                    Some((_, ref expr)) => bail_span!(
                        expr,
                        "enums with #[wasm_bidngen] may only have \
                         number literal values",
                    ),
                };

                Ok(ast::Variant {
                    name: v.ident.clone(),
                    value,
                })
            })
            .collect::<Result<_, Diagnostic>>()?;
        let comments = extract_doc_comments(&self.attrs);
        program.enums.push(ast::Enum {
            name: self.ident,
            variants,
            comments,
        });
        Ok(())
    }
}

impl MacroParse<BindgenAttrs> for syn::ItemConst {
    fn macro_parse(self, program: &mut ast::Program, opts: BindgenAttrs) -> Result<(), Diagnostic> {
        // Shortcut
        if !opts.typescript_custom_section() {
            bail_span!(self, "#[wasm_bindgen] will not work on constants unless you are defining a #[wasm_bindgen(typescript_custom_section)].");
        }

        match *self.expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(litstr),
                ..
            }) => {
                program.typescript_custom_sections.push(litstr.value());
            }
            _ => {
                bail_span!(self, "Expected a string literal to be used with #[wasm_bindgen(typescript_custom_section)].");
            }
        }

        Ok(())
    }
}

impl MacroParse<BindgenAttrs> for syn::ItemForeignMod {
    fn macro_parse(self, program: &mut ast::Program, opts: BindgenAttrs) -> Result<(), Diagnostic> {
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
        for mut item in self.items.into_iter() {
            if let Err(e) = item.macro_parse(program, &opts) {
                errors.push(e);
            }
        }
        Diagnostic::from_vec(errors)
    }
}

impl<'a> MacroParse<&'a BindgenAttrs> for syn::ForeignItem {
    fn macro_parse(
        mut self,
        program: &mut ast::Program,
        opts: &'a BindgenAttrs,
    ) -> Result<(), Diagnostic> {
        let item_opts = {
            let attrs = match self {
                syn::ForeignItem::Fn(ref mut f) => &mut f.attrs,
                syn::ForeignItem::Type(ref mut t) => &mut t.attrs,
                syn::ForeignItem::Static(ref mut s) => &mut s.attrs,
                _ => panic!("only foreign functions/types allowed for now"),
            };
            BindgenAttrs::find(attrs)?
        };
        let module = item_opts.module().or(opts.module()).map(|s| s.to_string());
        let js_namespace = item_opts.js_namespace().or(opts.js_namespace()).cloned();
        let kind = match self {
            syn::ForeignItem::Fn(f) => f.convert((item_opts, &module))?,
            syn::ForeignItem::Type(t) => t.convert(item_opts)?,
            syn::ForeignItem::Static(s) => s.convert((item_opts, &module))?,
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
    let path = match *t {
        syn::Type::Path(syn::TypePath {
            qself: None,
            ref path,
        }) => path,
        _ => bail_span!(t, "must be Result<...>"),
    };
    let seg = path
        .segments
        .last()
        .ok_or_else(|| err_span!(t, "must have at least one segment"))?
        .into_value();
    let generics = match seg.arguments {
        syn::PathArguments::AngleBracketed(ref t) => t,
        _ => bail_span!(t, "must be Result<...>"),
    };
    let generic = generics
        .args
        .first()
        .ok_or_else(|| err_span!(t, "must have at least one generic parameter"))?
        .into_value();
    let ty = match generic {
        syn::GenericArgument::Type(t) => t,
        other => bail_span!(other, "must be a type parameter"),
    };
    match *ty {
        syn::Type::Tuple(ref t) if t.elems.len() == 0 => return Ok(None),
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
                    a.tts.clone().into_iter().filter_map(|t| match t {
                        TokenTree::Literal(lit) => {
                            // this will always return the quoted string, we deal with
                            // that in the cli when we read in the comments
                            Some(lit.to_string())
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

/// Check there are no lifetimes on the function.
fn assert_no_lifetimes(decl: &syn::FnDecl) -> Result<(), Diagnostic> {
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
    syn::visit::Visit::visit_fn_decl(&mut walk, decl);
    Diagnostic::from_vec(walk.diagnostics)
}

/// This method always fails if the BindgenAttrs contain variadic
fn assert_not_variadic(attrs: &BindgenAttrs, span: &dyn ToTokens) -> Result<(), Diagnostic> {
    if attrs.variadic() {
        bail_span!(
            span,
            "the `variadic` attribute can only be applied to imported \
             (`extern`) functions"
        )
    }
    Ok(())
}

/// If the path is a single ident, return it.
fn extract_path_ident(path: &syn::Path) -> Result<Ident, Diagnostic> {
    if path.leading_colon.is_some() {
        bail_span!(path, "global paths are not supported yet");
    }
    if path.segments.len() != 1 {
        bail_span!(path, "multi-segment paths are not supported yet");
    }
    let value = &path.segments[0];
    match value.arguments {
        syn::PathArguments::None => {}
        _ => bail_span!(path, "paths with type parameters are not supported yet"),
    }
    Ok(value.ident.clone())
}
