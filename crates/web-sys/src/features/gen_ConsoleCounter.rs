use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ConsoleCounter ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ConsoleCounter` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleCounter`*
    pub type ConsoleCounter;

}

impl ConsoleCounter {
    ///Construct a new `ConsoleCounter`.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleCounter`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `count` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleCounter`*

    pub fn count(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("count"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `label` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleCounter`*

    pub fn label(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("label"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
