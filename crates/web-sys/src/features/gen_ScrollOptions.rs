use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ScrollOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ScrollOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ScrollOptions`*
    pub type ScrollOptions;

}

impl ScrollOptions {
    ///Construct a new `ScrollOptions`.
    ///
    ///*This API requires the following crate features to be activated: `ScrollOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    #[cfg(feature = "ScrollBehavior")]
    ///Change the `behavior` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollOptions`*

    pub fn behavior(&mut self, val: ScrollBehavior) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("behavior"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
