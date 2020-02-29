use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaStreamTrackEventInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaStreamTrackEventInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*
    pub type MediaStreamTrackEventInit;

}

impl MediaStreamTrackEventInit {
    #[cfg(feature = "MediaStreamTrack")]
    ///Construct a new `MediaStreamTrackEventInit`.
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEventInit`*

    pub fn new(track: &MediaStreamTrack) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.track(track);

        ret
    }

    ///Change the `bubbles` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*

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
    ///*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*

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
    ///*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*

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

    #[cfg(feature = "MediaStreamTrack")]
    ///Change the `track` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEventInit`*

    pub fn track(&mut self, val: &MediaStreamTrack) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("track"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
