use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct SummaryDisplay;

#[wasm_bindgen]
impl SummaryDisplay {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<SummaryDisplay> {
        let owned = instantiate! {
            super();
            SummaryDisplay
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let template = document
            .get_element_by_id("summary-display-template").unwrap()
            .unchecked_into::<web_sys::HtmlTemplateElement>();

        let template_content = template.content();

        let shadow_root = this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap();

        shadow_root.append_child(&template_content.clone_node_with_deep(true).unwrap()).unwrap();

        let items = js_sys::Array::from(&this.query_selector_all("li").unwrap());
        let descriptions = Box::leak(Box::new(js_sys::Array::from(&this.query_selector_all("p").unwrap())));

        fn update_display(description: web_sys::Element, item: &web_sys::HtmlElement) {
            description.remove_attribute("slot").unwrap();

            if description.get_attribute("data-name").unwrap() == item.text_content().unwrap() {
                description.set_attribute("slot", "choice").unwrap();
                item.style().set_property("background-color", "#bad0e4").unwrap();
            }
        };

        let items_ = Box::leak(Box::new(items.clone()));
        let click_handler = Closure::wrap(Box::new(move |e: web_sys::Event| {
            items_.for_each(&mut |item: JsValue, _: u32, _: js_sys::Array|
                item.unchecked_into::<web_sys::HtmlElement>()
                    .style()
                    .set_property("background-color", "white")
                    .unwrap()
            );

            let item = e.target().unwrap().unchecked_into();
            descriptions.for_each(&mut |description: JsValue, _: u32, _: js_sys::Array|
                update_display(description.unchecked_into(), &item)
            );
        }) as Box<dyn Fn(_)>);

        items.for_each(&mut |item: JsValue, _: u32, _: js_sys::Array|
            item.unchecked_into::<web_sys::EventTarget>()
                .add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())
                .unwrap()
        );

        click_handler.forget();

        let slots = this.shadow_root().unwrap().query_selector_all("slot").unwrap();

        let slots_ = Box::leak(Box::new(slots.clone()));
        let slotchange_handler = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let nodes = slots_.item(1).unwrap().unchecked_ref::<web_sys::HtmlSlotElement>().assigned_nodes();
            web_sys::console::log_1(&format!(
                r#"Element in Slot "{}" changed to "{}"."#,
                slots_.item(1).unwrap().unchecked_ref::<web_sys::HtmlSlotElement>().name(),
                nodes.get(0).unchecked_ref::<web_sys::Element>().outer_html(),
            ).into());
        }) as Box<dyn Fn(_)>);

        slots.item(1).unwrap().add_event_listener_with_callback("slotchange", slotchange_handler.as_ref().unchecked_ref()).unwrap();

        slotchange_handler.forget();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    web_sys::window().unwrap()
        .custom_elements()
        .define("summary-display", &js_sys::Function::of::<SummaryDisplay>()).unwrap();
}