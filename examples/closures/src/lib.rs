#![feature(use_extern_macros, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::js::Date;
use wasm_bindgen::js::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Binding for the `setInverval` method in JS. This function takes a "long
    // lived" closure as the first argument so we use `Closure` instead of
    // a bare `&Fn()` which only surives for that one stack frame.
    //
    // The second argument is then the interval and the return value is how we
    // clear this interval. We're not going to clear our interval in this
    // example though so the return value is ignored.
    #[wasm_bindgen(js_name = setInterval)]
    fn set_interval(cb: &Closure<FnMut()>, delay: u32) -> f64;

    // Bindings for `document` and various methods of updating HTML elements.
    // Like with the `dom` example these'll ideally be upstream in a generated
    // crate one day but for now we manually define them.
    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method, js_name = getElementById)]
    fn get_element_by_id(this: &HTMLDocument, id: &str) -> Element;
    #[wasm_bindgen(method, js_name = getElementById)]
    fn get_html_element_by_id(this: &HTMLDocument, id: &str) -> HTMLElement;

    type Element;
    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);

    type HTMLElement;
    #[wasm_bindgen(method, setter)]
    fn set_onclick(this: &HTMLElement, cb: &Closure<FnMut()>);
    #[wasm_bindgen(method, getter)]
    fn style(this: &HTMLElement) -> CSS2Properties;

    type CSS2Properties;
    #[wasm_bindgen(method, setter)]
    fn set_display(this: &CSS2Properties, display: &str);
}

#[wasm_bindgen]
pub fn run() {
    // Set up a clock on our page and update it each second to ensure it's got
    // an accurate date.
    let a = Closure::new(update_time);
    set_interval(&a, 1000);
    update_time();
    fn update_time() {
        document
            .get_element_by_id("current-time")
            .set_inner_html(&String::from(
                Date::new().to_locale_string(JsString::from("en-GB"), JsValue::undefined()),
            ));
    }

    // We also want to count the number of times that our green square has been
    // clicked. Our callback will update the `#num-clicks` div
    let square = document.get_html_element_by_id("green-square");
    let mut clicks = 0;
    let b = Closure::new(move || {
        clicks += 1;
        document
            .get_element_by_id("num-clicks")
            .set_inner_html(&clicks.to_string());
    });
    square.set_onclick(&b);

    // The instances of `Closure` that we created will invalidate their
    // corresponding JS callback whenever they're dropped, so if we were to
    // normally return from `run` then both of our registered closures will
    // raise exceptions when invoked.
    //
    // Normally we'd store these handles to later get dropped at an appropriate
    // time but for now we want these to be global handlers so we use the
    // `forget` method to drop them without invalidating the closure. Note that
    // this is leaking memory in Rust, so this should be done judiciously!
    a.forget();
    b.forget();

    // And finally now that our demo is ready to go let's switch things up so
    // everything is displayed and our loading prompt is hidden.
    document
        .get_html_element_by_id("loading")
        .style()
        .set_display("none");
    document
        .get_html_element_by_id("script")
        .style()
        .set_display("block");
}
