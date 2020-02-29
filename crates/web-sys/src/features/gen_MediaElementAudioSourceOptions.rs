use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaElementAudioSourceOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaElementAudioSourceOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `MediaElementAudioSourceOptions`*
    pub type MediaElementAudioSourceOptions;

}

impl MediaElementAudioSourceOptions {
    #[cfg(feature = "HtmlMediaElement")]
    ///Construct a new `MediaElementAudioSourceOptions`.
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*

    pub fn new(media_element: &HtmlMediaElement) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.media_element(media_element);

        ret
    }

    #[cfg(feature = "HtmlMediaElement")]
    ///Change the `mediaElement` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*

    pub fn media_element(&mut self, val: &HtmlMediaElement) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("mediaElement"),
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
