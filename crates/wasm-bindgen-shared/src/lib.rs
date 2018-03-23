#[macro_use]
extern crate serde_derive;
extern crate fnv;

use std::char;
use std::hash::{Hash, Hasher};

pub const SCHEMA_VERSION: &str = "0";

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
    pub descriptor: char,
    pub name: String,
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

pub fn mangled_import_name(struct_: Option<&str>, f: &str) -> String {
    match struct_ {
        Some(s) => format!("__wbg_s_{}_{}", s, f),
        None => format!("__wbg_f_{}", f),
    }
}

pub fn static_import_shim_name(statik: &str) -> String {
    format!("__wbg_field_import_shim_{}", statik)
}

pub type Type = char;

pub const TYPE_VECTOR_JSVALUE: char = '\u{5b}';
// Note: '\u{5c}' is '\' which breaks json encoding/decoding
pub const TYPE_ENUM: char = '\u{5d}';
pub const TYPE_NUMBER: char = '\u{5e}';
pub const TYPE_BORROWED_STR: char = '\u{5f}';
pub const TYPE_STRING: char = '\u{60}';
pub const TYPE_BOOLEAN: char = '\u{61}';
pub const TYPE_SLICE_U8: char = '\u{62}';
pub const TYPE_VECTOR_U8: char = '\u{63}';
pub const TYPE_SLICE_I8: char = '\u{64}';
pub const TYPE_VECTOR_I8: char = '\u{65}';
pub const TYPE_SLICE_U16: char = '\u{66}';
pub const TYPE_VECTOR_U16: char = '\u{67}';
pub const TYPE_SLICE_I16: char = '\u{68}';
pub const TYPE_VECTOR_I16: char = '\u{69}';
pub const TYPE_SLICE_U32: char = '\u{6a}';
pub const TYPE_VECTOR_U32: char = '\u{6b}';
pub const TYPE_SLICE_I32: char = '\u{6c}';
pub const TYPE_VECTOR_I32: char = '\u{6d}';
pub const TYPE_VECTOR_F32: char = '\u{6e}';
pub const TYPE_SLICE_F32: char = '\u{6f}';
pub const TYPE_VECTOR_F64: char = '\u{70}';
pub const TYPE_SLICE_F64: char = '\u{71}';
pub const TYPE_JS_OWNED: char = '\u{72}';
pub const TYPE_JS_REF: char = '\u{73}';

pub const TYPE_CUSTOM_START: u32 = 0x74;
pub const TYPE_CUSTOM_REF_FLAG: u32 = 1;

pub fn name_to_descriptor(name: &str) -> char {
    const CHAR_MAX: u32 = 0x10ffff;
    const CHAR_HOLE_START: u32 = 0xd800;
    const CHAR_HOLE_END: u32 = 0xe000;
    let mut h = fnv::FnvHasher::default();
    name.hash(&mut h);
    let val = h.finish();
    let range = (CHAR_MAX - (CHAR_HOLE_END - CHAR_HOLE_START) - TYPE_CUSTOM_START) / 2;
    let idx = (val % (range as u64)) as u32;
    let mut ret = TYPE_CUSTOM_START + idx * 2;
    if CHAR_HOLE_START <= ret && ret < CHAR_HOLE_END {
        ret += CHAR_HOLE_END - CHAR_HOLE_START;
    }
    char::from_u32(ret).unwrap()
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
