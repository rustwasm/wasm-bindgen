use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaStreamConstraints ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaStreamConstraints` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamConstraints`*
    pub type MediaStreamConstraints;

}

impl MediaStreamConstraints {
    ///Construct a new `MediaStreamConstraints`.
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamConstraints`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `audio` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamConstraints`*

    pub fn audio(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("audio"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `fake` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamConstraints`*

    pub fn fake(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("fake"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `peerIdentity` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamConstraints`*

    pub fn peer_identity(&mut self, val: Option<&str>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("peerIdentity"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `picture` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamConstraints`*

    pub fn picture(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("picture"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `video` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamConstraints`*

    pub fn video(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

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
