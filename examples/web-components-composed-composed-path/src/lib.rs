use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct OpenShadowHtmlElement;
#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct ClosedShadowHtmlElement;

#[wasm_bindgen]
impl OpenShadowHtmlElement {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<OpenShadowHtmlElement> {
        let owned = instantiate! {
            super();
            OpenShadowHtmlElement
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let p_elem = document.create_element("p").unwrap();
        p_elem.set_text_content(this.get_attribute("text").as_ref().map(String::as_str));

        this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap().append_child(&p_elem).unwrap();
    }
}

#[wasm_bindgen]
impl ClosedShadowHtmlElement {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<ClosedShadowHtmlElement> {
        let owned = instantiate! {
            super();
            ClosedShadowHtmlElement
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let p_elem = document.create_element("p").unwrap();
        p_elem.set_text_content(this.get_attribute("text").as_ref().map(String::as_str));

        this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Closed)
        ).unwrap().append_child(&p_elem).unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let custom_elements = window.custom_elements();
    custom_elements.define(  "open-shadow", &js_sys::Function::of::<  OpenShadowHtmlElement>()).unwrap();
    custom_elements.define("closed-shadow", &js_sys::Function::of::<ClosedShadowHtmlElement>()).unwrap();

    document
        .query_selector("html")
        .unwrap()
        .unwrap()
        .add_event_listener_with_callback("click", Closure::wrap(Box::new(|e: web_sys::Event| {
            web_sys::console::log_1(&e.composed().into());
            web_sys::console::log_1( e.composed_path().as_ref());
        }) as Box<dyn Fn(_)>).as_ref().unchecked_ref()).unwrap();
}