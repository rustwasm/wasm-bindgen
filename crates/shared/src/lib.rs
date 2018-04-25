#[macro_use]
extern crate serde_derive;

pub const SCHEMA_VERSION: &str = "4";

#[derive(Deserialize)]
pub struct ProgramOnlySchema {
    pub schema_version: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
pub struct Program {
    pub exports: Vec<Export>,
    pub enums: Vec<Enum>,
    pub imports: Vec<Import>,
    pub structs: Vec<Struct>,
    pub version: String,
    pub schema_version: String,
}

#[derive(Deserialize, Serialize)]
pub struct Import {
    pub module: Option<String>,
    pub version: Option<String>,
    pub js_namespace: Option<String>,
    pub kind: ImportKind,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ImportKind {
    Function(ImportFunction),
    Static(ImportStatic),
    Type(ImportType),
}

#[derive(Deserialize, Serialize)]
pub struct ImportFunction {
    pub shim: String,
    pub catch: bool,
    pub method: bool,
    pub js_new: bool,
    pub structural: bool,
    pub getter: Option<String>,
    pub setter: Option<String>,
    pub class: Option<String>,
    pub function: Function,
}

#[derive(Deserialize, Serialize)]
pub struct ImportStatic {
    pub name: String,
    pub shim: String,
}

#[derive(Deserialize, Serialize)]
pub struct ImportType {
}

#[derive(Deserialize, Serialize)]
pub struct Export {
    pub class: Option<String>,
    pub method: bool,
    pub constructor: Option<String>,
    pub function: Function,
}

#[derive(Deserialize, Serialize)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Deserialize, Serialize)]
pub struct EnumVariant {
    pub name: String,
    pub value: u32
}

#[derive(Deserialize, Serialize)]
pub struct Function {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Deserialize, Serialize)]
pub struct StructField {
    pub name: String,
    pub readonly: bool,
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

pub fn struct_field_get(struct_: &str, f: &str) -> String {
    let mut name = String::from("__wbg_get_");
    name.extend(struct_
        .chars()
        .flat_map(|s| s.to_lowercase()));
    name.push_str("_");
    name.push_str(f);
    return name
}

pub fn struct_field_set(struct_: &str, f: &str) -> String {
    let mut name = String::from("__wbg_set_");
    name.extend(struct_
        .chars()
        .flat_map(|s| s.to_lowercase()));
    name.push_str("_");
    name.push_str(f);
    return name
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
