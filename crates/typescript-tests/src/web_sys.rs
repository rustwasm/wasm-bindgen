use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct DocStruct {
    doc: Document,
}

#[wasm_bindgen]
impl DocStruct {
    pub fn new(doc: Document) -> Self {
        Self { doc }
    }

    pub fn get_doc(&self) -> Document {
        self.doc.clone()
    }

    pub fn append_element(&self, element: &HtmlElement) {
        self.doc.body().unwrap().append_child(element).unwrap();
    }

    pub fn append_many(&self, _: &HtmlElement, _: &HtmlElement, _: &HtmlElement) {}
}
