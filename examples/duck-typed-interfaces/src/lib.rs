use wasm_bindgen::prelude::*;

/// Here is a duck-typed interface for any JavaScript object that has a `quack`
/// method.
///
/// Note that any attempts to check if an object is a `Quacks` with
/// `JsCast::is_instance_of` (i.e. the `instanceof` operator) will fail because
/// there is no JS class named `Quacks`.
#[wasm_bindgen]
extern "C" {
    pub type Quacks;

    #[wasm_bindgen(structural, method)]
    pub fn quack(this: &Quacks) -> String;
}

/// Next, we can export a function that takes any object that quacks:
#[wasm_bindgen]
pub fn make_em_quack_to_this(duck: &Quacks) {
    let _s = duck.quack();
    // ...
}
