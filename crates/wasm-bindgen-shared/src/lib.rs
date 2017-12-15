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
}
