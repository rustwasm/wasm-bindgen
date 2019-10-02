use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Create a class for the element
#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct PopUpInfoExt;

#[wasm_bindgen]
impl PopUpInfoExt {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<PopUpInfoExt> {
        let owned = instantiate! {
            // Always call super first in constructor
            super();
            PopUpInfoExt
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        // Create a shadow root
        let shadow = this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap();

        // Create spans
        let wrapper = document.create_element("span").unwrap();
        wrapper.set_attribute("class", "wrapper").unwrap();

        let icon = document.create_element("span").unwrap();
        icon.set_attribute("class", "icon").unwrap();
        icon.set_attribute("tabindex", "0").unwrap();

        let info = document.create_element("span").unwrap();
        info.set_attribute("class", "info").unwrap();

        // Take attribute content and put it inside the info span
        let text = this.get_attribute("data-text").unwrap();
        info.set_text_content(Some(&text));

        // Insert icon
        let img_url = if this.has_attribute("img") {
            this.get_attribute("img").unwrap()
        } else {
            "img/default.png".to_string()
        };

        let img = document.create_element("img").unwrap().unchecked_into::<web_sys::HtmlImageElement>();
        img.set_src(&img_url);
        icon.append_child(&img).unwrap();

        // Apply external styles to the shadow dom
        let link_elem = document.create_element("link").unwrap();
        link_elem.set_attribute("rel", "stylesheet").unwrap();
        link_elem.set_attribute("href", "style.css").unwrap();

        // Attach the created elements to the shadow dom
        shadow.append_child(&link_elem).unwrap();
        shadow.append_child(&wrapper).unwrap();
        wrapper.append_child(&icon).unwrap();
        wrapper.append_child(&info).unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    // Define the new element
    web_sys::window().unwrap()
        .custom_elements()
        .define(
            "popup-info",
            &js_sys::Function::of::<PopUpInfoExt>(),
        ).unwrap();
}