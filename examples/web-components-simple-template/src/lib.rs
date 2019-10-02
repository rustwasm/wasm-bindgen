use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct MyParagraph;

#[wasm_bindgen]
impl MyParagraph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<MyParagraph> {
        let owned = instantiate! {
            super();
            MyParagraph
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let template = document
            .get_element_by_id("my-paragraph").unwrap()
            .unchecked_into::<web_sys::HtmlTemplateElement>();

        let template_content = template.content();

        this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap()
          .append_child(&template_content.clone_node_with_deep(true).unwrap()).unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    window.custom_elements().define(
        "my-paragraph",
        &js_sys::Function::of::<MyParagraph>(),
    ).unwrap();

    let slotted_span = document.query_selector("my-paragraph span").unwrap().unwrap();

    web_sys::console::log_1(&slotted_span.assigned_slot().unwrap());
    web_sys::console::log_1(&slotted_span.slot().into());
}
