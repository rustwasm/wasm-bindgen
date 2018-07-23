#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "events", version = "^6.9.0")]
extern {
    pub type EventEmitter;

    #[wasm_bindgen(constructor)]
    fn new() -> EventEmitter;
    #[wasm_bindgen(method)]
    fn emit(this: &EventEmitter, event: &str, a: &str, b: &str) -> String;
    #[wasm_bindgen(method)]
    fn on(this: &EventEmitter, event: &str, cb: &Closure<FnMut() -> ()>) -> String;
}

// Import `console.log` so we can log something to the console
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_emitter() -> EventEmitter {
    let emitter = EventEmitter::new();

    log("created emitter");

    return emitter;
}

#[wasm_bindgen]
pub fn create_event(emiter: EventEmitter) {
    let cb = Closure::new(move || {
        log("rsevent");
    });

    emiter.on("rsevent", &cb);
}

#[wasm_bindgen]
pub fn emit_event(emiter: EventEmitter) {
    emiter.emit("jsevent", "a", "b");

    log("emitted event");
}