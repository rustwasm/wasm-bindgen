use backend::ast;
use backend::util::{ident_ty, ShortHash};
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use shared;
use syn;

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Default)]
pub struct BindgenAttrs {
    pub attrs: Vec<BindgenAttr>,
}

impl BindgenAttrs {
    fn find(attrs: &mut Vec<syn::Attribute>) -> BindgenAttrs {
        let pos = attrs
            .iter()
            .enumerate()
            .find(|&(_, ref m)| m.path.segments[0].ident == "wasm_bindgen")
            .map(|a| a.0);
        let pos = match pos {
            Some(i) => i,
            None => return BindgenAttrs::default(),
        };
        let mut tts = attrs.remove(pos).tts.into_iter();
        let tt = match tts.next() {
            Some(TokenTree::Group(d)) => d.stream(),
            Some(_) => panic!("malformed #[wasm_bindgen] attribute"),
            None => return BindgenAttrs::default(),
        };
        if tts.next().is_some() {
            panic!("malformed #[wasm_bindgen] attribute");
        }
        syn::parse(tt.into()).expect("malformed #[wasm_bindgen] attribute")
    }

    fn module(&self) -> Option<&str> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::Module(s) => Some(&s[..]),
                _ => None,
            })
            .next()
    }

    fn version(&self) -> Option<&str> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::Version(s) => Some(&s[..]),
                _ => None,
            })
            .next()
    }

    fn catch(&self) -> bool {
        self.attrs.iter().any(|a| match a {
            BindgenAttr::Catch => true,
            _ => false,
        })
    }

    fn constructor(&self) -> bool {
        self.attrs.iter().any(|a| match a {
            BindgenAttr::Constructor => true,
            _ => false,
        })
    }

    fn static_method_of(&self) -> Option<&Ident> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::StaticMethodOf(c) => Some(c),
                _ => None,
            })
            .next()
    }

    fn method(&self) -> bool {
        self.attrs.iter().any(|a| match a {
            BindgenAttr::Method => true,
            _ => false,
        })
    }

    fn js_namespace(&self) -> Option<&Ident> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::JsNamespace(s) => Some(s),
                _ => None,
            })
            .next()
    }

    fn getter(&self) -> Option<Option<Ident>> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::Getter(g) => Some(g.clone()),
                _ => None,
            })
            .next()
    }

    fn setter(&self) -> Option<Option<Ident>> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::Setter(s) => Some(s.clone()),
                _ => None,
            })
            .next()
    }

    fn structural(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::Structural => true,
            _ => false,
        })
    }

    fn readonly(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::Readonly => true,
            _ => false,
        })
    }

    fn js_name(&self) -> Option<&Ident> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::JsName(s) => Some(s),
                _ => None,
            })
            .next()
    }

    fn js_class(&self) -> Option<&str> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::JsClass(s) => Some(&s[..]),
                _ => None,
            })
            .next()
    }
}

impl syn::synom::Synom for BindgenAttrs {
    named!(parse -> Self, alt!(
        do_parse!(
            opts: call!(
                syn::punctuated::Punctuated::<_, syn::token::Comma>::parse_terminated
            ) >>
            (BindgenAttrs {
                attrs: opts.into_iter().collect(),
            })
        ) => { |s| s }
        |
        epsilon!() => { |_| BindgenAttrs { attrs: Vec::new() } }
    ));
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub enum BindgenAttr {
    Catch,
    Constructor,
    Method,
    StaticMethodOf(Ident),
    JsNamespace(Ident),
    Module(String),
    Version(String),
    Getter(Option<Ident>),
    Setter(Option<Ident>),
    Structural,
    Readonly,
    JsName(Ident),
    JsClass(String),
}

impl syn::synom::Synom for BindgenAttr {
    named!(parse -> Self, alt!(
        call!(term, "catch") => { |_| BindgenAttr::Catch }
        |
        call!(term, "constructor") => { |_| BindgenAttr::Constructor }
        |
        call!(term, "method") => { |_| BindgenAttr::Method }
        |
        do_parse!(
            call!(term, "static_method_of") >>
            punct!(=) >>
            cls: call!(term2ident) >>
            (cls)
        )=> { BindgenAttr::StaticMethodOf }
        |
        do_parse!(
            call!(term, "getter") >>
            val: option!(do_parse!(
                punct!(=) >>
                s: call!(term2ident) >>
                (s)
            )) >>
            (val)
        )=> { BindgenAttr::Getter }
        |
        do_parse!(
            call!(term, "setter") >>
            val: option!(do_parse!(
                punct!(=) >>
                s: call!(term2ident) >>
                (s)
            )) >>
            (val)
        )=> { BindgenAttr::Setter }
        |
        call!(term, "structural") => { |_| BindgenAttr::Structural }
        |
        call!(term, "readonly") => { |_| BindgenAttr::Readonly }
        |
        do_parse!(
            call!(term, "js_namespace") >>
            punct!(=) >>
            ns: call!(term2ident) >>
            (ns)
        )=> { BindgenAttr::JsNamespace }
        |
        do_parse!(
            call!(term, "module") >>
            punct!(=) >>
            s: syn!(syn::LitStr) >>
            (s.value())
        )=> { BindgenAttr::Module }
        |
        do_parse!(
            call!(term, "version") >>
            punct!(=) >>
            s: syn!(syn::LitStr) >>
            (s.value())
        )=> { BindgenAttr::Version }
        |
        do_parse!(
            call!(term, "js_name") >>
            punct!(=) >>
            ns: call!(term2ident) >>
            (ns)
        )=> { BindgenAttr::JsName }
        |
        do_parse!(
            call!(term, "js_class") >>
            punct!(=) >>
            s: syn!(syn::LitStr) >>
            (s.value())
        )=> { BindgenAttr::JsClass }
    ));
}

fn term<'a>(cursor: syn::buffer::Cursor<'a>, name: &str) -> syn::synom::PResult<'a, ()> {
    if let Some((ident, next)) = cursor.ident() {
        if ident == name {
            return Ok(((), next));
        }
    }
    syn::parse_error()
}

fn term2ident<'a>(cursor: syn::buffer::Cursor<'a>) -> syn::synom::PResult<'a, Ident> {
    match cursor.ident() {
        Some(pair) => Ok(pair),
        None => syn::parse_error(),
    }
}

trait ConvertToAst<Ctx> {
    type Target;
    fn convert(self, context: Ctx) -> Self::Target;
}

impl<'a> ConvertToAst<()> for &'a mut syn::ItemStruct {
    type Target = ast::Struct;

    fn convert(self, (): ()) -> Self::Target {
        if self.generics.params.len() > 0 {
            panic!(
                "structs with #[wasm_bindgen] cannot have lifetime or \
                 type parameters currently"
            );
        }
        let mut fields = Vec::new();
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
                let ident = self.ident.to_string();
                let name_str = name.to_string();
                let getter = shared::struct_field_get(&ident, &name_str);
                let setter = shared::struct_field_set(&ident, &name_str);
                let opts = BindgenAttrs::find(&mut field.attrs);
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
        ast::Struct {
            name: self.ident.clone(),
            fields,
            comments,
        }
    }
}

impl ConvertToAst<BindgenAttrs> for syn::ForeignItemFn {
    type Target = ast::ImportKind;

    fn convert(self, opts: BindgenAttrs) -> Self::Target {
        let js_name = opts.js_name().unwrap_or(&self.ident).clone();
        let wasm = function_from_decl(&js_name, self.decl, self.attrs, self.vis, false).0;
        let catch = opts.catch();
        let js_ret = if catch {
            // TODO: this assumes a whole bunch:
            //
            // * The outer type is actually a `Result`
            // * The error type is a `JsValue`
            // * The actual type is the first type parameter
            //
            // should probably fix this one day...
            extract_first_ty_param(wasm.ret.as_ref())
                .expect("can't `catch` without returning a Result")
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

        let kind = if opts.method() {
            let class = wasm
                .arguments
                .get(0)
                .expect("methods must have at least one argument");
            let class = match class.ty {
                syn::Type::Reference(syn::TypeReference {
                    mutability: None,
                    ref elem,
                    ..
                }) => &**elem,
                _ => panic!("first argument of method must be a shared reference"),
            };
            let class_name = match *class {
                syn::Type::Path(syn::TypePath {
                    qself: None,
                    ref path,
                }) => path,
                _ => panic!("first argument of method must be a path"),
            };
            let class_name = extract_path_ident(class_name)
                .expect("first argument of method must be a bare type");
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
            let class = cls.to_string();
            let ty = ident_ty(cls.clone());

            let kind = ast::MethodKind::Operation(ast::Operation {
                is_static: true,
                kind: operation_kind,
            });

            ast::ImportFunctionKind::Method { class, ty, kind }
        } else if opts.constructor() {
            let class = match wasm.ret {
                Some(ref ty) => ty,
                _ => panic!("constructor returns must be bare types"),
            };
            let class_name = match *class {
                syn::Type::Path(syn::TypePath {
                    qself: None,
                    ref path,
                }) => path,
                _ => panic!("first argument of method must be a path"),
            };
            let class_name = extract_path_ident(class_name)
                .expect("first argument of method must be a bare type");

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
            let data = (ns, &self.ident);
            format!("__wbg_{}_{}", js_name, ShortHash(data))
        };
        ast::ImportKind::Function(ast::ImportFunction {
            function: wasm,
            kind,
            js_ret,
            catch,
            structural: opts.structural(),
            rust_name: self.ident.clone(),
            shim: Ident::new(&shim, Span::call_site()),
        })
    }
}

impl ConvertToAst<()> for syn::ForeignItemType {
    type Target = ast::ImportKind;

    fn convert(self, (): ()) -> Self::Target {
        ast::ImportKind::Type(ast::ImportType {
            vis: self.vis,
            name: self.ident,
            attrs: self.attrs,
        })
    }
}

impl ConvertToAst<BindgenAttrs> for syn::ForeignItemStatic {
    type Target = ast::ImportKind;

    fn convert(self, opts: BindgenAttrs) -> Self::Target {
        if self.mutability.is_some() {
            panic!("cannot import mutable globals yet")
        }
        let js_name = opts.js_name().unwrap_or(&self.ident);
        let shim = format!("__wbg_static_accessor_{}_{}", js_name, self.ident);
        ast::ImportKind::Static(ast::ImportStatic {
            ty: *self.ty,
            vis: self.vis,
            rust_name: self.ident.clone(),
            js_name: js_name.clone(),
            shim: Ident::new(&shim, Span::call_site()),
        })
    }
}

impl ConvertToAst<()> for syn::ItemFn {
    type Target = ast::Function;

    fn convert(self, (): ()) -> Self::Target {
        match self.vis {
            syn::Visibility::Public(_) => {}
            _ => panic!("can only bindgen public functions"),
        }
        if self.constness.is_some() {
            panic!("can only bindgen non-const functions");
        }
        if self.unsafety.is_some() {
            panic!("can only bindgen safe functions");
        }

        function_from_decl(&self.ident, self.decl, self.attrs, self.vis, false).0
    }
}

fn function_from_decl(
    name: &Ident,
    mut decl: Box<syn::FnDecl>,
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    allow_self: bool,
) -> (ast::Function, Option<ast::MethodSelf>) {
    if decl.variadic.is_some() {
        panic!("can't bindgen variadic functions")
    }
    if decl.generics.params.len() > 0 {
        panic!("can't bindgen functions with lifetime or type parameters")
    }

    assert_no_lifetimes(&mut decl);

    let syn::FnDecl { inputs, output, .. } = { *decl };

    let mut method_self = None;
    let arguments = inputs
        .into_iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Captured(c) => Some(c),
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
        syn::ReturnType::Type(_, ty) => Some(*ty),
    };

    (
        ast::Function {
            name: name.clone(),
            arguments,
            ret,
            rust_vis: vis,
            rust_attrs: attrs,
        },
        method_self,
    )
}

pub(crate) trait MacroParse<Ctx> {
    fn macro_parse(self, program: &mut ast::Program, context: Ctx);
}

impl<'a> MacroParse<(Option<BindgenAttrs>, &'a mut TokenStream)> for syn::Item {
    fn macro_parse(
        self,
        program: &mut ast::Program,
        (opts, tokens): (Option<BindgenAttrs>, &'a mut TokenStream),
    ) {
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
                program.exports.push(ast::Export {
                    class: None,
                    method_self: None,
                    constructor: None,
                    function: f.convert(()),
                    comments,
                });
            }
            syn::Item::Struct(mut s) => {
                program.structs.push((&mut s).convert(()));
                s.to_tokens(tokens);
            }
            syn::Item::Impl(mut i) => {
                (&mut i).macro_parse(program, ());
                i.to_tokens(tokens);
            }
            syn::Item::ForeignMod(mut f) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut f.attrs));
                f.macro_parse(program, opts);
            }
            syn::Item::Enum(e) => {
                e.to_tokens(tokens);
                e.macro_parse(program, ());
            }
            _ => panic!(
                "#[wasm_bindgen] can only be applied to a function, \
                 struct, enum, impl, or extern block"
            ),
        }
    }
}

impl<'a> MacroParse<()> for &'a mut syn::ItemImpl {
    fn macro_parse(self, program: &mut ast::Program, (): ()) {
        if self.defaultness.is_some() {
            panic!("default impls are not supported");
        }
        if self.unsafety.is_some() {
            panic!("unsafe impls are not supported");
        }
        if self.trait_.is_some() {
            panic!("trait impls are not supported");
        }
        if self.generics.params.len() > 0 {
            panic!("generic impls aren't supported");
        }
        let name = match *self.self_ty {
            syn::Type::Path(syn::TypePath {
                qself: None,
                ref path,
            }) => match extract_path_ident(path) {
                Some(ident) => ident,
                None => panic!("unsupported self type in impl"),
            },
            _ => panic!("unsupported self type in impl"),
        };
        for item in self.items.iter_mut() {
            (&name, item).macro_parse(program, ())
        }
    }
}

impl<'a, 'b> MacroParse<()> for (&'a Ident, &'b mut syn::ImplItem) {
    fn macro_parse(self, program: &mut ast::Program, (): ()) {
        let (class, item) = self;
        replace_self(class, item);
        let method = match item {
            syn::ImplItem::Const(_) => panic!("const definitions aren't supported"),
            syn::ImplItem::Type(_) => panic!("type definitions in impls aren't supported"),
            syn::ImplItem::Method(ref mut m) => m,
            syn::ImplItem::Macro(_) => panic!("macros in impls aren't supported"),
            syn::ImplItem::Verbatim(_) => panic!("unparsed impl item?"),
        };
        match method.vis {
            syn::Visibility::Public(_) => {}
            _ => return,
        }
        if method.defaultness.is_some() {
            panic!("default methods are not supported");
        }
        if method.sig.constness.is_some() {
            panic!("can only bindgen non-const functions");
        }
        if method.sig.unsafety.is_some() {
            panic!("can only bindgen safe functions");
        }

        let opts = BindgenAttrs::find(&mut method.attrs);
        let comments = extract_doc_comments(&method.attrs);
        let is_constructor = opts.constructor();
        let constructor = if is_constructor {
            Some(method.sig.ident.to_string())
        } else {
            None
        };

        let (function, method_self) = function_from_decl(
            &method.sig.ident,
            Box::new(method.sig.decl.clone()),
            method.attrs.clone(),
            method.vis.clone(),
            true,
        );

        program.exports.push(ast::Export {
            class: Some(class.clone()),
            method_self,
            constructor,
            function,
            comments,
        });
    }
}

impl MacroParse<()> for syn::ItemEnum {
    fn macro_parse(self, program: &mut ast::Program, (): ()) {
        match self.vis {
            syn::Visibility::Public(_) => {}
            _ => panic!("only public enums are allowed"),
        }

        let variants = self
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| {
                match v.fields {
                    syn::Fields::Unit => (),
                    _ => panic!("Only C-Style enums allowed"),
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
                            panic!("Enums can only support numbers that can be represented as u32");
                        }
                        int_lit.value() as u32
                    }
                    None => i as u32,
                    _ => panic!("Enums may only have number literal values"),
                };

                ast::Variant {
                    name: v.ident.clone(),
                    value,
                }
            })
            .collect();
        let comments = extract_doc_comments(&self.attrs);
        program.enums.push(ast::Enum {
            name: self.ident,
            variants,
            comments,
        });
    }
}

impl MacroParse<BindgenAttrs> for syn::ItemForeignMod {
    fn macro_parse(self, program: &mut ast::Program, opts: BindgenAttrs) {
        match self.abi.name {
            Some(ref l) if l.value() == "C" => {}
            None => {}
            _ => panic!("only foreign mods with the `C` ABI are allowed"),
        }
        for mut item in self.items.into_iter() {
            let item_opts = {
                let attrs = match item {
                    syn::ForeignItem::Fn(ref mut f) => &mut f.attrs,
                    syn::ForeignItem::Type(ref mut t) => &mut t.attrs,
                    syn::ForeignItem::Static(ref mut s) => &mut s.attrs,
                    _ => panic!("only foreign functions/types allowed for now"),
                };
                BindgenAttrs::find(attrs)
            };
            let module = item_opts.module().or(opts.module()).map(|s| s.to_string());
            let version = item_opts
                .version()
                .or(opts.version())
                .map(|s| s.to_string());
            let js_namespace = item_opts.js_namespace().or(opts.js_namespace()).cloned();
            let mut kind = match item {
                syn::ForeignItem::Fn(f) => f.convert(item_opts),
                syn::ForeignItem::Type(t) => t.convert(()),
                syn::ForeignItem::Static(s) => s.convert(item_opts),
                _ => panic!("only foreign functions/types allowed for now"),
            };

            program.imports.push(ast::Import {
                module,
                version,
                js_namespace,
                kind,
            });
        }
    }
}

fn extract_first_ty_param(ty: Option<&syn::Type>) -> Option<Option<syn::Type>> {
    let t = match ty {
        Some(t) => t,
        None => return Some(None),
    };
    let path = match *t {
        syn::Type::Path(syn::TypePath {
            qself: None,
            ref path,
        }) => path,
        _ => return None,
    };
    let seg = path.segments.last()?.into_value();
    let generics = match seg.arguments {
        syn::PathArguments::AngleBracketed(ref t) => t,
        _ => return None,
    };
    let ty = match *generics.args.first()?.into_value() {
        syn::GenericArgument::Type(ref t) => t,
        _ => return None,
    };
    match *ty {
        syn::Type::Tuple(ref t) if t.elems.len() == 0 => return Some(None),
        _ => {}
    }
    Some(Some(ty.clone()))
}

fn replace_self(name: &Ident, item: &mut syn::ImplItem) {
    struct Walk<'a>(&'a Ident);

    impl<'a> syn::visit_mut::VisitMut for Walk<'a> {
        fn visit_ident_mut(&mut self, i: &mut Ident) {
            if i == "Self" {
                *i = self.0.clone();
            }
        }
    }

    syn::visit_mut::VisitMut::visit_impl_item_mut(&mut Walk(name), item);
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
                    },
                    _ => None,
                })
            )
        } else {
            None
        }
    })
    //Fold up the [[String]] iter we created into Vec<String>
    .fold(vec![], |mut acc, a| {acc.extend(a); acc})
}

fn assert_no_lifetimes(decl: &mut syn::FnDecl) {
    struct Walk;

    impl<'ast> syn::visit_mut::VisitMut for Walk {
        fn visit_lifetime_mut(&mut self, _i: &mut syn::Lifetime) {
            panic!(
                "it is currently not sound to use lifetimes in function \
                 signatures"
            );
        }
    }

    syn::visit_mut::VisitMut::visit_fn_decl_mut(&mut Walk, decl);
}

fn extract_path_ident(path: &syn::Path) -> Option<Ident> {
    if path.leading_colon.is_some() {
        return None;
    }
    if path.segments.len() != 1 {
        return None;
    }
    match path.segments.first().unwrap().value().arguments {
        syn::PathArguments::None => {}
        _ => return None,
    }
    path.segments.first().map(|v| v.value().ident.clone())
}
