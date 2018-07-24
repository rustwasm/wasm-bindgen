use wasm_bindgen::prelude::*;
use js_sys::{Array, Function};

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = Function)]
    fn new_function(s: &str) -> Function;

    type This;
    #[wasm_bindgen(method, getter, structural, js_name = self)]
    fn self_(me: &This) -> JsValue;
}

/// Returns whether it's likely we're executing in a browser environment, as
/// opposed to node.js.
pub fn is_browser() -> bool {
    // This is a bit tricky to define. The basic crux of this is that we want to
    // test if the `self` identifier is defined. That is defined in browsers
    // (and web workers!) but not in Node. To that end you might expect:
    //
    //      #[wasm_bindgen]
    //      extern {
    //          #[wasm_bindgen(js_name = self)]
    //          static SELF: JsValue;
    //      }
    //
    //      *SELF != JsValue::undefined()
    //
    // this currently, however, throws a "not defined" error in JS because the
    // generated function looks like `function() { return self; }` which throws
    // an error in Node because `self` isn't defined.
    //
    // To work around this limitation we instead lookup the value of `self`
    // through the `this` object, basically generating `this.self`.
    //
    // Unfortunately that's also hard to do! In ESM modes the top-level `this`
    // object is undefined, meaning that we can't just generate a function that
    // returns `this.self` as it'll throw "can't access field `self` of
    // `undefined`" whenever ESMs are being used.
    //
    // So finally we reach the current implementation. According to
    // StackOverflow you can access the global object via:
    //
    //      const global = Function('return this')();
    //
    // I think that's because the manufactured function isn't in "strict" mode.
    // It also turns out that non-strict functions will ignore `undefined`
    // values for `this` when using the `apply` function. Add it all up, and you
    // get the below code:
    //
    // * Manufacture a function
    // * Call `apply` where we specify `this` but the function ignores it
    // * Once we have `this`, use a structural getter to get the value of `self`
    // * Last but not least, test whether `self` is defined or not.
    //
    // Whew!
    let this = new_function("return this")
        .apply(&JsValue::undefined(), &Array::new())
        .unwrap();
    assert!(this != JsValue::undefined());
    This::from(this).self_() != JsValue::undefined()
}
