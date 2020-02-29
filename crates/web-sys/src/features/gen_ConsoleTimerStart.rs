use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ConsoleTimerStart ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ConsoleTimerStart` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleTimerStart`*
    pub type ConsoleTimerStart;

}

impl ConsoleTimerStart {
    ///Construct a new `ConsoleTimerStart`.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleTimerStart`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `name` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConsoleTimerStart`*

    pub fn name(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("name"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
