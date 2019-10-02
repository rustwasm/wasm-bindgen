use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct PersonDetailsHtmlElement;
#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct EditWordHtmlElement;

#[wasm_bindgen]
impl PersonDetailsHtmlElement {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<PersonDetailsHtmlElement> {
        let owned = instantiate! {
            super();
            PersonDetailsHtmlElement
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let template = document.get_element_by_id("person-template").unwrap().unchecked_into::<web_sys::HtmlTemplateElement>();
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
        "));

        shadow_root.append_child(&style).unwrap();
        shadow_root.append_child(&template_content.clone_node_with_deep(true).unwrap()).unwrap();
    }
}

#[wasm_bindgen]
impl EditWordHtmlElement {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<EditWordHtmlElement> {
        let owned = instantiate! {
            super();
            EditWordHtmlElement {}
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let shadow_root = this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap();

        let form = document.create_element("form").unwrap().unchecked_into::<web_sys::HtmlElement>();
        let input = document.create_element("input").unwrap().unchecked_into::<web_sys::HtmlInputElement>();
        let span = document.create_element("span").unwrap().unchecked_into::<web_sys::HtmlElement>();

        let style = document.create_element("style").unwrap();
        style.set_text_content(Some("span { background-color: #eef; padding: 0 2px }"));

        shadow_root.append_child(&style).unwrap();
        shadow_root.append_child(&form).unwrap();
        shadow_root.append_child(&span).unwrap();

        let text_content = this.text_content();
        let text_content = text_content.as_ref().map(String::as_str);
        span.set_text_content(text_content);
        input.set_value(text_content.unwrap());

        form.append_child(&input).unwrap();
        form.style().set_property("display", "none").unwrap();
        span.style().set_property("display", "inline-block").unwrap();
        input.style().set_property("width", &format!("{}px", span.client_width())).unwrap();

        this.set_attribute("tabindex", "0").unwrap();
        input.set_attribute("required", "required").unwrap();
        this.style().set_property("display", "inline-block").unwrap();

        let span_ = span.clone();
        let form_ = form.clone();
        let input_ = input.clone();
        let click_handler = Closure::wrap(Box::new(move || {
            use std::convert::TryInto;
            span_.style().set_property("display", "none").unwrap();
            form_.style().set_property("display", "inline-block").unwrap();
            input_.focus().unwrap();
            input_.set_selection_range(0, input_.value().len().try_into().unwrap()).unwrap();
        }) as Box<dyn Fn()>);
        this.add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref()).unwrap();
        click_handler.forget();

        let span_ = span.clone();
        let form_ = form.clone();
        let input_ = input.clone();
        let update_display = move || {
            span_.style().set_property("display", "inline-block").unwrap();
            form_.style().set_property("display", "none").unwrap();
            span_.set_text_content(Some(&input_.value()));
            input_.style().set_property("width", &format!("{}px", span_.client_width())).unwrap();
        };

        let update_display_ = update_display.clone();
        form.add_event_listener_with_callback("submit", &Closure::once_into_js(move |e: web_sys::Event| {
            update_display_();
            e.prevent_default();
        }).into()).unwrap();

        let update_display = Closure::wrap(Box::new(update_display) as Box<dyn Fn()>);
        input.add_event_listener_with_callback(
            "blur",
            update_display.as_ref().unchecked_ref(),
        ).unwrap();

        update_display.forget();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();

    let custom_elements = window.custom_elements();
    custom_elements.define("person-details", &js_sys::Function::of::<PersonDetailsHtmlElement>()).unwrap();
    custom_elements.define(  "edit-word"   , &js_sys::Function::of::<     EditWordHtmlElement>()).unwrap();
}