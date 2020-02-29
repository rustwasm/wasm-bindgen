use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = BlobPropertyBag ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `BlobPropertyBag` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `BlobPropertyBag`*
    pub type BlobPropertyBag;

}

impl BlobPropertyBag {
    ///Construct a new `BlobPropertyBag`.
    ///
    ///*This API requires the following crate features to be activated: `BlobPropertyBag`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    #[cfg(feature = "EndingTypes")]
    ///Change the `endings` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BlobPropertyBag`, `EndingTypes`*

    pub fn endings(&mut self, val: EndingTypes) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("endings"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `type` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BlobPropertyBag`*

    pub fn type_(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
