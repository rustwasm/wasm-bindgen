use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = BlobEventInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `BlobEventInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `BlobEventInit`*
    pub type BlobEventInit;

}

impl BlobEventInit {
    ///Construct a new `BlobEventInit`.
    ///
    ///*This API requires the following crate features to be activated: `BlobEventInit`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `bubbles` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BlobEventInit`*

    pub fn bubbles(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("bubbles"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `cancelable` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BlobEventInit`*

    pub fn cancelable(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("cancelable"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `composed` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BlobEventInit`*

    pub fn composed(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("composed"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "Blob")]
    ///Change the `data` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `BlobEventInit`*

    pub fn data(&mut self, val: Option<&Blob>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("data"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
