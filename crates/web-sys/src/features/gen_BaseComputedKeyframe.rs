use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = BaseComputedKeyframe ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `BaseComputedKeyframe` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `BaseComputedKeyframe`*
    pub type BaseComputedKeyframe;

}

impl BaseComputedKeyframe {
    ///Construct a new `BaseComputedKeyframe`.
    ///
    ///*This API requires the following crate features to be activated: `BaseComputedKeyframe`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    #[cfg(feature = "CompositeOperation")]
    ///Change the `composite` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BaseComputedKeyframe`, `CompositeOperation`*

    pub fn composite(&mut self, val: Option<CompositeOperation>) -> &mut Self {

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

    ///Change the `easing` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BaseComputedKeyframe`*

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

    ///Change the `offset` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BaseComputedKeyframe`*

    pub fn offset(&mut self, val: Option<f64>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("offset"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `simulateComputeValuesFailure` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BaseComputedKeyframe`*

    pub fn simulate_compute_values_failure(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("simulateComputeValuesFailure"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `computedOffset` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BaseComputedKeyframe`*

    pub fn computed_offset(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("computedOffset"),
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
