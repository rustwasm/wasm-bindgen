use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURequestAdapterOptions ) ]
    #[doc = "The `GpuRequestAdapterOptions` dictionary.\n\n*This API requires the following crate features to be activated: `GpuRequestAdapterOptions`*"]
    pub type GpuRequestAdapterOptions;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRequestAdapterOptions {
    #[doc = "Construct a new `GpuRequestAdapterOptions`.\n\n*This API requires the following crate features to be activated: `GpuRequestAdapterOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "GpuPowerPreference")]
    #[doc = "Change the `powerPreference` field of this object.\n\n*This API requires the following crate features to be activated: `GpuPowerPreference`, `GpuRequestAdapterOptions`*"]
    pub fn power_preference(&mut self, val: GpuPowerPreference) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("powerPreference"),
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
