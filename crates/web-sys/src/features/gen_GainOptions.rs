use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GainOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GainOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `GainOptions`*
    pub type GainOptions;

}

impl GainOptions {
    ///Construct a new `GainOptions`.
    ///
    ///*This API requires the following crate features to be activated: `GainOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `channelCount` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GainOptions`*

    pub fn channel_count(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("channelCount"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "ChannelCountMode")]
    ///Change the `channelCountMode` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ChannelCountMode`, `GainOptions`*

    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("channelCountMode"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "ChannelInterpretation")]
    ///Change the `channelInterpretation` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ChannelInterpretation`, `GainOptions`*

    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("channelInterpretation"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `gain` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GainOptions`*

    pub fn gain(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("gain"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
