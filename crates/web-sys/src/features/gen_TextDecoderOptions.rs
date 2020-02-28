use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TextDecoderOptions ) ]
    #[doc = "The `TextDecoderOptions` dictionary.\n\n*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
    pub type TextDecoderOptions;
}
impl TextDecoderOptions {
    #[doc = "Construct a new `TextDecoderOptions`.\n\n*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `fatal` field of this object.\n\n*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
    pub fn fatal(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("fatal"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
