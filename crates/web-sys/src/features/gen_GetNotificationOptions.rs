use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GetNotificationOptions ) ]
    #[doc = "The `GetNotificationOptions` dictionary.\n\n*This API requires the following crate features to be activated: `GetNotificationOptions`*"]
    pub type GetNotificationOptions;
}
impl GetNotificationOptions {
    #[doc = "Construct a new `GetNotificationOptions`.\n\n*This API requires the following crate features to be activated: `GetNotificationOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `tag` field of this object.\n\n*This API requires the following crate features to be activated: `GetNotificationOptions`*"]
    pub fn tag(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("tag"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
