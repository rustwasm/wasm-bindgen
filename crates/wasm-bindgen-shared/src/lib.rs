#[macro_use]
extern crate serde_derive;

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
}

impl Type {
    pub fn is_number(&self) -> bool {
        match *self {
            Type::Number => true,
            _ => false,
        }
    }
}
