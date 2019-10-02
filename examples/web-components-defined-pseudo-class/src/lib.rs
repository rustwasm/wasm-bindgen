use wasm_bindgen::prelude::*;
#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct SimpleCustomHtmlElement;

#[wasm_bindgen]
impl SimpleCustomHtmlElement {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<SimpleCustomHtmlElement> {
        let owned = instantiate! {
            super();
            SimpleCustomHtmlElement
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let div_elem = document.create_element("div").unwrap();
        div_elem.set_text_content(this.get_attribute("text").as_ref().map(String::as_str));

        let shadow_root = this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap();
        shadow_root.append_child(&div_elem).unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    web_sys::window().unwrap()
      .custom_elements()
      .define(
          "simple-custom",
          &js_sys::Function::of::<SimpleCustomHtmlElement>()
      ).unwrap();
}