#[macro_use] mod utils;

use futures::{future, Future};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
// use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, XrFrame, XrRenderStateInit, XrSession, XrSessionMode, XrWebGlLayer};
use web_sys::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use utils::set_panic_hook;


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// #[wasm_bindgen]
// pub async fn run_webxr() {
//     log!("Starting WebXR...");
//     let navigator: web_sys::Navigator = web_sys::window().unwrap().navigator();
//     let xr = navigator.xr();
//     let session_mode = XrSessionMode::Inline;
//     let session_supported_promise = xr.is_session_supported(session_mode);
//     let supports_session = wasm_bindgen_futures::JsFuture::from(session_supported_promise).await;
//     let supports_session = supports_session.unwrap();
//     if supports_session == false {
//         log!("XR session not supported");
//         return()
//     }
//     let session_promise = xr.request_session(session_mode);
//     let session = wasm_bindgen_futures::JsFuture::from(session_promise).await;
//     let session: XrSession = session.unwrap().into();

//     // Reuse the logic from the requestAnimationFrame() example
//     let f = Rc::new(RefCell::new(None));
//     let g = f.clone();

//     let mut i = 0;
//     *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64, frame: XrFrame| {
//         if i > 300 {
//             log!("All done!");

//             // Drop our handle to this closure so that it will get cleaned
//             // up once we return.
//             let _ = f.borrow_mut().take();
//             return;
//         }

//         i += 1;
//         let text = format!("requestAnimationFrame has been called {} times.", i);
//         log!("{}", text);

//         // Schedule ourself for another requestAnimationFrame callback.
//         let inner_session: XrSession = frame.session();
//         request_animation_frame(&inner_session, f.borrow().as_ref().unwrap());
//     }) as Box<dyn FnMut(f64, XrFrame)>));

//     log!("Starting animation frame...");
//     request_animation_frame(&session, g.borrow().as_ref().unwrap());

//     ()
// }


fn request_animation_frame(session: &XrSession, f: &Closure<dyn FnMut(f64, XrFrame)>) -> i32 {
    // This turns the Closure into a js_sys::Function
    // See https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html#casting-a-closure-to-a-js_sysfunction
    session
        .request_animation_frame(f.as_ref().unchecked_ref())
}


pub fn create_webgl_context(xr_mode: bool) -> Result<WebGl2RenderingContext, JsValue> {
    let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap(); 

    let gl: WebGl2RenderingContext = if xr_mode {
        let mut gl_attribs = HashMap::new();
        gl_attribs.insert(String::from("xrCompatible"), true);
        let js_gl_attribs = JsValue::from_serde(&gl_attribs).unwrap();

        canvas.get_context_with_context_options("webgl2", &js_gl_attribs)?.unwrap().dyn_into()?
    }
    else {
        canvas.get_context("webgl2")?.unwrap().dyn_into()?
    };

    Ok(gl)
}


#[wasm_bindgen]
pub struct XrApp {
    session: Rc<RefCell<Option<XrSession>>>,
    gl: Rc<WebGl2RenderingContext>
}


#[wasm_bindgen]
impl XrApp {

    #[wasm_bindgen(constructor)]
    pub fn new() -> XrApp {
        set_panic_hook();
        
        let session = Rc::new(RefCell::new(None));

        let xr_mode = true;
        let gl = Rc::new(create_webgl_context(xr_mode).unwrap());

        XrApp { session, gl }
    }

    pub fn init(&self) -> Promise {
        log!("Starting WebXR...");
        let navigator: web_sys::Navigator = web_sys::window().unwrap().navigator();
        let gpu = navigator.gpu();
        let xr = navigator.xr();
        let session_mode = XrSessionMode::Inline;
        let session_supported_promise = xr.is_session_supported(session_mode);        

        // Note: &self is on the stack so we can't use it in a future (which will
        // run after the &self reference is out or scope). Clone ref to the parts
        // of self we'll need, then move them into the Future
        // See https://github.com/rustwasm/wasm-bindgen/issues/1858#issuecomment-552095511
        let session = self.session.clone();
        let gl = self.gl.clone();

        let future_ = async move {
            let supports_session = wasm_bindgen_futures::JsFuture::from(session_supported_promise).await;
            let supports_session = supports_session.unwrap();
            if supports_session == false {
                log!("XR session not supported");
                // return future_to_promise(async { Ok(JsValue::from("XR session not supported"))});
                return Ok(JsValue::from("XR session not supported"));
            }
            
            let xr_session_promise = xr.request_session(session_mode);
            let xr_session = wasm_bindgen_futures::JsFuture::from(xr_session_promise).await;
            let xr_session: XrSession = xr_session.unwrap().into();
            
            let xr_gl_layer = XrWebGlLayer::new_with_web_gl2_rendering_context(&xr_session, &gl)?;
            let mut render_state_init = XrRenderStateInit::new();
            render_state_init.base_layer(Some(&xr_gl_layer));
            xr_session.update_render_state_with_state(&render_state_init);

            let mut session = session.borrow_mut();
            session.replace(xr_session);

            Ok(JsValue::from("Session set"))
        };

        future_to_promise(future_)
    }

    pub fn start(&self) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut i = 0;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64, frame: XrFrame| {
            if i > 2 {
                log!("All done!");

                // Drop our handle to this closure so that it will get cleaned
                // up once we return.
                let _ = f.borrow_mut().take();
                return;
            }

            let sess: XrSession = frame.session();
            i += 1;

            // Schedule ourself for another requestAnimationFrame callback.
            // TODO: WebXR samples call this at top of request_animation_frame - should this be moved?
            // request_animation_frame(&sess, f.borrow().as_ref().unwrap());
            
        }) as Box<dyn FnMut(f64, XrFrame)>));

        // let session_refcell: &RefCell<Option<XrSession>> = self.session.as_ref();
        let session: &Option<XrSession> = &self.session.borrow();
        // let sess: &Option<XrSession> = session_refcell.borrow();

        // let sess = if let Some(val) = session { sess } else { return () };
        let sess: &XrSession = if let Some(sess) = session { sess } else { return () };

        // sess.request_animation_frame(g.borrow().as_ref().unwrap());
        // sess.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref());

        // match session {
        //     Some(sess) => {
        //         log!("There is a session");
        //         // let i: i32 = request_animation_frame(&sess, g.borrow().as_ref().unwrap());
        //         let h = g.borrow().as_ref().unwrap();
        //         let i: i32 = sess.request_animation_frame(h.as_ref().unchecked_ref());
        //     },
        //     None => {}
        // }
    }

    pub fn start_rendering(&self) {
        // let session: &XrSession;

        // if self.session.borrow().as_ref() == None {
        //     return ();
        // }
        // else {
        //     session = self.session.borrow().as_ref().unwrap();
        // }

        // match self.session.borrow().as_ref() {
        //     Some(s) => session = s,
        //     None => { log!("No XR session. Aborting"); return ();}
        // }

        // match self.session.borrow().borrow() {
        //     Some(s) => session = s,
        //     None => { log!("No XR session. Aborting"); return ();}
        // }

        

        log!("Starting animation frame...");
        // request_animation_frame(&session, g.borrow().as_ref().unwrap());

        match &self.session.borrow().as_ref() {
            Some(s) => {
                // Reuse the logic from the requestAnimationFrame() example
                let f = Rc::new(RefCell::new(None));
                let g = f.clone();

                let mut i = 0;
                *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64, frame: XrFrame| {
                    if i > 2 {
                        log!("All done!");

                        // Drop our handle to this closure so that it will get cleaned
                        // up once we return.
                        let _ = f.borrow_mut().take();
                        return;
                    }

                    i += 1;
                    let text = format!("requestAnimationFrame has been called {} times. Time {}", i, time);
                    log!("{}", text);

                    // Schedule ourself for another requestAnimationFrame callback.
                    // let inner_session: XrSession = frame.session();
                    // request_animation_frame(&inner_session, f.borrow().as_ref().unwrap());
                }) as Box<dyn FnMut(f64, XrFrame)>));

                request_animation_frame(s, g.borrow().as_ref().unwrap());
            },
            // Some(s) => { log!("Has XR session. Rendering"); return (); }
            None => {
                log!("No XR session. Aborting"); return ();
            }
        }
    }
}
