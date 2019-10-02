use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct EditableList {
    item_list: Option<web_sys::Element>,
}

#[wasm_bindgen]
impl EditableList {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<EditableList> {
        let owned = instantiate! {
            // establish prototype chain
            super();
            EditableList { item_list: None }
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // attaches shadow tree and returns shadow root reference
        // https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow
        let shadow = this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap();

        // creating a container for the editable-list component
        let editable_list_container = document.create_element("div").unwrap();

        // get attribute values from getters
        let title = this.title();
        let add_item_text = this.add_item_text();
        let list_items: Vec<String> = this.items();

        // adding a class toour container for the sake of clarity
        editable_list_container.class_list().add(&js_sys::Array::of1(&"editable-list".into())).unwrap();

        // creating the inner HTML of the editable list element
        editable_list_container.set_inner_html(&format!(r#"
            <style>
              li, div > div {{
                display: flex;
                align-items: center;
                justify-content: space-between;
              }}

              .icon {{
                background-color: #fff;
                border: none;
                cursor: pointer;
                float: right;
                font-size: 1.8rem;
              }}
            </style>
            <h3>{}</h3>
            <ul class="item-list">
              {}
            </ul>
            <div>
              <label>{}</label>
              <input class="add-new-list-item-input" type="text"></input>
              <button class="editable-list-add-item icon">&oplus;</button>
            </div>
        "#, title, list_items.iter().map(|item: &String| format!(r#"
                <li>{}
                  <button class="editable-list-remove-item icon">&ominus;</button>
                </li>
        "#, item)).collect::<Vec<_>>().join(""), add_item_text));

        // binding methods
        // this.addListItem = this.addListItem.bind(this);
        // this.handleRemoveItemListeners = this.handleRemoveItemListeners.bind(this);
        // this.removeListItem = this.removeListItem.bind(this);

        // appending the container to the shadow DOM
        shadow.append_child(&editable_list_container).unwrap();
    }

    // add items to the list
    // #[wasm_bindgen(js_name="addListItem")]
    fn add_list_item(&self, _: web_sys::Event) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_input = self
            .shadow_root().unwrap()
            .query_selector(".add-new-list-item-input").unwrap().unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>();

        if !text_input.value().is_empty() {
            let li = document.create_element("li").unwrap();
            let button = document.create_element("button").unwrap();
            let children_length = self.item_list.as_ref().unwrap().children().length();

            li.set_text_content(Some(text_input.value().as_str()));
            button.class_list().add(&js_sys::Array::of2(&"editable-list-remove-item".into(), &"icon".into())).unwrap();
            button.set_inner_html("&ominus;");

            self.item_list.as_ref().unwrap().append_child(&li).unwrap();
            self.item_list.as_ref().unwrap().children().item(children_length).unwrap().append_child(&button).unwrap();
            li.append_child(&button).unwrap();

            Self::handle_remove_item_listeners(js_sys::Array::of1(&button));

            text_input.set_value("");
        }
    }

    // fires after the element has been attached to the DOM
    #[wasm_bindgen(js_name="connectedCallback")]
    pub fn connected_callback(&mut self) {
        let remove_element_buttons = self.shadow_root().unwrap().query_selector_all(".editable-list-remove-item").unwrap();
        let add_element_button = self.shadow_root().unwrap().query_selector(".editable-list-add-item").unwrap().unwrap();

        self.item_list = self.shadow_root().unwrap().query_selector(".item-list").unwrap();

        let this = self as *mut Self;
        let add_list_item = Closure::wrap(Box::new(
            move |e| (unsafe { &mut *this }).add_list_item(e)
        ) as Box<dyn Fn(_)>);

        Self::handle_remove_item_listeners(remove_element_buttons.unchecked_into::<js_sys::Array>());
        add_element_button.add_event_listener_with_callback_and_bool(
            "click",
            add_list_item.as_ref().unchecked_ref(),
            false,
        ).unwrap();

        add_list_item.forget();
    }

    // gathering data from element attributes
    // #[wasm_bindgen(getter)]
    fn title(&self) -> String {
        self.get_attribute("title").unwrap_or("".to_string())
    }

    // #[wasm_bindgen(getter)]
    fn items(&self) -> Vec<String> {
        let mut items = Vec::new();

        let attributes = self.attributes();
        for i in 0..attributes.length() {
            let attr = attributes.item(i).unwrap();
            if attr.name().contains("list-item") {
                items.push(attr.value());
            }
        }

        items
    }

    // #[wasm_bindgen(getter)]
    fn add_item_text(&self) -> String {
        self.get_attribute("add-item-text").unwrap_or("".to_string())
    }

    fn handle_remove_item_listeners(array_of_elements: js_sys::Array) {
        let remove_list_item = Closure::wrap(Box::new(
            Self::remove_list_item
        ) as Box<dyn Fn(_)>);

        array_of_elements.for_each(&mut |element: JsValue, _: u32, _: js_sys::Array| {
            element
                .unchecked_into::<web_sys::EventTarget>()
                .add_event_listener_with_callback_and_bool(
                    "click",
                    remove_list_item.as_ref().unchecked_ref(),
                    false,
                ).unwrap();
        });

        remove_list_item.forget();
    }

    fn remove_list_item(event: web_sys::Event) {
        event
            .target().unwrap()
            .unchecked_into::<web_sys::Node>()
            .parent_node().unwrap()
            .unchecked_into::<web_sys::Element>()
            .remove();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    // let the browser know about the custom element
    let window = web_sys::window().unwrap();

    let custom_elements = window.custom_elements();
    custom_elements.define("editable-list", &js_sys::Function::of::<EditableList>()).unwrap();
}
