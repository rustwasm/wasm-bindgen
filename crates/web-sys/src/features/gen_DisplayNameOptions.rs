use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DisplayNameOptions ) ]
    #[doc = "The `DisplayNameOptions` dictionary.\n\n*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub type DisplayNameOptions;
}
impl DisplayNameOptions {
    #[doc = "Construct a new `DisplayNameOptions`.\n\n*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `keys` field of this object.\n\n*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub fn keys(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("keys"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `style` field of this object.\n\n*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub fn style(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("style"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
