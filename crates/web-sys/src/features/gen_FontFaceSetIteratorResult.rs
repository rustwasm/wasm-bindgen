use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FontFaceSetIteratorResult ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FontFaceSetIteratorResult` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*
    pub type FontFaceSetIteratorResult;

}

impl FontFaceSetIteratorResult {
    ///Construct a new `FontFaceSetIteratorResult`.
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*

    pub fn new(done: bool, value: &::wasm_bindgen::JsValue) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.done(done);

        ret.value(value);

        ret
    }

    ///Change the `done` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*

    pub fn done(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("done"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `value` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*

    pub fn value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("value"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
