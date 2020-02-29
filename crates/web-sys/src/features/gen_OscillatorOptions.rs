use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = OscillatorOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `OscillatorOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `OscillatorOptions`*
    pub type OscillatorOptions;

}

impl OscillatorOptions {
    ///Construct a new `OscillatorOptions`.
    ///
    ///*This API requires the following crate features to be activated: `OscillatorOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `channelCount` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `OscillatorOptions`*

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
    ///*This API requires the following crate features to be activated: `ChannelCountMode`, `OscillatorOptions`*

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
    ///*This API requires the following crate features to be activated: `ChannelInterpretation`, `OscillatorOptions`*

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

    ///Change the `detune` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `OscillatorOptions`*

    pub fn detune(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("detune"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `frequency` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `OscillatorOptions`*

    pub fn frequency(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("frequency"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "PeriodicWave")]
    ///Change the `periodicWave` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `OscillatorOptions`, `PeriodicWave`*

    pub fn periodic_wave(&mut self, val: &PeriodicWave) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("periodicWave"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "OscillatorType")]
    ///Change the `type` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `OscillatorOptions`, `OscillatorType`*

    pub fn type_(&mut self, val: OscillatorType) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
