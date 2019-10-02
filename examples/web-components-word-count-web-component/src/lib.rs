use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Create a class for the element
#[wasm_bindgen(prototype=web_sys::HtmlParagraphElement)]
pub struct WordCount;

#[wasm_bindgen]
impl WordCount {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<WordCount> {
        let owned = instantiate! {
            // Always call super first in constructor
            super();
            WordCount
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        // count words in element's parent element
        let wc_parent = Box::new(this.parent_node().unwrap());

        fn count_words(node: &web_sys::Node) -> usize {
            let text = node.text_content().unwrap();
            text.split_whitespace().count()
        }

        let count = format!("Words: {}", count_words(&*wc_parent));

        // Create a shadow root
        let shadow = this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap();

        // Create text node and add word count to it
        let text = Box::new(document.create_element("span").unwrap());
        text.set_text_content(Some(&count));

        // Append it to the shadow root
        shadow.append_child(&*text).unwrap();

        let wc_parent = Box::leak(wc_parent);
        let text = Box::leak(text);

        let callback = Closure::wrap(Box::new(move || {
            let count = format!("Words: {}", count_words(wc_parent));
            text.set_text_content(Some(&count));
        }) as Box<dyn Fn()>);

        // Update count when element content changes
        window.set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            200,
        ).unwrap();
        
        callback.forget();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let mut options = web_sys::ElementDefinitionOptions::new();
    options.extends("p");

    web_sys::window().unwrap()
        .custom_elements()
        .define_with_options(
            "word-count",
            &js_sys::Function::of::<WordCount>(),
            &options
        ).unwrap();
}