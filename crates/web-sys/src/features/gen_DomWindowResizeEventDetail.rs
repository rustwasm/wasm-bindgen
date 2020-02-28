use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMWindowResizeEventDetail ) ]
    #[doc = "The `DomWindowResizeEventDetail` dictionary.\n\n*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
    pub type DomWindowResizeEventDetail;
}
impl DomWindowResizeEventDetail {
    #[doc = "Construct a new `DomWindowResizeEventDetail`.\n\n*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `height` field of this object.\n\n*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
    pub fn height(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("height"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `width` field of this object.\n\n*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
    pub fn width(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("width"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
