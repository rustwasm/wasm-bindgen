use std::cell::RefCell;
use std::collections::HashMap;

use proc_macro2::{Ident, Span};

use Diagnostic;
use ast;

pub fn encode(program: &ast::Program) -> Result<Vec<u8>, Diagnostic> {
    let mut e = Encoder::new();
    let i = Interner::new();
    shared_program(program, &i)?.encode(&mut e);
    Ok(e.finish())
}

struct Interner {
    map: RefCell<HashMap<Ident, String>>,
}

impl Interner {
    fn new() -> Interner {
        Interner { map: RefCell::new(HashMap::new()) }
    }

    fn intern(&self, s: &Ident) -> &str {
        let mut map = self.map.borrow_mut();
        if let Some(s) = map.get(s) {
            return unsafe { &*(&**s as *const str) }
        }
        map.insert(s.clone(), s.to_string());
        unsafe { &*(&*map[s] as *const str) }
    }

    fn intern_str(&self, s: &str) -> &str {
        self.intern(&Ident::new(s, Span::call_site()))
    }
}

fn shared_program<'a>(prog: &'a ast::Program, intern: &'a Interner)
    -> Result<Program<'a>, Diagnostic>
{
    Ok(Program {
        exports: prog.exports.iter().map(|a| shared_export(a, intern)).collect(),
        structs: prog.structs.iter().map(|a| shared_struct(a, intern)).collect(),
        enums: prog.enums.iter().map(|a| shared_enum(a, intern)).collect(),
        imports: prog.imports.iter()
            .map(|a| shared_import(a, intern))
            .collect::<Result<Vec<_>, _>>()?,
        typescript_custom_sections: prog.typescript_custom_sections.iter().map(|x| -> &'a str { &x }).collect(),
        // version: shared::version(),
        // schema_version: shared::SCHEMA_VERSION.to_string(),
    })
}

fn shared_export<'a>(export: &'a ast::Export, intern: &'a Interner) -> Export<'a> {
    let (method, consumed) = match export.method_self {
        Some(ast::MethodSelf::ByValue) => (true, true),
        Some(_) => (true, false),
        None => (false, false),
    };
    Export {
        class: export.js_class.as_ref().map(|s| &**s),
        method,
        consumed,
        is_constructor: export.is_constructor,
        function: shared_function(&export.function, intern),
        comments: export.comments.iter().map(|s| &**s).collect(),
    }
}

fn shared_function<'a>(func: &'a ast::Function, _intern: &'a Interner) -> Function<'a> {
    Function {
        name: &func.name,
    }
}

fn shared_enum<'a>(e: &'a ast::Enum, intern: &'a Interner) -> Enum<'a> {
    Enum {
        name: intern.intern(&e.name),
        variants: e.variants.iter().map(|v| shared_variant(v, intern)).collect(),
        comments: e.comments.iter().map(|s| &**s).collect(),
    }
}

fn shared_variant<'a>(v: &'a ast::Variant, intern: &'a Interner) -> EnumVariant<'a> {
    EnumVariant {
        name: intern.intern(&v.name),
        value: v.value,
    }
}

fn shared_import<'a>(i: &'a ast::Import, intern: &'a Interner)
    -> Result<Import<'a>, Diagnostic>
{
    Ok(Import {
        module: i.module.as_ref().map(|s| &**s),
        js_namespace: i.js_namespace.as_ref().map(|s| intern.intern(s)),
        kind: shared_import_kind(&i.kind, intern)?,
    })
}

fn shared_import_kind<'a>(i: &'a ast::ImportKind, intern: &'a Interner)
    -> Result<ImportKind<'a>, Diagnostic>
{
    Ok(match i {
        ast::ImportKind::Function(f) => ImportKind::Function(shared_import_function(f, intern)?),
        ast::ImportKind::Static(f) => ImportKind::Static(shared_import_static(f, intern)),
        ast::ImportKind::Type(f) => ImportKind::Type(shared_import_type(f, intern)),
        ast::ImportKind::Enum(f) => ImportKind::Enum(shared_import_enum(f, intern)),
    })
}

fn shared_import_function<'a>(i: &'a ast::ImportFunction, intern: &'a Interner)
    -> Result<ImportFunction<'a>, Diagnostic>
{
    let method = match &i.kind {
        ast::ImportFunctionKind::Method { class, kind, ..  } => {
            let kind = match kind {
                ast::MethodKind::Constructor => MethodKind::Constructor,
                ast::MethodKind::Operation(ast::Operation { is_static, kind }) => {
                    let is_static = *is_static;
                    let kind = match kind {
                        ast::OperationKind::Regular => OperationKind::Regular,
                        ast::OperationKind::Getter(g) => {
                            let g = g.as_ref().map(|g| intern.intern(g));
                            OperationKind::Getter(
                                g.unwrap_or_else(|| i.infer_getter_property()),
                            )
                        }
                        ast::OperationKind::Setter(s) => {
                            let s = s.as_ref().map(|s| intern.intern(s));
                            OperationKind::Setter(match s {
                                Some(s) => s,
                                None => intern.intern_str(&i.infer_setter_property()?),
                            })
                        }
                        ast::OperationKind::IndexingGetter => OperationKind::IndexingGetter,
                        ast::OperationKind::IndexingSetter => OperationKind::IndexingSetter,
                        ast::OperationKind::IndexingDeleter => OperationKind::IndexingDeleter,
                    };
                    MethodKind::Operation(Operation { is_static, kind })
                }
            };
            Some(MethodData {
                class,
                kind,
            })
        }
        ast::ImportFunctionKind::Normal => None,
    };

    Ok(ImportFunction {
        shim: intern.intern(&i.shim),
        catch: i.catch,
        method,
        structural: i.structural,
        function: shared_function(&i.function, intern),
        variadic: i.variadic,
    })
}

fn shared_import_static<'a>(i: &'a ast::ImportStatic, intern: &'a Interner)
    -> ImportStatic<'a>
{
    ImportStatic {
        name: &i.js_name,
        shim: intern.intern(&i.shim),
    }
}

fn shared_import_type<'a>(i: &'a ast::ImportType, intern: &'a Interner)
    -> ImportType<'a>
{
    ImportType {
        name: &i.js_name,
        instanceof_shim: &i.instanceof_shim,
        vendor_prefixes: i.vendor_prefixes.iter()
            .map(|x| intern.intern(x))
            .collect(),
    }
}

fn shared_import_enum<'a>(_i: &'a ast::ImportEnum, _intern: &'a Interner)
    -> ImportEnum
{
    ImportEnum {}
}

fn shared_struct<'a>(s: &'a ast::Struct, intern: &'a Interner) -> Struct<'a> {
    Struct {
        name: &s.js_name,
        fields: s.fields.iter().map(|s| shared_struct_field(s, intern)).collect(),
        comments: s.comments.iter().map(|s| &**s).collect(),
    }
}

fn shared_struct_field<'a>(s: &'a ast::StructField, intern: &'a Interner)
    -> StructField<'a>
{
    StructField {
        name: intern.intern(&s.name),
        readonly: s.readonly,
        comments: s.comments.iter().map(|s| &**s).collect(),
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
shared_api!(encode_api);
