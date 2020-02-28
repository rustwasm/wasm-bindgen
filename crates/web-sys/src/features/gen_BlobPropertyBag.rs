use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = BlobPropertyBag ) ]
    #[doc = "The `BlobPropertyBag` dictionary.\n\n*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
    pub type BlobPropertyBag;
}
impl BlobPropertyBag {
    #[doc = "Construct a new `BlobPropertyBag`.\n\n*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "EndingTypes")]
    #[doc = "Change the `endings` field of this object.\n\n*This API requires the following crate features to be activated: `BlobPropertyBag`, `EndingTypes`*"]
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
    #[doc = "Change the `type` field of this object.\n\n*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
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
