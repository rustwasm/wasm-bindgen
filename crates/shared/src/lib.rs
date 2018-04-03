#[macro_use]
extern crate serde_derive;
extern crate fnv;

use std::hash::{Hash, Hasher};

pub const SCHEMA_VERSION: &str = "1";

#[derive(Deserialize)]
pub struct Program {
    pub exports: Vec<Export>,
    pub enums: Vec<Enum>,
    pub imports: Vec<Import>,
    pub custom_type_names: Vec<CustomTypeName>,
    pub version: String,
    pub schema_version: String,
}

#[derive(Deserialize)]
pub struct Import {
    pub module: Option<String>,
    pub js_namespace: Option<String>,
    pub kind: ImportKind,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ImportKind {
    Function(ImportFunction),
    Static(ImportStatic),
    Type(ImportType),
}

#[derive(Deserialize)]
pub struct ImportFunction {
    pub shim: String,
    pub module: Option<String>,
    pub catch: bool,
    pub method: bool,
    pub js_new: bool,
    pub structural: bool,
    pub getter: Option<String>,
    pub setter: Option<String>,
    pub class: Option<String>,
    pub function: Function,
}

#[derive(Deserialize)]
pub struct ImportStatic {
    pub module: Option<String>,
    pub name: String,
    pub shim: String,
}

#[derive(Deserialize)]
pub struct ImportType {
}

#[derive(Deserialize)]
pub struct Export {
    pub class: Option<String>,
    pub method: bool,
    pub function: Function,
}

#[derive(Deserialize)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub value: u32
}

#[derive(Deserialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Type>,
    pub ret: Option<Type>,
}

#[derive(Deserialize)]
pub struct CustomTypeName {
    pub descriptor: u32,
    pub name: String,
}

pub fn new_function(struct_name: &str) -> String {
    let mut name = format!("__wbg_");
    name.extend(struct_name
        .chars()
        .flat_map(|s| s.to_lowercase()));
    name.push_str("_new");
    return name
}

pub fn free_function(struct_name: &str) -> String {
    let mut name = format!("__wbg_");
    name.extend(struct_name
        .chars()
        .flat_map(|s| s.to_lowercase()));
    name.push_str("_free");
    return name
}

pub fn free_function_export_name(function_name: &str) -> String {
    function_name.to_string()
}

pub fn struct_function_export_name(struct_: &str, f: &str) -> String {
    let mut name = struct_
        .chars()
        .flat_map(|s| s.to_lowercase())
        .collect::<String>();
    name.push_str("_");
    name.push_str(f);
    return name
}

pub type Type = u32;

pub const TYPE_VECTOR_JSVALUE: u32 = 0;
pub const TYPE_ENUM: u32 = 1;
pub const TYPE_NUMBER: u32 = 2;
pub const TYPE_BORROWED_STR: u32 = 3;
pub const TYPE_STRING: u32 = 4;
pub const TYPE_BOOLEAN: u32 = 5;
pub const TYPE_SLICE_U8: u32 = 6;
pub const TYPE_VECTOR_U8: u32 = 7;
pub const TYPE_SLICE_I8: u32 = 8;
pub const TYPE_VECTOR_I8: u32 = 9;
pub const TYPE_SLICE_U16: u32 = 10;
pub const TYPE_VECTOR_U16: u32 = 11;
pub const TYPE_SLICE_I16: u32 = 12;
pub const TYPE_VECTOR_I16: u32 = 13;
pub const TYPE_SLICE_U32: u32 = 14;
pub const TYPE_VECTOR_U32: u32 = 15;
pub const TYPE_SLICE_I32: u32 = 16;
pub const TYPE_VECTOR_I32: u32 = 17;
pub const TYPE_SLICE_F32: u32 = 18;
pub const TYPE_VECTOR_F32: u32 = 19;
pub const TYPE_SLICE_F64: u32 = 20;
pub const TYPE_VECTOR_F64: u32 = 21;
pub const TYPE_JS_OWNED: u32 = 22;
pub const TYPE_JS_REF: u32 = 23;

pub const TYPE_CUSTOM_START: u32 = 24;
pub const TYPE_CUSTOM_REF_FLAG: u32 = 1;

pub fn name_to_descriptor(name: &str) -> u32 {
    const MAX: u32 = 10_000;
    let mut h = fnv::FnvHasher::default();
    name.hash(&mut h);
    (((h.finish() as u32) % (MAX - TYPE_CUSTOM_START)) + TYPE_CUSTOM_START) & !1

}

pub fn version() -> String {
    let mut v = env!("CARGO_PKG_VERSION").to_string();
    if let Some(s) = option_env!("WBG_VERSION") {
        v.push_str(" (");
        v.push_str(s);
        v.push_str(")");
    }
    return v
}
