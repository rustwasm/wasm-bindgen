use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Create a class for the element
#[wasm_bindgen(prototype=web_sys::HtmlElement)]
pub struct Square;

#[wasm_bindgen]
impl Square {
    // Specify observed attributes so that
    // attributeChangedCallback will work
    #[wasm_bindgen(getter = observedAttributes)]
    pub fn observed_attributes() -> js_sys::Array {
        js_sys::Array::of2(&"c".into(), &"l".into())
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<Square> {
        let owned = instantiate! {
            // Always call super first in constructor
            super();
            Square
        };
        let this = owned.borrow();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let shadow = this.attach_shadow(
            &web_sys::ShadowRootInit::new(web_sys::ShadowRootMode::Open)
        ).unwrap();

        let div = document.create_element("div").unwrap();
        let style = document.create_element("style").unwrap();
        shadow.append_child(&style).unwrap();
        shadow.append_child(&div).unwrap();
    }

    #[wasm_bindgen(js_name="connectedCallback")]
    pub fn connected_callback(&self) {
        web_sys::console::log_1(&"Custom square element added to page.".into());
        self.update_style();
    }

    #[wasm_bindgen(js_name="disconnectedCallback")]
    pub fn disconnected_callback(&self) {
        web_sys::console::log_1(&"Custom square element removed from page.".into());
    }

    #[wasm_bindgen(js_name="adoptedCallback")]
    pub fn adopted_callback(&self) {
        web_sys::console::log_1(&"Custom square element moved to new page.".into());
    }

    #[wasm_bindgen(js_name="attributeChangedCallback")]
    pub fn attribute_changed_callback(&self, _name: &str, _old_value: Option<String>, _new_value: &str) {
        web_sys::console::log_1(&"Custom square element attributes changed.".into());
        self.update_style();
    }

    fn update_style(&self) {
      let shadow = self.shadow_root().unwrap();
      shadow.query_selector("style").unwrap().unwrap().set_text_content(Some(&format!(
          "
          div {{
              width: {0}px;
              height: {0}px;
              background-color: {1};
          }}
          ",
          self.get_attribute("l").unwrap_or(String::from("100")),
          self.get_attribute("c").unwrap_or(String::from("red")),
      )));
    }
}

fn random(min: u32, max: u32) -> u32 {
    js_sys::Math::floor(js_sys::Math::random() * ((max - min + 1) + min) as f64) as u32
}

struct Context {
    add: web_sys::HtmlButtonElement,
    update: web_sys::HtmlButtonElement,
    remove: web_sys::HtmlButtonElement,
    square: Option<web_sys::Element>,
}

impl Context {
    fn new() -> Context {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let mut me = Context {
            add   : document.query_selector(".add"   ).unwrap().unwrap().unchecked_into(),
            update: document.query_selector(".update").unwrap().unwrap().unchecked_into(),
            remove: document.query_selector(".remove").unwrap().unwrap().unchecked_into(),
            square: None,
        };
        me.update_buttons();

        me
    }

    fn update_buttons(&mut self) {
        let square_exists = self.square.is_some();
        self.add.set_disabled(square_exists);
        self.update.set_disabled(!square_exists);
        self.remove.set_disabled(!square_exists);
    }

    // Create a custom square element
    fn add(&mut self) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        self.square = document.create_element("custom-square").ok();
        let square = self.square.as_ref().unwrap();
        square.set_attribute("l", "100").unwrap();
        square.set_attribute("c", "red").unwrap();
        document.body().unwrap().append_child(square).unwrap();

        self.update_buttons();
    }

    // Randomly update square's attributes
    fn update(&mut self) {
        let square = self.square.as_ref().unwrap();
        square.set_attribute("l", &random(50, 200).to_string()).unwrap();
        square.set_attribute("c", &format!("rgb({}, {}, {})", random(0, 255), random(0, 255), random(0, 255))).unwrap();
    }

    // Remove the square
    fn remove(&mut self) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let square = self.square.take().unwrap();
        document.body().unwrap().remove_child(&square).unwrap();
        self.update_buttons();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    web_sys::window().unwrap()
        .custom_elements()
        .define("custom-square", &js_sys::Function::of::<Square>()).unwrap();

    let context = Box::into_raw(Box::new(Context::new()));

    let add_handler    = Closure::wrap(Box::new(move || unsafe { (*context).add();    }) as Box<dyn Fn()>);
    let update_handler = Closure::wrap(Box::new(move || unsafe { (*context).update(); }) as Box<dyn Fn()>);
    let remove_handler = Closure::wrap(Box::new(move || unsafe { (*context).remove(); }) as Box<dyn Fn()>);

    let context = unsafe { &mut *context };

    context.add   .set_onclick(Some(add_handler   .as_ref().unchecked_ref()));
    context.update.set_onclick(Some(update_handler.as_ref().unchecked_ref()));
    context.remove.set_onclick(Some(remove_handler.as_ref().unchecked_ref()));

    add_handler   .forget();
    update_handler.forget();
    remove_handler.forget();
}
