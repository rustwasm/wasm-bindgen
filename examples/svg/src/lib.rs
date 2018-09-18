extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]

extern "C" {
    static document: web_sys::Document;
}
// Definitions of the functionality available in JS, which wasm-bindgen will
// generate shims for today (and eventually these should be near-0 cost!)
//
// These definitions need to be hand-written today but the current vision is
// that we'll use WebIDL to generate this `extern` block into a crate which you
// can link and import. There's a tracking issue for this at
// https://github.com/rustwasm/wasm-bindgen/issues/42
//
// In the meantime these are written out by hand and correspond to the names and
// signatures documented on MDN, for example

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() {
    let svg = document.create_element_ns("http://www.w3.org/2000/svg", "svg"); 
    svg.set_attribute("width", &JsValue::from("100%"));
    svg.set_attribute("height", &JsValue::from("100%"));
    document.body().append_child(&svg);

    let rect = document.create_element_ns("http://www.w3.org/2000/svg", "rect");
    rect.set_attribute("width", &JsValue::from("100%"));
    rect.set_attribute("fill", &JsValue::from("red"));    
    rect.set_attribute("height", &JsValue::from("100%"));
    svg.append_child(&rect);    

    let circle = document.create_element_ns("http://www.w3.org/2000/svg", "circle");
    circle.set_attribute("stroke", &JsValue::from("black"));
    circle.set_attribute("cx", &JsValue::from("100"));
    circle.set_attribute("fill", &JsValue::from("blue"));    
    circle.set_attribute("cy", &JsValue::from("100"));
    circle.set_attribute("r", &JsValue::from("150"));
    svg.append_child(&circle); 

    let polygon = document.create_element_ns("http://www.w3.org/2000/svg", "polygon");
    polygon.set_attribute("points", &JsValue::from("120,0 240,225 0,225"));
    polygon.set_attribute("fill", &JsValue::from("green"));
    svg.append_child(&polygon);
    
    let text = document.create_element_ns("http://www.w3.org/2000/svg", "text");
    text.set_attribute("stroke", &JsValue::from("black"));
    text.set_attribute("x", &JsValue::from("50"));
    text.set_attribute("fill", &JsValue::from("white"));    
    text.set_attribute("y", &JsValue::from("100"));
    text.set_attribute("font-size", &JsValue::from("55"));
    text.set_attribute("font-family", &JsValue::from("Verdana"));
    text.set_attribute("stroke-width", &JsValue::from("2"));
    text.set_inner_html("Hello!");
    svg.append_child(&text);

}
