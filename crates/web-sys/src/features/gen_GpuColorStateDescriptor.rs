use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUColorStateDescriptor ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuColorStateDescriptor` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `GpuColorStateDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuColorStateDescriptor;

}

#[cfg(web_sys_unstable_apis)]

impl GpuColorStateDescriptor {
    #[cfg(feature = "GpuTextureFormat")]
    ///Construct a new `GpuColorStateDescriptor`.
    ///
    ///*This API requires the following crate features to be activated: `GpuColorStateDescriptor`, `GpuTextureFormat`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn new(format: GpuTextureFormat) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.format(format);

        ret
    }

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendDescriptor")]
    ///Change the `alphaBlend` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuBlendDescriptor`, `GpuColorStateDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn alpha_blend(&mut self, val: &GpuBlendDescriptor) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("alphaBlend"),
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
    #[cfg(feature = "GpuBlendDescriptor")]
    ///Change the `colorBlend` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuBlendDescriptor`, `GpuColorStateDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn color_blend(&mut self, val: &GpuBlendDescriptor) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("colorBlend"),
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
    #[cfg(feature = "GpuTextureFormat")]
    ///Change the `format` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuColorStateDescriptor`, `GpuTextureFormat`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn format(&mut self, val: GpuTextureFormat) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("format"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(web_sys_unstable_apis)]
    ///Change the `writeMask` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuColorStateDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn write_mask(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("writeMask"),
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
