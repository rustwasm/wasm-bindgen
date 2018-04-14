use quote::{ToTokens, Tokens};
use shared;
use syn;

#[derive(Default)]
pub struct Program {
    pub exports: Vec<Export>,
    pub imports: Vec<Import>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
}

pub struct Export {
    pub class: Option<syn::Ident>,
    pub method: bool,
    pub mutable: bool,
    pub constructor: Option<String>,
    pub function: Function,
}

pub struct Import {
    pub module: Option<String>,
    pub js_namespace: Option<syn::Ident>,
    pub kind: ImportKind,
}

pub enum ImportKind {
    Function(ImportFunction),
    Static(ImportStatic),
    Type(ImportType),
}

pub struct ImportFunction {
    pub function: Function,
    pub rust_name: syn::Ident,
    pub kind: ImportFunctionKind,
    pub shim: syn::Ident,
}

pub enum ImportFunctionKind {
    Method { class: String, ty: syn::Type },
    JsConstructor { class: String, ty: syn::Type },
    Normal,
}

pub struct ImportStatic {
    pub vis: syn::Visibility,
    pub ty: syn::Type,
    pub shim: syn::Ident,
    pub rust_name: syn::Ident,
    pub js_name: syn::Ident,
}

pub struct ImportType {
    pub vis: syn::Visibility,
    pub name: syn::Ident,
}

pub struct Function {
    pub name: syn::Ident,
    pub arguments: Vec<Type>,
    pub ret: Option<Type>,
    pub opts: BindgenAttrs,
    pub rust_attrs: Vec<syn::Attribute>,
    pub rust_decl: Box<syn::FnDecl>,
    pub rust_vis: syn::Visibility,
}

pub struct Struct {
    pub name: syn::Ident,
}

pub struct Enum {
    pub name: syn::Ident,
    pub variants: Vec<Variant>,
}

pub struct Variant {
    pub name: syn::Ident,
    pub value: u32,
}

pub struct Type {
    pub ty: syn::Type,
    pub kind: TypeKind,
    pub loc: TypeLocation,
}

#[derive(Copy, Clone)]
pub enum TypeKind {
    ByRef,
    ByMutRef,
    ByValue,
}

#[derive(Copy, Clone)]
pub enum TypeLocation {
    ImportArgument,
    ImportRet,
    ExportArgument,
    ExportRet,
}

impl Program {
    pub fn push_item(&mut self, item: syn::Item, opts: Option<BindgenAttrs>, tokens: &mut Tokens) {
        match item {
            syn::Item::Fn(mut f) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut f.attrs));

                let no_mangle = f.attrs
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
                f.to_tokens(tokens);
                self.exports.push(Export {
                    class: None,
                    method: false,
                    mutable: false,
                    constructor: None,
                    function: Function::from(f, opts),
                });
            }
            syn::Item::Struct(mut s) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut s.attrs));
                s.to_tokens(tokens);
                self.structs.push(Struct::from(s, opts));
            }
            syn::Item::Impl(mut i) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut i.attrs));
                self.push_impl(&mut i, opts);
                i.to_tokens(tokens);
            }
            syn::Item::ForeignMod(mut f) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut f.attrs));
                self.push_foreign_mod(f, opts);
            }
            syn::Item::Enum(mut e) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut e.attrs));
                e.to_tokens(tokens);
                self.push_enum(e, opts);
            }
            _ => panic!(
                "#[wasm_bindgen] can only be applied to a function, \
                 struct, enum, impl, or extern block"
            ),
        }
    }

    pub fn push_impl(&mut self, item: &mut syn::ItemImpl, _opts: BindgenAttrs) {
        if item.defaultness.is_some() {
            panic!("default impls are not supported");
        }
        if item.unsafety.is_some() {
            panic!("unsafe impls are not supported");
        }
        if item.trait_.is_some() {
            panic!("trait impls are not supported");
        }
        if item.generics.params.len() > 0 {
            panic!("generic impls aren't supported");
        }
        let name = match *item.self_ty {
            syn::Type::Path(syn::TypePath {
                qself: None,
                ref path,
            }) => match extract_path_ident(path) {
                Some(ident) => ident,
                None => panic!("unsupported self type in impl"),
            },
            _ => panic!("unsupported self type in impl"),
        };
        for mut item in item.items.iter_mut() {
            self.push_impl_item(name, &mut item);
        }
    }

    fn push_impl_item(&mut self, class: syn::Ident, item: &mut syn::ImplItem) {
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
        let is_constructor = opts.constructor();
        let constructor = if is_constructor {
            Some(method.sig.ident.to_string())
        } else {
            None
        };

        let (function, mutable) = Function::from_decl(
            method.sig.ident,
            Box::new(method.sig.decl.clone()),
            method.attrs.clone(),
            opts,
            method.vis.clone(),
            true,
            false,
        );

        self.exports.push(Export {
            class: Some(class),
            method: mutable.is_some(),
            mutable: mutable.unwrap_or(false),
            constructor,
            function,
        });
    }

    pub fn push_enum(&mut self, item: syn::ItemEnum, _opts: BindgenAttrs) {
        match item.vis {
            syn::Visibility::Public(_) => {}
            _ => panic!("only public enums are allowed"),
        }

        let variants = item.variants
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

                Variant {
                    name: v.ident,
                    value,
                }
            })
            .collect();
        self.enums.push(Enum {
            name: item.ident,
            variants,
        });
    }

    pub fn push_foreign_mod(&mut self, f: syn::ItemForeignMod, opts: BindgenAttrs) {
        match f.abi.name {
            Some(ref l) if l.value() == "C" => {}
            None => {}
            _ => panic!("only foreign mods with the `C` ABI are allowed"),
        }
        for mut item in f.items.into_iter() {
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
            let js_namespace = item_opts.js_namespace().or(opts.js_namespace());
            let mut kind = match item {
                syn::ForeignItem::Fn(f) => self.push_foreign_fn(f, item_opts),
                syn::ForeignItem::Type(t) => self.push_foreign_ty(t),
                syn::ForeignItem::Static(s) => self.push_foreign_static(s, item_opts),
                _ => panic!("only foreign functions/types allowed for now"),
            };

            self.imports.push(Import {
                module,
                js_namespace,
                kind,
            });
        }
    }

    pub fn push_foreign_fn(&mut self, f: syn::ForeignItemFn, opts: BindgenAttrs) -> ImportKind {
        let js_name = opts.js_name().unwrap_or(f.ident);
        let mut wasm = Function::from_decl(
            js_name,
            f.decl,
            f.attrs,
            opts,
            f.vis,
            false,
            true,
        ).0;
        if wasm.opts.catch() {
            // TODO: this assumes a whole bunch:
            //
            // * The outer type is actually a `Result`
            // * The error type is a `JsValue`
            // * The actual type is the first type parameter
            //
            // should probably fix this one day...
            wasm.ret = extract_first_ty_param(wasm.ret.as_ref())
                .expect("can't `catch` without returning a Result");
        }

        let kind = if wasm.opts.method() {
            let class = wasm.arguments
                .get(0)
                .expect("methods must have at least one argument");
            let class_name = match class.ty {
                syn::Type::Path(syn::TypePath {
                    qself: None,
                    ref path,
                }) => path,
                _ => panic!("first argument of method must be a path"),
            };
            let class_name = extract_path_ident(class_name)
                .expect("first argument of method must be a bare type");

            ImportFunctionKind::Method {
                class: class_name.as_ref().to_string(),
                ty: class.ty.clone(),
            }
        } else if wasm.opts.constructor() {
            let class = match wasm.ret {
                Some(Type { ref ty, kind: TypeKind::ByValue, .. }) => ty,
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

            ImportFunctionKind::JsConstructor {
                class: class_name.as_ref().to_string(),
                ty: class.clone(),
            }
        } else {
            ImportFunctionKind::Normal
        };

        let shim = {
            let ns = match kind {
                ImportFunctionKind::Normal => "n",
                ImportFunctionKind::Method { ref class, .. } => class,
                ImportFunctionKind::JsConstructor { ref class, .. } => class,
            };
            format!("__wbg_f_{}_{}_{}", js_name, f.ident, ns)
        };
        ImportKind::Function(ImportFunction {
            function: wasm,
            kind,
            rust_name: f.ident,
            shim: shim.into(),
        })
    }

    pub fn push_foreign_ty(&mut self, f: syn::ForeignItemType) -> ImportKind {
        ImportKind::Type(ImportType {
            vis: f.vis,
            name: f.ident,
        })
    }

    pub fn push_foreign_static(
        &mut self,
        f: syn::ForeignItemStatic,
        opts: BindgenAttrs,
    ) -> ImportKind {
        if f.mutability.is_some() {
            panic!("cannot import mutable globals yet")
        }
        let js_name = opts.js_name().unwrap_or(f.ident);
        let shim = format!("__wbg_static_accessor_{}_{}", js_name, f.ident);
        ImportKind::Static(ImportStatic {
            ty: *f.ty,
            vis: f.vis,
            rust_name: f.ident,
            js_name,
            shim: shim.into(),
        })
    }

    pub fn shared(&self) -> shared::Program {
        shared::Program {
            exports: self.exports.iter().map(|a| a.shared()).collect(),
            enums: self.enums.iter().map(|a| a.shared()).collect(),
            imports: self.imports.iter().map(|a| a.shared()).collect(),
            version: shared::version(),
            schema_version: shared::SCHEMA_VERSION.to_string(),
        }
    }
}

impl Function {
    pub fn from(input: syn::ItemFn, opts: BindgenAttrs) -> Function {
        match input.vis {
            syn::Visibility::Public(_) => {}
            _ => panic!("can only bindgen public functions"),
        }
        if input.constness.is_some() {
            panic!("can only bindgen non-const functions");
        }
        if input.unsafety.is_some() {
            panic!("can only bindgen safe functions");
        }

        Function::from_decl(
            input.ident,
            input.decl,
            input.attrs,
            opts,
            input.vis,
            false,
            false,
        ).0
    }

    pub fn from_decl(
        name: syn::Ident,
        decl: Box<syn::FnDecl>,
        attrs: Vec<syn::Attribute>,
        opts: BindgenAttrs,
        vis: syn::Visibility,
        allow_self: bool,
        import: bool,
    ) -> (Function, Option<bool>) {
        if decl.variadic.is_some() {
            panic!("can't bindgen variadic functions")
        }
        if decl.generics.params.len() > 0 {
            panic!("can't bindgen functions with lifetime or type parameters")
        }

        let mut mutable = None;
        let arguments = decl.inputs
            .iter()
            .filter_map(|arg| match *arg {
                syn::FnArg::Captured(ref c) => Some(c),
                syn::FnArg::SelfValue(_) => {
                    panic!("by-value `self` not yet supported");
                }
                syn::FnArg::SelfRef(ref a) if allow_self => {
                    assert!(mutable.is_none());
                    mutable = Some(a.mutability.is_some());
                    None
                }
                _ => panic!("arguments cannot be `self` or ignored"),
            })
            .map(|arg| {
                Type::from(&arg.ty, if import {
                    TypeLocation::ImportArgument
                } else {
                    TypeLocation::ExportArgument
                })
            })
            .collect::<Vec<_>>();

        let ret = match decl.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ref t) => {
                Some(Type::from(t, if import {
                    TypeLocation::ImportRet
                } else {
                    TypeLocation::ExportRet
                }))
            }
        };

        (
            Function {
                name,
                arguments,
                ret,
                opts,
                rust_vis: vis,
                rust_decl: decl,
                rust_attrs: attrs,
            },
            mutable,
        )
    }

    fn shared(&self) -> shared::Function {
        shared::Function {
            name: self.name.as_ref().to_string(),
        }
    }
}

pub fn extract_path_ident(path: &syn::Path) -> Option<syn::Ident> {
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
    path.segments.first().map(|v| v.value().ident)
}

impl Export {
    pub fn rust_symbol(&self) -> syn::Ident {
        let mut generated_name = format!("__wasm_bindgen_generated");
        if let Some(class) = self.class {
            generated_name.push_str("_");
            generated_name.push_str(class.as_ref());
        }
        generated_name.push_str("_");
        generated_name.push_str(self.function.name.as_ref());
        syn::Ident::from(generated_name)
    }

    pub fn export_name(&self) -> String {
        match self.class {
            Some(class) => {
                shared::struct_function_export_name(class.as_ref(), self.function.name.as_ref())
            }
            None => shared::free_function_export_name(self.function.name.as_ref()),
        }
    }

    fn shared(&self) -> shared::Export {
        shared::Export {
            class: self.class.map(|s| s.as_ref().to_string()),
            method: self.method,
            constructor: self.constructor.clone(),
            function: self.function.shared(),
        }
    }
}

impl Enum {
    fn shared(&self) -> shared::Enum {
        shared::Enum {
            name: self.name.as_ref().to_string(),
            variants: self.variants.iter().map(|v| v.shared()).collect(),
        }
    }
}

impl Variant {
    fn shared(&self) -> shared::EnumVariant {
        shared::EnumVariant {
            name: self.name.as_ref().to_string(),
            value: self.value,
        }
    }
}

impl Import {
    fn shared(&self) -> shared::Import {
        shared::Import {
            module: self.module.clone(),
            js_namespace: self.js_namespace.map(|s| s.as_ref().to_string()),
            kind: self.kind.shared(),
        }
    }
}

impl ImportKind {
    fn shared(&self) -> shared::ImportKind {
        match *self {
            ImportKind::Function(ref f) => shared::ImportKind::Function(f.shared()),
            ImportKind::Static(ref f) => shared::ImportKind::Static(f.shared()),
            ImportKind::Type(ref f) => shared::ImportKind::Type(f.shared()),
        }
    }
}

impl ImportFunction {
    pub fn infer_getter_property(&self) -> String {
        self.function.name.as_ref().to_string()
    }

    pub fn infer_setter_property(&self) -> String {
        let name = self.function.name.as_ref();
        assert!(name.starts_with("set_"), "setters must start with `set_`");
        name[4..].to_string()
    }

    fn shared(&self) -> shared::ImportFunction {
        let mut method = false;
        let mut js_new = false;
        let mut class_name = None;
        match self.kind {
            ImportFunctionKind::Method { ref class, .. } => {
                method = true;
                class_name = Some(class);
            }
            ImportFunctionKind::JsConstructor { ref class, .. } => {
                js_new = true;
                class_name = Some(class);
            }
            ImportFunctionKind::Normal => {}
        }
        let mut getter = None;
        let mut setter = None;

        if let Some(s) = self.function.opts.getter() {
            let s = s.map(|s| s.to_string());
            getter = Some(s.unwrap_or_else(|| self.infer_getter_property()));
        }
        if let Some(s) = self.function.opts.setter() {
            let s = s.map(|s| s.to_string());
            setter = Some(s.unwrap_or_else(|| self.infer_setter_property()));
        }
        shared::ImportFunction {
            shim: self.shim.as_ref().to_string(),
            catch: self.function.opts.catch(),
            method,
            js_new,
            structural: self.function.opts.structural(),
            getter,
            setter,
            class: class_name.cloned(),
            function: self.function.shared(),
        }
    }
}

impl ImportStatic {
    fn shared(&self) -> shared::ImportStatic {
        shared::ImportStatic {
            name: self.js_name.as_ref().to_string(),
            shim: self.shim.as_ref().to_string(),
        }
    }
}

impl ImportType {
    fn shared(&self) -> shared::ImportType {
        shared::ImportType { }
    }
}

impl Struct {
    fn from(s: syn::ItemStruct, _opts: BindgenAttrs) -> Struct {
        Struct { name: s.ident }
    }
}

impl Type {
    pub fn from(ty: &syn::Type, loc: TypeLocation) -> Type {
        let (ty, kind) = match *ty {
            syn::Type::Reference(ref r) => {
                if r.mutability.is_some() {
                    ((*r.elem).clone(), TypeKind::ByMutRef)
                } else {
                    ((*r.elem).clone(), TypeKind::ByRef)
                }
            }
            _ => (ty.clone(), TypeKind::ByValue),
        };
        Type { loc, ty, kind }
    }
}

#[derive(Default)]
pub struct BindgenAttrs {
    attrs: Vec<BindgenAttr>,
}

impl BindgenAttrs {
    pub fn find(attrs: &mut Vec<syn::Attribute>) -> BindgenAttrs {
        let pos = attrs
            .iter()
            .enumerate()
            .find(|&(_, ref m)| m.path.segments[0].ident == "wasm_bindgen")
            .map(|a| a.0);
        let pos = match pos {
            Some(i) => i,
            None => return BindgenAttrs::default(),
        };
        syn::parse(attrs.remove(pos).tts.into()).expect("malformed #[wasm_bindgen] attribute")
    }

    fn module(&self) -> Option<&str> {
        self.attrs
            .iter()
            .filter_map(|a| match *a {
                BindgenAttr::Module(ref s) => Some(&s[..]),
                _ => None,
            })
            .next()
    }

    pub fn catch(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::Catch => true,
            _ => false,
        })
    }

    fn constructor(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::Constructor => true,
            _ => false,
        })
    }

    fn method(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::Method => true,
            _ => false,
        })
    }

    fn js_namespace(&self) -> Option<syn::Ident> {
        self.attrs
            .iter()
            .filter_map(|a| match *a {
                BindgenAttr::JsNamespace(s) => Some(s),
                _ => None,
            })
            .next()
    }

    pub fn getter(&self) -> Option<Option<syn::Ident>> {
        self.attrs
            .iter()
            .filter_map(|a| match *a {
                BindgenAttr::Getter(s) => Some(s),
                _ => None,
            })
            .next()
    }

    pub fn setter(&self) -> Option<Option<syn::Ident>> {
        self.attrs
            .iter()
            .filter_map(|a| match *a {
                BindgenAttr::Setter(s) => Some(s),
                _ => None,
            })
            .next()
    }

    pub fn structural(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::Structural => true,
            _ => false,
        })
    }

    pub fn js_name(&self) -> Option<syn::Ident> {
        self.attrs
            .iter()
            .filter_map(|a| match *a {
                BindgenAttr::JsName(s) => Some(s),
                _ => None,
            })
            .next()
    }
}

impl syn::synom::Synom for BindgenAttrs {
    named!(parse -> Self, alt!(
        do_parse!(
            opts: parens!(call!(
                syn::punctuated::Punctuated::<_, syn::token::Comma>::parse_terminated
            )) >>
            (BindgenAttrs {
                attrs: opts.1.into_iter().collect(),
            })
        ) => { |s| s }
        |
        epsilon!() => { |_| BindgenAttrs { attrs: Vec::new() } }
    ));
}

#[derive(PartialEq)]
enum BindgenAttr {
    Catch,
    Constructor,
    Method,
    JsNamespace(syn::Ident),
    Module(String),
    Getter(Option<syn::Ident>),
    Setter(Option<syn::Ident>),
    Structural,
    JsName(syn::Ident),
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
            call!(term, "getter") >>
            val: option!(do_parse!(
                punct!(=) >>
                s: syn!(syn::Ident) >>
                (s)
            )) >>
            (val)
        )=> { BindgenAttr::Getter }
        |
        do_parse!(
            call!(term, "setter") >>
            val: option!(do_parse!(
                punct!(=) >>
                s: syn!(syn::Ident) >>
                (s)
            )) >>
            (val)
        )=> { BindgenAttr::Setter }
        |
        call!(term, "structural") => { |_| BindgenAttr::Structural }
        |
        do_parse!(
            call!(term, "js_namespace") >>
            punct!(=) >>
            ns: syn!(syn::Ident) >>
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
            call!(term, "js_name") >>
            punct!(=) >>
            ns: syn!(syn::Ident) >>
            (ns)
        )=> { BindgenAttr::JsName }
    ));
}

fn extract_first_ty_param(ty: Option<&Type>) -> Option<Option<Type>> {
    let t = match ty {
        Some(t) => t,
        None => return Some(None),
    };
    let ty = match *t {
        Type { ref ty, kind: TypeKind::ByValue, .. } => ty,
        _ => return None,
    };
    let path = match *ty {
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
    Some(Some(Type {
        ty: ty.clone(),
        kind: TypeKind::ByValue,
        loc: t.loc,
    }))
}

fn term<'a>(cursor: syn::buffer::Cursor<'a>, name: &str) -> syn::synom::PResult<'a, ()> {
    if let Some((term, next)) = cursor.term() {
        if term.as_str() == name {
            return Ok(((), next));
        }
    }
    syn::parse_error()
}
