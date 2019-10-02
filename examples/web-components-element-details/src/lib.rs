use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct ElementDetails;

#[wasm_bindgen]
impl ElementDetails {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<ElementDetails> {
        let owned = instantiate! {
            super();
            ElementDetails
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let template = document
            .get_element_by_id("element-details-template").unwrap()
            .unchecked_into::<web_sys::HtmlTemplateElement>()
            .content();
        
        this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap()
          .append_child(&template.clone_node_with_deep(true).unwrap()).unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    web_sys::window().unwrap()
        .custom_elements()
        .define(
            "element-details",
            &js_sys::Function::of::<ElementDetails>(),
        ).unwrap();
}