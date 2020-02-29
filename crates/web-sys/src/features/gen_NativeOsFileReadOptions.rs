use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = NativeOSFileReadOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `NativeOsFileReadOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*
    pub type NativeOsFileReadOptions;

}

impl NativeOsFileReadOptions {
    ///Construct a new `NativeOsFileReadOptions`.
    ///
    ///*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `bytes` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*

    pub fn bytes(&mut self, val: Option<f64>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("bytes"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `encoding` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*

    pub fn encoding(&mut self, val: Option<&str>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("encoding"),
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
