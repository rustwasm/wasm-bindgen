use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use shared;
use syn;
use util;

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Default)]
pub struct Program {
    pub exports: Vec<Export>,
    pub imports: Vec<Import>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub type_aliases: Vec<TypeAlias>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct Export {
    pub class: Option<Ident>,
    pub method_self: Option<MethodSelf>,
    pub constructor: Option<String>,
    pub function: Function,
    pub comments: Vec<String>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub enum MethodSelf {
    ByValue,
    RefMutable,
    RefShared,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct Import {
    pub module: Option<String>,
    pub version: Option<String>,
    pub js_namespace: Option<Ident>,
    pub kind: ImportKind,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub enum ImportKind {
    Function(ImportFunction),
    Static(ImportStatic),
    Type(ImportType),
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct ImportFunction {
    pub function: Function,
    pub rust_name: Ident,
    pub js_ret: Option<syn::Type>,
    pub kind: ImportFunctionKind,
    pub shim: Ident,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub enum ImportFunctionKind {
    Method {
        class: String,
        ty: syn::Type,
        kind: MethodKind,
    },
    Normal,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub enum MethodKind {
    Normal,
    Constructor,
    Static,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct ImportStatic {
    pub vis: syn::Visibility,
    pub ty: syn::Type,
    pub shim: Ident,
    pub rust_name: Ident,
    pub js_name: Ident,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct ImportType {
    pub vis: syn::Visibility,
    pub name: Ident,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct Function {
    pub name: Ident,
    pub arguments: Vec<syn::ArgCaptured>,
    pub ret: Option<syn::Type>,
    pub opts: BindgenAttrs,
    pub rust_attrs: Vec<syn::Attribute>,
    pub rust_vis: syn::Visibility,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct Struct {
    pub name: Ident,
    pub fields: Vec<StructField>,
    pub comments: Vec<String>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct StructField {
    pub opts: BindgenAttrs,
    pub name: Ident,
    pub struct_name: Ident,
    pub ty: syn::Type,
    pub getter: Ident,
    pub setter: Ident,
    pub comments: Vec<String>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct Enum {
    pub name: Ident,
    pub variants: Vec<Variant>,
    pub comments: Vec<String>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct Variant {
    pub name: Ident,
    pub value: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypeKind {
    ByRef,
    ByMutRef,
    ByValue,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypeLocation {
    ImportArgument,
    ImportRet,
    ExportArgument,
    ExportRet,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct TypeAlias {
    pub vis: syn::Visibility,
    pub dest: Ident,
    pub src: syn::Type,
}

impl Program {
    pub fn push_item(
        &mut self,
        item: syn::Item,
        opts: Option<BindgenAttrs>,
        tokens: &mut TokenStream,
    ) {
        match item {
            syn::Item::Fn(mut f) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut f.attrs));

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
                self.exports.push(Export {
                    class: None,
                    method_self: None,
                    constructor: None,
                    function: Function::from(f, opts),
                    comments,
                });
            }
            syn::Item::Struct(mut s) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut s.attrs));
                self.structs.push(Struct::from(&mut s, opts));
                s.to_tokens(tokens);
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
        for item in item.items.iter_mut() {
            self.push_impl_item(&name, item);
        }
    }

    fn push_impl_item(&mut self, class: &Ident, item: &mut syn::ImplItem) {
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

        let (function, method_self) = Function::from_decl(
            &method.sig.ident,
            Box::new(method.sig.decl.clone()),
            method.attrs.clone(),
            opts,
            method.vis.clone(),
            true,
        );

        self.exports.push(Export {
            class: Some(class.clone()),
            method_self,
            constructor,
            function,
            comments,
        });
    }

    pub fn push_enum(&mut self, item: syn::ItemEnum, _opts: BindgenAttrs) {
        match item.vis {
            syn::Visibility::Public(_) => {}
            _ => panic!("only public enums are allowed"),
        }

        let variants = item
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

                Variant {
                    name: v.ident.clone(),
                    value,
                }
            })
            .collect();
        let comments = extract_doc_comments(&item.attrs);
        self.enums.push(Enum {
            name: item.ident,
            variants,
            comments,
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
            let version = item_opts
                .version()
                .or(opts.version())
                .map(|s| s.to_string());
            let js_namespace = item_opts.js_namespace().or(opts.js_namespace()).cloned();
            let mut kind = match item {
                syn::ForeignItem::Fn(f) => self.push_foreign_fn(f, item_opts),
                syn::ForeignItem::Type(t) => self.push_foreign_ty(t),
                syn::ForeignItem::Static(s) => self.push_foreign_static(s, item_opts),
                _ => panic!("only foreign functions/types allowed for now"),
            };

            self.imports.push(Import {
                module,
                version,
                js_namespace,
                kind,
            });
        }
    }

    pub fn push_foreign_fn(&mut self, f: syn::ForeignItemFn, opts: BindgenAttrs) -> ImportKind {
        let js_name = opts.js_name().unwrap_or(&f.ident).clone();
        let wasm = Function::from_decl(&js_name, f.decl, f.attrs, opts, f.vis, false).0;
        let js_ret = if wasm.opts.catch() {
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

        let kind = if wasm.opts.method() {
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
            let class_name = wasm
                .opts
                .js_class()
                .map(Into::into)
                .unwrap_or_else(|| class_name.to_string());

            ImportFunctionKind::Method {
                class: class_name,
                ty: class.clone(),
                kind: MethodKind::Normal,
            }
        } else if let Some(cls) = wasm.opts.static_method_of() {
            let class = cls.to_string();
            let kind = MethodKind::Static;
            let ty = util::ident_ty(cls.clone());
            ImportFunctionKind::Method { class, ty, kind }
        } else if wasm.opts.constructor() {
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

            ImportFunctionKind::Method {
                class: class_name.to_string(),
                ty: class.clone(),
                kind: MethodKind::Constructor,
            }
        } else {
            ImportFunctionKind::Normal
        };

        let shim = {
            let ns = match kind {
                ImportFunctionKind::Normal => "n",
                ImportFunctionKind::Method { ref class, .. } => class,
            };
            format!("__wbg_f_{}_{}_{}", js_name, f.ident, ns)
        };
        ImportKind::Function(ImportFunction {
            function: wasm,
            kind,
            js_ret,
            rust_name: f.ident.clone(),
            shim: Ident::new(&shim, Span::call_site()),
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
        let js_name = opts.js_name().unwrap_or(&f.ident);
        let shim = format!("__wbg_static_accessor_{}_{}", js_name, f.ident);
        ImportKind::Static(ImportStatic {
            ty: *f.ty,
            vis: f.vis,
            rust_name: f.ident.clone(),
            js_name: js_name.clone(),
            shim: Ident::new(&shim, Span::call_site()),
        })
    }

    pub fn shared(&self) -> shared::Program {
        shared::Program {
            exports: self.exports.iter().map(|a| a.shared()).collect(),
            structs: self.structs.iter().map(|a| a.shared()).collect(),
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
            &input.ident,
            input.decl,
            input.attrs,
            opts,
            input.vis,
            false,
        ).0
    }

    pub fn from_decl(
        name: &Ident,
        mut decl: Box<syn::FnDecl>,
        attrs: Vec<syn::Attribute>,
        opts: BindgenAttrs,
        vis: syn::Visibility,
        allow_self: bool,
    ) -> (Function, Option<MethodSelf>) {
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
                    method_self = Some(MethodSelf::ByValue);
                    None
                }
                syn::FnArg::SelfRef(ref a) if allow_self => {
                    assert!(method_self.is_none());
                    if a.mutability.is_some() {
                        method_self = Some(MethodSelf::RefMutable);
                    } else {
                        method_self = Some(MethodSelf::RefShared);
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
            Function {
                name: name.clone(),
                arguments,
                ret,
                opts,
                rust_vis: vis,
                rust_attrs: attrs,
            },
            method_self,
        )
    }

    fn shared(&self) -> shared::Function {
        shared::Function {
            name: self.name.to_string(),
        }
    }
}

pub fn extract_path_ident(path: &syn::Path) -> Option<Ident> {
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

impl Export {
    pub fn rust_symbol(&self) -> Ident {
        let mut generated_name = format!("__wasm_bindgen_generated");
        if let Some(class) = &self.class {
            generated_name.push_str("_");
            generated_name.push_str(&class.to_string());
        }
        generated_name.push_str("_");
        generated_name.push_str(&self.function.name.to_string());
        Ident::new(&generated_name, Span::call_site())
    }

    pub fn export_name(&self) -> String {
        let fn_name = self.function.name.to_string();
        match &self.class {
            Some(class) => shared::struct_function_export_name(&class.to_string(), &fn_name),
            None => shared::free_function_export_name(&fn_name),
        }
    }

    fn shared(&self) -> shared::Export {
        let (method, consumed) = match self.method_self {
            Some(MethodSelf::ByValue) => (true, true),
            Some(_) => (true, false),
            None => (false, false),
        };
        shared::Export {
            class: self.class.as_ref().map(|s| s.to_string()),
            method,
            consumed,
            constructor: self.constructor.clone(),
            function: self.function.shared(),
            comments: self.comments.clone(),
        }
    }
}

impl Enum {
    fn shared(&self) -> shared::Enum {
        shared::Enum {
            name: self.name.to_string(),
            variants: self.variants.iter().map(|v| v.shared()).collect(),
            comments: self.comments.clone(),
        }
    }
}

impl Variant {
    fn shared(&self) -> shared::EnumVariant {
        shared::EnumVariant {
            name: self.name.to_string(),
            value: self.value,
        }
    }
}

impl Import {
    fn shared(&self) -> shared::Import {
        match (&self.module, &self.version) {
            (&Some(ref m), None) if m.starts_with("./") => {}
            (&Some(ref m), &Some(_)) if m.starts_with("./") => {
                panic!(
                    "when a module path starts with `./` that indicates \
                     that a local file is being imported so the `version` \
                     key cannot also be specified"
                );
            }
            (&Some(_), &Some(_)) => {}
            (&Some(_), &None) => panic!(
                "when the `module` directive doesn't start with `./` \
                 then it's interpreted as an NPM package which requires \
                 a `version` to be specified as well, try using \
                 #[wasm_bindgen(module = \"...\", version = \"...\")]"
            ),
            (&None, &Some(_)) => {
                panic!(
                    "the #[wasm_bindgen(version = \"...\")] attribute can only \
                     be used when `module = \"...\"` is also specified"
                );
            }
            (&None, &None) => {}
        }
        shared::Import {
            module: self.module.clone(),
            version: self.version.clone(),
            js_namespace: self.js_namespace.as_ref().map(|s| s.to_string()),
            kind: self.kind.shared(),
        }
    }
}

impl ImportKind {
    pub fn fits_on_impl(&self) -> bool {
        match *self {
            ImportKind::Function(_) => true,
            ImportKind::Static(_) => false,
            ImportKind::Type(_) => false,
        }
    }

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
        self.function.name.to_string()
    }

    pub fn infer_setter_property(&self) -> String {
        let name = self.function.name.to_string();
        assert!(name.starts_with("set_"), "setters must start with `set_`");
        name[4..].to_string()
    }

    fn shared(&self) -> shared::ImportFunction {
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

        let mut method = None;
        match self.kind {
            ImportFunctionKind::Method {
                ref class,
                ref kind,
                ..
            } => {
                let kind = match kind {
                    MethodKind::Normal => shared::MethodKind::Normal,
                    MethodKind::Constructor => shared::MethodKind::Constructor,
                    MethodKind::Static => shared::MethodKind::Static,
                };
                method = Some(shared::MethodData {
                    class: class.clone(),
                    kind,
                    getter,
                    setter,
                });
            }
            ImportFunctionKind::Normal => {}
        }

        shared::ImportFunction {
            shim: self.shim.to_string(),
            catch: self.function.opts.catch(),
            method,
            structural: self.function.opts.structural(),
            function: self.function.shared(),
        }
    }
}

impl ImportStatic {
    fn shared(&self) -> shared::ImportStatic {
        shared::ImportStatic {
            name: self.js_name.to_string(),
            shim: self.shim.to_string(),
        }
    }
}

impl ImportType {
    fn shared(&self) -> shared::ImportType {
        shared::ImportType {}
    }
}

impl Struct {
    fn from(s: &mut syn::ItemStruct, _opts: BindgenAttrs) -> Struct {
        if s.generics.params.len() > 0 {
            panic!(
                "structs with #[wasm_bindgen] cannot have lifetime or \
                 type parameters currently"
            );
        }
        let mut fields = Vec::new();
        if let syn::Fields::Named(names) = &mut s.fields {
            for field in names.named.iter_mut() {
                match field.vis {
                    syn::Visibility::Public(..) => {}
                    _ => continue,
                }
                let name = match &field.ident {
                    Some(n) => n,
                    None => continue,
                };
                let ident = s.ident.to_string();
                let name_str = name.to_string();
                let getter = shared::struct_field_get(&ident, &name_str);
                let setter = shared::struct_field_set(&ident, &name_str);
                let opts = BindgenAttrs::find(&mut field.attrs);
                let comments = extract_doc_comments(&field.attrs);
                fields.push(StructField {
                    opts,
                    name: name.clone(),
                    struct_name: s.ident.clone(),
                    ty: field.ty.clone(),
                    getter: Ident::new(&getter, Span::call_site()),
                    setter: Ident::new(&setter, Span::call_site()),
                    comments,
                });
            }
        }
        let comments: Vec<String> = extract_doc_comments(&s.attrs);
        Struct {
            name: s.ident.clone(),
            fields,
            comments,
        }
    }

    fn shared(&self) -> shared::Struct {
        shared::Struct {
            name: self.name.to_string(),
            fields: self.fields.iter().map(|s| s.shared()).collect(),
            comments: self.comments.clone(),
        }
    }
}

impl StructField {
    fn shared(&self) -> shared::StructField {
        shared::StructField {
            name: self.name.to_string(),
            readonly: self.opts.readonly(),
            comments: self.comments.clone(),
        }
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Default)]
pub struct BindgenAttrs {
    pub attrs: Vec<BindgenAttr>,
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

    pub fn catch(&self) -> bool {
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

    pub fn getter(&self) -> Option<Option<&Ident>> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::Getter(s) => Some(s.as_ref()),
                _ => None,
            })
            .next()
    }

    pub fn setter(&self) -> Option<Option<&Ident>> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                BindgenAttr::Setter(s) => Some(s.as_ref()),
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

    pub fn readonly(&self) -> bool {
        self.attrs.iter().any(|a| match *a {
            BindgenAttr::Readonly => true,
            _ => false,
        })
    }

    pub fn js_name(&self) -> Option<&Ident> {
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
