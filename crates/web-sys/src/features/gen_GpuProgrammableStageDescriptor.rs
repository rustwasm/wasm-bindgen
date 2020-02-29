use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUProgrammableStageDescriptor ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuProgrammableStageDescriptor` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `GpuProgrammableStageDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuProgrammableStageDescriptor;

}

#[cfg(web_sys_unstable_apis)]

impl GpuProgrammableStageDescriptor {
    #[cfg(feature = "GpuShaderModule")]
    ///Construct a new `GpuProgrammableStageDescriptor`.
    ///
    ///*This API requires the following crate features to be activated: `GpuProgrammableStageDescriptor`, `GpuShaderModule`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn new(entry_point: &str, module: &GpuShaderModule) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.entry_point(entry_point);

        ret.module(module);

        ret
    }

    #[cfg(web_sys_unstable_apis)]
    ///Change the `entryPoint` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuProgrammableStageDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn entry_point(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("entryPoint"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    ///Change the `module` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuProgrammableStageDescriptor`, `GpuShaderModule`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn module(&mut self, val: &GpuShaderModule) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("module"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
