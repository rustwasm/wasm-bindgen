use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // stack
    fn takes_immutable_closure(f: &dyn Fn());
    fn takes_mutable_closure(f: &mut dyn FnMut());

    // heap
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
}

#[wasm_bindgen]
pub fn exported() {
    takes_immutable_closure(&|| {});
    takes_mutable_closure(&mut || {});

    let closure = Closure::new(|| {});
    setInterval(&closure, 1000);
}
