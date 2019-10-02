use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(prototype=web_sys::HtmlUListElement)]
pub struct ExpandingList;

#[wasm_bindgen]
impl ExpandingList {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<ExpandingList> {
        // web_sys::window().unwrap().set_onload(Some(&Closure::once_into_js(|| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let uls = document.query_selector_all(":root ul").unwrap();
            let lis = document.query_selector_all(":root li").unwrap();

            for i in 1..uls.length() {
                uls .item(i).unwrap()
                    .unchecked_into::<web_sys::HtmlElement>()
                    .style()
                    .set_property("display", "none").unwrap();
            }

            for i in 0..lis.length() {
                let li = lis.item(i).unwrap();

                let child_text = li.child_nodes().item(0).unwrap();
                let new_span = document.create_element("span").unwrap();

                new_span.set_text_content(child_text.text_content().as_ref().map(String::as_str));
                li.insert_before(&new_span, Some(&child_text)).unwrap();
                li.remove_child(&child_text).unwrap();
            }

            let showul = Closure::wrap(Box::new(|e: web_sys::Event| {
                let nextul = e.target().unwrap()
                    .unchecked_into::<web_sys::HtmlElement>()
                    .next_element_sibling().unwrap()
                    .unchecked_into::<web_sys::HtmlElement>();

                let parent = nextul.parent_node().unwrap()
                    .unchecked_into::<web_sys::Element>();

                let style = nextul.style();

                if style.get_property_value("display").unwrap() == "block" {
                    style.set_property("display", "none").unwrap();
                    parent.set_attribute("class", "closed").unwrap();
                } else {
                    style.set_property("display", "block").unwrap();
                    parent.set_attribute("class", "open").unwrap();
                }
            }) as Box<dyn Fn(_)>);

            let spans = document.query_selector_all(":root span").unwrap();

            for i in 0..spans.length() {
                let span = spans.item(i).unwrap().unchecked_into::<web_sys::HtmlElement>();
                if let Some(_) = span.next_element_sibling() {
                    span.style().set_property("cursor", "pointer").unwrap();
                    span.parent_node().unwrap()
                        .unchecked_into::<web_sys::Element>()
                        .set_attribute("class", "closed").unwrap();
                    span.set_onclick(Some(showul.as_ref().unchecked_ref()));
                }
            }

            showul.forget();
        // }).into()));

        instantiate! {
            super();
            ExpandingList
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let mut options = web_sys::ElementDefinitionOptions::new();
    options.extends("ul");

    web_sys::window().unwrap()
        .custom_elements()
        .define_with_options(
            "expanding-list",
            &js_sys::Function::of::<ExpandingList>(),
            &options
        ).unwrap();
}