extern crate proc_macro2;
extern crate quote;
extern crate wasm_bindgen_typescript;

use wasm_bindgen_typescript::parser;

use proc_macro2::TokenStream;
use quote::ToTokens;

fn main() {
    let program = parser::ts_to_program("dist/wasm.api.json");

    let mut tokens = TokenStream::new();
    program.to_tokens(&mut tokens);
    println!("{:#?}", tokens);
}
