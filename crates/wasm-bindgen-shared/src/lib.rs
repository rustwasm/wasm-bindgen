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
    pub ctor: Function,
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
