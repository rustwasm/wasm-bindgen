use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebrtcGlobalStatisticsReport ) ]
    #[doc = "The `WebrtcGlobalStatisticsReport` dictionary.\n\n*This API requires the following crate features to be activated: `WebrtcGlobalStatisticsReport`*"]
    pub type WebrtcGlobalStatisticsReport;
}
impl WebrtcGlobalStatisticsReport {
    #[doc = "Construct a new `WebrtcGlobalStatisticsReport`.\n\n*This API requires the following crate features to be activated: `WebrtcGlobalStatisticsReport`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `reports` field of this object.\n\n*This API requires the following crate features to be activated: `WebrtcGlobalStatisticsReport`*"]
    pub fn reports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("reports"),
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
