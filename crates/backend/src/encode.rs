use crate::util::ShortHash;
use proc_macro2::{Ident, Span};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::ast;
use crate::Diagnostic;

pub struct EncodeResult {
    pub custom_section: Vec<u8>,
    pub included_files: Vec<PathBuf>,
}

pub fn encode(program: &ast::Program) -> Result<EncodeResult, Diagnostic> {
    let mut e = Encoder::new();
    let i = Interner::new();
    shared_program(program, &i)?.encode(&mut e);
    let custom_section = e.finish();
    let included_files = i
        .files
        .borrow()
        .values()
        .map(|p| &p.path)
        .cloned()
        .collect();
    Ok(EncodeResult {
        custom_section,
        included_files,
    })
}

struct Interner {
    bump: bumpalo::Bump,
    files: RefCell<HashMap<String, LocalFile>>,
    root: PathBuf,
    crate_name: String,
    has_package_json: Cell<bool>,
}

struct LocalFile {
    path: PathBuf,
    definition: Span,
    new_identifier: String,
}

impl Interner {
    fn new() -> Interner {
        Interner {
            bump: bumpalo::Bump::new(),
            files: RefCell::new(HashMap::new()),
            root: env::var_os("CARGO_MANIFEST_DIR").unwrap().into(),
            crate_name: env::var("CARGO_PKG_NAME").unwrap(),
            has_package_json: Cell::new(false),
        }
    }

    fn intern(&self, s: &Ident) -> &str {
        self.intern_str(&s.to_string())
    }

    fn intern_str(&self, s: &str) -> &str {
        // NB: eventually this could be used to intern `s` to only allocate one
        // copy, but for now let's just "transmute" `s` to have the same
        // lifetime as this struct itself (which is our main goal here)
        self.bump.alloc_str(s)
    }

    /// Given an import to a local module `id` this generates a unique module id
    /// to assign to the contents of `id`.
    ///
    /// Note that repeated invocations of this function will be memoized, so the
    /// same `id` will always return the same resulting unique `id`.
    fn resolve_import_module(&self, id: &str, span: Span) -> Result<&str, Diagnostic> {
        let mut files = self.files.borrow_mut();
        if let Some(file) = files.get(id) {
            return Ok(self.intern_str(&file.new_identifier));
        }
        self.check_for_package_json();
        let path = if id.starts_with("/") {
            self.root.join(&id[1..])
        } else if id.starts_with("./") || id.starts_with("../") {
            let msg = "relative module paths aren't supported yet";
            return Err(Diagnostic::span_error(span, msg));
        } else {
            return Ok(self.intern_str(&id));
        };

        // Generate a unique ID which is somewhat readable as well, so mix in
        // the crate name, hash to make it unique, and then the original path.
        let new_identifier = format!("{}{}", self.unique_crate_identifier(), id);
        let file = LocalFile {
            path,
            definition: span,
            new_identifier,
        };
        files.insert(id.to_string(), file);
        drop(files);
        self.resolve_import_module(id, span)
    }

    fn unique_crate_identifier(&self) -> String {
        format!("{}-{}", self.crate_name, ShortHash(0))
    }

    fn check_for_package_json(&self) {
        if self.has_package_json.get() {
            return;
        }
        let path = self.root.join("package.json");
        if path.exists() {
            self.has_package_json.set(true);
        }
    }
}

fn shared_program<'a>(
    prog: &'a ast::Program,
    intern: &'a Interner,
) -> Result<Program<'a>, Diagnostic> {
    Ok(Program {
        exports: prog
            .exports
            .iter()
            .map(|a| shared_export(a, intern))
            .collect::<Result<Vec<_>, _>>()?,
        structs: prog
            .structs
            .iter()
            .map(|a| shared_struct(a, intern))
            .collect(),
        enums: prog.enums.iter().map(|a| shared_enum(a, intern)).collect(),
        imports: prog
            .imports
            .iter()
            .map(|a| shared_import(a, intern))
            .collect::<Result<Vec<_>, _>>()?,
        typescript_custom_sections: prog
            .typescript_custom_sections
            .iter()
            .map(|x| -> &'a str { &x })
            .collect(),
        local_modules: intern
            .files
            .borrow()
            .values()
            .map(|file| {
                fs::read_to_string(&file.path)
                    .map(|s| LocalModule {
                        identifier: intern.intern_str(&file.new_identifier),
                        contents: intern.intern_str(&s),
                    })
                    .map_err(|e| {
                        let msg = format!("failed to read file `{}`: {}", file.path.display(), e);
                        Diagnostic::span_error(file.definition, msg)
                    })
            })
            .collect::<Result<Vec<_>, _>>()?,
        inline_js: prog
            .inline_js
            .iter()
            .map(|js| intern.intern_str(js))
            .collect(),
        unique_crate_identifier: intern.intern_str(&intern.unique_crate_identifier()),
        package_json: if intern.has_package_json.get() {
            Some(intern.intern_str(intern.root.join("package.json").to_str().unwrap()))
        } else {
            None
        },
    })
}

fn shared_export<'a>(
    export: &'a ast::Export,
    intern: &'a Interner,
) -> Result<Export<'a>, Diagnostic> {
    let consumed = match export.method_self {
        Some(ast::MethodSelf::ByValue) => true,
        _ => false,
    };
    let method_kind = from_ast_method_kind(&export.function, intern, &export.method_kind)?;
    Ok(Export {
        class: export.js_class.as_ref().map(|s| &**s),
        comments: export.comments.iter().map(|s| &**s).collect(),
        consumed,
        function: shared_function(&export.function, intern),
        method_kind,
        start: export.start,
    })
}

fn shared_function<'a>(func: &'a ast::Function, _intern: &'a Interner) -> Function<'a> {
    let arg_names = func
        .arguments
        .iter()
        .enumerate()
        .map(|(idx, arg)| {
            if let syn::Pat::Ident(x) = &*arg.pat {
                return x.ident.to_string();
            }
            format!("arg{}", idx)
        })
        .collect::<Vec<_>>();
    Function {
        arg_names,
        name: &func.name,
        generate_typescript: func.generate_typescript,
    }
}

fn shared_enum<'a>(e: &'a ast::Enum, intern: &'a Interner) -> Enum<'a> {
    Enum {
        name: &e.js_name,
        variants: e
            .variants
            .iter()
            .map(|v| shared_variant(v, intern))
            .collect(),
        comments: e.comments.iter().map(|s| &**s).collect(),
        generate_typescript: e.generate_typescript,
    }
}

fn shared_variant<'a>(v: &'a ast::Variant, intern: &'a Interner) -> EnumVariant<'a> {
    EnumVariant {
        name: intern.intern(&v.name),
        value: v.value,
        comments: v.comments.iter().map(|s| &**s).collect(),
    }
}

fn shared_import<'a>(i: &'a ast::Import, intern: &'a Interner) -> Result<Import<'a>, Diagnostic> {
    Ok(Import {
        module: match &i.module {
            ast::ImportModule::Named(m, span) => {
                ImportModule::Named(intern.resolve_import_module(m, *span)?)
            }
            ast::ImportModule::RawNamed(m, _span) => ImportModule::RawNamed(intern.intern_str(m)),
            ast::ImportModule::Inline(idx, _) => ImportModule::Inline(*idx as u32),
            ast::ImportModule::None => ImportModule::None,
        },
        js_namespace: i.js_namespace.clone(),
        kind: shared_import_kind(&i.kind, intern)?,
    })
}

fn shared_import_kind<'a>(
    i: &'a ast::ImportKind,
    intern: &'a Interner,
) -> Result<ImportKind<'a>, Diagnostic> {
    Ok(match i {
        ast::ImportKind::Function(f) => ImportKind::Function(shared_import_function(f, intern)?),
        ast::ImportKind::Static(f) => ImportKind::Static(shared_import_static(f, intern)),
        ast::ImportKind::Type(f) => ImportKind::Type(shared_import_type(f, intern)),
        ast::ImportKind::Enum(f) => ImportKind::Enum(shared_import_enum(f, intern)),
    })
}

fn shared_import_function<'a>(
    i: &'a ast::ImportFunction,
    intern: &'a Interner,
) -> Result<ImportFunction<'a>, Diagnostic> {
    let method = match &i.kind {
        ast::ImportFunctionKind::Method { class, kind, .. } => {
            let kind = from_ast_method_kind(&i.function, intern, kind)?;
            Some(MethodData { class, kind })
        }
        ast::ImportFunctionKind::Normal => None,
    };

    Ok(ImportFunction {
        shim: intern.intern(&i.shim),
        catch: i.catch,
        method,
        assert_no_shim: i.assert_no_shim,
        structural: i.structural,
        function: shared_function(&i.function, intern),
        variadic: i.variadic,
    })
}

fn shared_import_static<'a>(i: &'a ast::ImportStatic, intern: &'a Interner) -> ImportStatic<'a> {
    ImportStatic {
        name: &i.js_name,
        shim: intern.intern(&i.shim),
    }
}

fn shared_import_type<'a>(i: &'a ast::ImportType, intern: &'a Interner) -> ImportType<'a> {
    ImportType {
        name: &i.js_name,
        instanceof_shim: &i.instanceof_shim,
        vendor_prefixes: i.vendor_prefixes.iter().map(|x| intern.intern(x)).collect(),
    }
}

fn shared_import_enum<'a>(_i: &'a ast::ImportEnum, _intern: &'a Interner) -> ImportEnum {
    ImportEnum {}
}

fn shared_struct<'a>(s: &'a ast::Struct, intern: &'a Interner) -> Struct<'a> {
    Struct {
        name: &s.js_name,
        fields: s
            .fields
            .iter()
            .map(|s| shared_struct_field(s, intern))
            .collect(),
        comments: s.comments.iter().map(|s| &**s).collect(),
        is_inspectable: s.is_inspectable,
        generate_typescript: s.generate_typescript,
    }
}

fn shared_struct_field<'a>(s: &'a ast::StructField, intern: &'a Interner) -> StructField<'a> {
    StructField {
        name: match &s.name {
            syn::Member::Named(ident) => intern.intern(ident),
            syn::Member::Unnamed(index) => intern.intern_str(&index.index.to_string()),
        },
        readonly: s.readonly,
        comments: s.comments.iter().map(|s| &**s).collect(),
        generate_typescript: s.generate_typescript,
    }
}

trait Encode {
    fn encode(&self, dst: &mut Encoder);
}

struct Encoder {
    dst: Vec<u8>,
}

impl Encoder {
    fn new() -> Encoder {
        Encoder {
            dst: vec![0, 0, 0, 0],
        }
    }

    fn finish(mut self) -> Vec<u8> {
        let len = self.dst.len() - 4;
        self.dst[0] = (len >> 0) as u8;
        self.dst[1] = (len >> 8) as u8;
        self.dst[2] = (len >> 16) as u8;
        self.dst[3] = (len >> 24) as u8;
        self.dst
    }

    fn byte(&mut self, byte: u8) {
        self.dst.push(byte);
    }
}

impl Encode for bool {
    fn encode(&self, dst: &mut Encoder) {
        dst.byte(*self as u8);
    }
}

impl Encode for u32 {
    fn encode(&self, dst: &mut Encoder) {
        let mut val = *self;
        while (val >> 7) != 0 {
            dst.byte((val as u8) | 0x80);
            val >>= 7;
        }
        assert_eq!(val >> 7, 0);
        dst.byte(val as u8);
    }
}

impl Encode for usize {
    fn encode(&self, dst: &mut Encoder) {
        assert!(*self <= u32::max_value() as usize);
        (*self as u32).encode(dst);
    }
}

impl<'a> Encode for &'a [u8] {
    fn encode(&self, dst: &mut Encoder) {
        self.len().encode(dst);
        dst.dst.extend_from_slice(*self);
    }
}

impl<'a> Encode for &'a str {
    fn encode(&self, dst: &mut Encoder) {
        self.as_bytes().encode(dst);
    }
}

impl<'a> Encode for String {
    fn encode(&self, dst: &mut Encoder) {
        self.as_bytes().encode(dst);
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, dst: &mut Encoder) {
        self.len().encode(dst);
        for item in self {
            item.encode(dst);
        }
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, dst: &mut Encoder) {
        match self {
            None => dst.byte(0),
            Some(val) => {
                dst.byte(1);
                val.encode(dst)
            }
        }
    }
}

macro_rules! encode_struct {
    ($name:ident ($($lt:tt)*) $($field:ident: $ty:ty,)*) => {
        struct $name $($lt)* {
            $($field: $ty,)*
        }

        impl $($lt)* Encode for $name $($lt)* {
            fn encode(&self, _dst: &mut Encoder) {
                $(self.$field.encode(_dst);)*
            }
        }
    }
}

macro_rules! encode_enum {
    ($name:ident ($($lt:tt)*) $($fields:tt)*) => (
        enum $name $($lt)* { $($fields)* }

        impl$($lt)* Encode for $name $($lt)* {
            fn encode(&self, dst: &mut Encoder) {
                use self::$name::*;
                encode_enum!(@arms self dst (0) () $($fields)*)
            }
        }
    );

    (@arms $me:ident $dst:ident ($cnt:expr) ($($arms:tt)*)) => (
        encode_enum!(@expr match $me { $($arms)* })
    );

    (@arms $me:ident $dst:ident ($cnt:expr) ($($arms:tt)*) $name:ident, $($rest:tt)*) => (
        encode_enum!(
            @arms
            $me
            $dst
            ($cnt+1)
            ($($arms)* $name => $dst.byte($cnt),)
            $($rest)*
        )
    );

    (@arms $me:ident $dst:ident ($cnt:expr) ($($arms:tt)*) $name:ident($t:ty), $($rest:tt)*) => (
        encode_enum!(
            @arms
            $me
            $dst
            ($cnt+1)
            ($($arms)* $name(val) => { $dst.byte($cnt); val.encode($dst) })
            $($rest)*
        )
    );

    (@expr $e:expr) => ($e);
}

macro_rules! encode_api {
    () => ();
    (struct $name:ident<'a> { $($fields:tt)* } $($rest:tt)*) => (
        encode_struct!($name (<'a>) $($fields)*);
        encode_api!($($rest)*);
    );
    (struct $name:ident { $($fields:tt)* } $($rest:tt)*) => (
        encode_struct!($name () $($fields)*);
        encode_api!($($rest)*);
    );
    (enum $name:ident<'a> { $($variants:tt)* } $($rest:tt)*) => (
        encode_enum!($name (<'a>) $($variants)*);
        encode_api!($($rest)*);
    );
    (enum $name:ident { $($variants:tt)* } $($rest:tt)*) => (
        encode_enum!($name () $($variants)*);
        encode_api!($($rest)*);
    );
}
wasm_bindgen_shared::shared_api!(encode_api);

fn from_ast_method_kind<'a>(
    function: &'a ast::Function,
    intern: &'a Interner,
    method_kind: &'a ast::MethodKind,
) -> Result<MethodKind<'a>, Diagnostic> {
    Ok(match method_kind {
        ast::MethodKind::Constructor => MethodKind::Constructor,
        ast::MethodKind::Operation(ast::Operation { is_static, kind }) => {
            let is_static = *is_static;
            let kind = match kind {
                ast::OperationKind::Getter(g) => {
                    let g = g.as_ref().map(|g| intern.intern(g));
                    OperationKind::Getter(g.unwrap_or_else(|| function.infer_getter_property()))
                }
                ast::OperationKind::Regular => OperationKind::Regular,
                ast::OperationKind::Setter(s) => {
                    let s = s.as_ref().map(|s| intern.intern(s));
                    OperationKind::Setter(match s {
                        Some(s) => s,
                        None => intern.intern_str(&function.infer_setter_property()?),
                    })
                }
                ast::OperationKind::IndexingGetter => OperationKind::IndexingGetter,
                ast::OperationKind::IndexingSetter => OperationKind::IndexingSetter,
                ast::OperationKind::IndexingDeleter => OperationKind::IndexingDeleter,
            };
            MethodKind::Operation(Operation { is_static, kind })
        }
    })
}
