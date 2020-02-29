use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ObserverCallback ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ObserverCallback` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ObserverCallback`*
    pub type ObserverCallback;

}

impl ObserverCallback {
    ///Construct a new `ObserverCallback`.
    ///
    ///*This API requires the following crate features to be activated: `ObserverCallback`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `handleEvent` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ObserverCallback`*

    pub fn handle_event(&mut self, val: &::js_sys::Function) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("handleEvent"),
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
