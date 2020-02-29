use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RcwnStatus ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RcwnStatus` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RcwnStatus`*
    pub type RcwnStatus;

}

impl RcwnStatus {
    ///Construct a new `RcwnStatus`.
    ///
    ///*This API requires the following crate features to be activated: `RcwnStatus`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `cacheNotSlowCount` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RcwnStatus`*

    pub fn cache_not_slow_count(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("cacheNotSlowCount"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `cacheSlowCount` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RcwnStatus`*

    pub fn cache_slow_count(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("cacheSlowCount"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `perfStats` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RcwnStatus`*

    pub fn perf_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("perfStats"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `rcwnCacheWonCount` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RcwnStatus`*

    pub fn rcwn_cache_won_count(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rcwnCacheWonCount"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `rcwnNetWonCount` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RcwnStatus`*

    pub fn rcwn_net_won_count(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rcwnNetWonCount"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `totalNetworkRequests` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RcwnStatus`*

    pub fn total_network_requests(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("totalNetworkRequests"),
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
