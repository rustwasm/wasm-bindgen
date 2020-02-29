use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaConfiguration ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaConfiguration` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `MediaConfiguration`*
    pub type MediaConfiguration;

}

impl MediaConfiguration {
    ///Construct a new `MediaConfiguration`.
    ///
    ///*This API requires the following crate features to be activated: `MediaConfiguration`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    #[cfg(feature = "AudioConfiguration")]
    ///Change the `audio` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaConfiguration`*

    pub fn audio(&mut self, val: &AudioConfiguration) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("audio"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "VideoConfiguration")]
    ///Change the `video` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaConfiguration`, `VideoConfiguration`*

    pub fn video(&mut self, val: &VideoConfiguration) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("video"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
