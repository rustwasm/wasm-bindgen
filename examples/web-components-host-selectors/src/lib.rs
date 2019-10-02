use wasm_bindgen::prelude::*;
#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct ContextSpan;

#[wasm_bindgen]
impl ContextSpan {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<ContextSpan> {
        let owned = instantiate! {
            super();
            ContextSpan
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let style = document.create_element("style").unwrap();
        let span = document.create_element("span").unwrap();
        span.set_text_content(this.text_content().as_ref().map(String::as_str));

        let shadow_root = this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap();
        shadow_root.append_child(&style).unwrap();
        shadow_root.append_child(&span).unwrap();

        style.set_text_content(Some(r#"
            span:hover { text-decoration: underline; }
            :host-context(h1) { font-style: italic; }
            :host-context(h1):after { content: " - no links in headers!" }
            :host-context(article, aside) { color: gray; }
            :host(.footer) { color : red; }
            :host { background: rgba(0,0,0,0.1); padding: 2px 5px; }
        "#));
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    // Define the new element
    web_sys::window().unwrap()
        .custom_elements()
        .define(
            "context-span",
            &js_sys::Function::of::<ContextSpan>(),
        ).unwrap();
}