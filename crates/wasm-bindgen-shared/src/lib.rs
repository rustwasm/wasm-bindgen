#[macro_use]
extern crate serde_derive;
extern crate fnv;

use std::char;
use std::hash::{Hash, Hasher};

#[derive(Deserialize)]
pub struct Program {
    pub exports: Vec<Export>,
    pub imports: Vec<Import>,
    pub custom_type_names: Vec<CustomTypeName>,
}

#[derive(Deserialize)]
pub struct Import {
    pub module: Option<String>,
    pub catch: bool,
    pub method: bool,
    pub js_new: bool,
    pub statik: bool,
    pub class: Option<String>,
    pub function: Function,
}

#[derive(Deserialize)]
pub struct Export {
    pub class: Option<String>,
    pub method: bool,
    pub function: Function,
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

pub type Type = char;

pub const TYPE_NUMBER: char = '\u{5e}';
pub const TYPE_BORROWED_STR: char = '\u{5f}';
pub const TYPE_STRING: char = '\u{60}';
pub const TYPE_BOOLEAN: char = '\u{61}';
pub const TYPE_JS_OWNED: char = '\u{62}';
pub const TYPE_JS_REF: char = '\u{63}';

pub const TYPE_CUSTOM_START: u32 = 0x64;
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
