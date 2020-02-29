use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUBufferBinding ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuBufferBinding` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferBinding`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuBufferBinding;

}

#[cfg(web_sys_unstable_apis)]

impl GpuBufferBinding {
    #[cfg(feature = "GpuBuffer")]
    ///Construct a new `GpuBufferBinding`.
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferBinding`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn new(buffer: &GpuBuffer) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.buffer(buffer);

        ret
    }

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    ///Change the `buffer` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferBinding`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn buffer(&mut self, val: &GpuBuffer) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("buffer"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(web_sys_unstable_apis)]
    ///Change the `offset` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferBinding`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn offset(&mut self, val: f64) -> &mut Self {

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

    #[cfg(web_sys_unstable_apis)]
    ///Change the `size` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferBinding`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn size(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("size"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
