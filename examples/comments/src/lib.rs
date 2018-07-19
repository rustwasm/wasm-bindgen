#![feature(use_extern_macros, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// A user defined comment
pub struct Comment {
    name: String,
    comment: String,
    /// The position this comment
    /// should exist at
    pub count: u32,
    color: Color,
}

#[wasm_bindgen]
impl Comment {
    #[wasm_bindgen(method)]
    /// The name of the user who
    /// posted this comment
    pub fn name(&self) -> String {
        self.name.clone()
    }
    #[wasm_bindgen(method)]
    /// The content of this comment
    pub fn comment(&self) -> String {
        self.comment.clone()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(name: String, comment: String, count: u32) -> Comment {
        let color = if count % 2 == 0 {
            Color::Blue
        } else {
            Color::Pink
        };
        Comment {
            name,
            comment,
            count,
            color,
        }
    }
    #[wasm_bindgen(method)]
    /// What color should this comment be
    pub fn color(&self) -> Color {
        self.color.clone()
    }
}

/// The border of a comment
#[wasm_bindgen]
#[derive(Clone)]
pub enum Color {
    Blue,
    Pink,
}
