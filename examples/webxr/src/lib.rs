use wasm_bindgen::prelude::*;
use futures::{future, Future};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use web_sys::{XrFrame, XrSession, XrSessionMode};
use std::cell::RefCell;
use std::rc::Rc;


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn request_animation_frame(session: &XrSession, f: &Closure<dyn FnMut(f64, XrFrame)>) {
    session
        .request_animation_frame(f.as_ref().unchecked_ref());
}

#[wasm_bindgen]
pub async fn run_webxr() {
    log!("Starting WebXR...");
    let navigator: web_sys::Navigator = web_sys::window().unwrap().navigator();
    let xr = navigator.xr();
    let session_mode = XrSessionMode::Inline;
    let session_supported_promise = xr.is_session_supported(session_mode);
    let supports_session = wasm_bindgen_futures::JsFuture::from(session_supported_promise).await;
    let supports_session = supports_session.unwrap();
    if supports_session == false {
        log!("XR session not supported");
        return()
    }
    let session_promise = xr.request_session(session_mode);
    let session = wasm_bindgen_futures::JsFuture::from(session_promise).await;
    let session: XrSession = session.unwrap().into();

    // Reuse the logic from the requestAnimationFrame() example
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64, frame: XrFrame| {
        if i > 300 {
            log!("All done!");

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        i += 1;
        let text = format!("requestAnimationFrame has been called {} times.", i);
        log!("{}", text);

        // Schedule ourself for another requestAnimationFrame callback.
        let inner_session: XrSession = frame.session();
        request_animation_frame(&inner_session, f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64, XrFrame)>));

    log!("Starting animation frame...");
    request_animation_frame(&session, g.borrow().as_ref().unwrap());

    ()
}
