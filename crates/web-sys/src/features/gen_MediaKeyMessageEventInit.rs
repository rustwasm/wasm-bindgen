use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaKeyMessageEventInit ) ]
    #[doc = "The `MediaKeyMessageEventInit` dictionary.\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"]
    pub type MediaKeyMessageEventInit;
}
impl MediaKeyMessageEventInit {
    #[cfg(feature = "MediaKeyMessageType")]
    #[doc = "Construct a new `MediaKeyMessageEventInit`.\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"]
    pub fn new(message: &::js_sys::ArrayBuffer, message_type: MediaKeyMessageType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.message(message);
        ret.message_type(message_type);
        ret
    }
    #[doc = "Change the `bubbles` field of this object.\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
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
    #[doc = "Change the `cancelable` field of this object.\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
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
    #[doc = "Change the `composed` field of this object.\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
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
    #[doc = "Change the `message` field of this object.\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    pub fn message(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("message"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "MediaKeyMessageType")]
    #[doc = "Change the `messageType` field of this object.\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"]
    pub fn message_type(&mut self, val: MediaKeyMessageType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("messageType"),
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
