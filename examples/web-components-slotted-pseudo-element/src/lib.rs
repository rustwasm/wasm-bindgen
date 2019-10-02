use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct PersonDetails;

#[wasm_bindgen]
impl PersonDetails {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<PersonDetails> {
        let owned = instantiate! {
            super();
            PersonDetails
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        let template = document
            .get_element_by_id("person-template").unwrap()
            .unchecked_into::<web_sys::HtmlTemplateElement>();
        let template_content = template.content();

        let shadow_root = this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap();

        let style = document.create_element("style").unwrap();
        style.set_text_content(Some("
            div { padding: 10px; border: 1px solid gray; width: 200px; margin: 10px; }
            h2 { margin: 0 0 10px; }
            ul { margin: 0; }
            p { margin: 10px 0; }
            ::slotted(*) { color: gray; font-family: sans-serif; }
        "));

        shadow_root.append_child(&style).unwrap();
        shadow_root.append_child(&template_content.clone_node_with_deep(true).unwrap()).unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();

    let custom_elements = window.custom_elements();
    custom_elements.define("person-details", &js_sys::Function::of::<PersonDetails>()).unwrap();
}