use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PannerOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PannerOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*
    pub type PannerOptions;

}

impl PannerOptions {
    ///Construct a new `PannerOptions`.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `channelCount` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

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
    ///*This API requires the following crate features to be activated: `ChannelCountMode`, `PannerOptions`*

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
    ///*This API requires the following crate features to be activated: `ChannelInterpretation`, `PannerOptions`*

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

    ///Change the `coneInnerAngle` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn cone_inner_angle(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("coneInnerAngle"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `coneOuterAngle` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn cone_outer_angle(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("coneOuterAngle"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `coneOuterGain` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn cone_outer_gain(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("coneOuterGain"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "DistanceModelType")]
    ///Change the `distanceModel` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DistanceModelType`, `PannerOptions`*

    pub fn distance_model(&mut self, val: DistanceModelType) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("distanceModel"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `maxDistance` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn max_distance(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("maxDistance"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `orientationX` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn orientation_x(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("orientationX"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `orientationY` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn orientation_y(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("orientationY"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `orientationZ` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn orientation_z(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("orientationZ"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "PanningModelType")]
    ///Change the `panningModel` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`, `PanningModelType`*

    pub fn panning_model(&mut self, val: PanningModelType) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("panningModel"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `positionX` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn position_x(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("positionX"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `positionY` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn position_y(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("positionY"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `positionZ` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn position_z(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("positionZ"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `refDistance` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn ref_distance(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("refDistance"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `rolloffFactor` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `PannerOptions`*

    pub fn rolloff_factor(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rolloffFactor"),
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
