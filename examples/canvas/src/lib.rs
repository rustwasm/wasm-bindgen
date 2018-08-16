#![feature(use_extern_macros)]

extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    static document: web_sys::Document;
}

#[wasm_bindgen]
pub fn draw() {
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context_using_context_id("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2D>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context.arc_using_x_and_y_and_radius_and_start_angle_and_end_angle(
        75.0,
        75.0,
        50.0,
        0.0,
        f64::consts::PI * 2.0,
    );

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc_using_x_and_y_and_radius_and_start_angle_and_end_angle(
        75.0,
        75.0,
        35.0,
        0.0,
        f64::consts::PI,
    );

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context.arc_using_x_and_y_and_radius_and_start_angle_and_end_angle(
        60.0,
        65.0,
        5.0,
        0.0,
        f64::consts::PI * 2.0,
    );

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context.arc_using_x_and_y_and_radius_and_start_angle_and_end_angle(
        90.0,
        65.0,
        5.0,
        0.0,
        f64::consts::PI * 2.0,
    );

    context.stroke();
}
