#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub structs: Vec<Struct>,
    pub free_functions: Vec<Function>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub functions: Vec<Function>,
    pub methods: Vec<Method>,
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

impl Struct {
    pub fn free_function(&self) -> String {
        let mut name = format!("__wbindgen_");
        name.extend(self.name
            .chars()
            .flat_map(|s| s.to_lowercase()));
        name.push_str("_free");
        return name
    }
}

impl Function {
    pub fn free_function_export_name(&self) -> String {
        self.name.clone()
    }

    pub fn struct_function_export_name(&self, struct_: &str) -> String {
        let mut name = struct_
            .chars()
            .flat_map(|s| s.to_lowercase())
            .collect::<String>();
        name.push_str("_");
        name.push_str(&self.name);
        return name
    }
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    Number,
    BorrowedStr,
    String,
    ByValue(String),
    ByRef(String),
    ByMutRef(String),
}

impl Type {
    pub fn is_number(&self) -> bool {
        match *self {
            Type::Number => true,
            _ => false,
        }
    }
}
