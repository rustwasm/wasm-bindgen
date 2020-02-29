use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = KeyframeEffectOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `KeyframeEffectOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffectOptions`*
    pub type KeyframeEffectOptions;

}

impl KeyframeEffectOptions {
    ///Construct a new `KeyframeEffectOptions`.
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffectOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `delay` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffectOptions`*

    pub fn delay(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("delay"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "PlaybackDirection")]
    ///Change the `direction` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffectOptions`, `PlaybackDirection`*

    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("direction"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `duration` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffectOptions`*

    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("duration"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `easing` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffectOptions`*

    pub fn easing(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("easing"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `endDelay` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffectOptions`*

    pub fn end_delay(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("endDelay"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "FillMode")]
    ///Change the `fill` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `FillMode`, `KeyframeEffectOptions`*

    pub fn fill(&mut self, val: FillMode) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("fill"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `iterationStart` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffectOptions`*

    pub fn iteration_start(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("iterationStart"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `iterations` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `KeyframeEffectOptions`*

    pub fn iterations(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("iterations"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "CompositeOperation")]
    ///Change the `composite` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffectOptions`*

    pub fn composite(&mut self, val: CompositeOperation) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("composite"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "IterationCompositeOperation")]
    ///Change the `iterationComposite` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffectOptions`*

    pub fn iteration_composite(&mut self, val: IterationCompositeOperation) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("iterationComposite"),
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
