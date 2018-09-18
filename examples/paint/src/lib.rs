extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn main() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    (document.body().unwrap().as_ref() as &web_sys::Node)
        .append_child(canvas.as_ref() as &web_sys::Node)
        .unwrap();
    canvas.set_width(640);
    canvas.set_height(480);
    (canvas.as_ref() as &web_sys::HtmlElement)
        .style()
        .set_property("border", "solid")
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure: Closure<FnMut(_)> = Closure::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        });
        (canvas.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure: Closure<FnMut(_)> = Closure::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        });
        (canvas.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure: Closure<FnMut(_)> = Closure::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        });
        (canvas.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}
