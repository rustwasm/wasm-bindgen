#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub structs: Vec<Struct>,
    pub free_functions: Vec<Function>,
    pub imports: Vec<Import>,
    pub imported_structs: Vec<ImportStruct>,
    pub custom_type_names: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub functions: Vec<Function>,
    pub methods: Vec<Method>,
}

#[derive(Serialize, Deserialize)]
pub struct Import {
    pub module: String,
    pub function: Function,
}

#[derive(Serialize, Deserialize)]
pub struct ImportStruct {
    pub module: Option<String>,
    pub name: String,
    pub functions: Vec<ImportStructFunction>,
}

#[derive(Serialize, Deserialize)]
pub struct ImportStructFunction {
    pub method: bool,
    pub function: Function,
}

#[derive(Serialize, Deserialize)]
pub struct Method {
    pub mutable: bool,
    pub function: Function,
}

#[derive(Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Type>,
    pub ret: Option<Type>,
}

pub fn free_function(struct_name: &str) -> String {
    let mut name = format!("__wbindgen_");
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
