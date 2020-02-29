use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURenderPassColorAttachmentDescriptor ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuRenderPassColorAttachmentDescriptor` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassColorAttachmentDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuRenderPassColorAttachmentDescriptor;

}

#[cfg(web_sys_unstable_apis)]

impl GpuRenderPassColorAttachmentDescriptor {
    #[cfg(feature = "GpuTextureView")]
    ///Construct a new `GpuRenderPassColorAttachmentDescriptor`.
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassColorAttachmentDescriptor`, `GpuTextureView`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn new(attachment: &GpuTextureView, load_value: &::wasm_bindgen::JsValue) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.attachment(attachment);

        ret.load_value(load_value);

        ret
    }

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    ///Change the `attachment` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassColorAttachmentDescriptor`, `GpuTextureView`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn attachment(&mut self, val: &GpuTextureView) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("attachment"),
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
    ///Change the `loadValue` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassColorAttachmentDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn load_value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("loadValue"),
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
    #[cfg(feature = "GpuTextureView")]
    ///Change the `resolveTarget` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassColorAttachmentDescriptor`, `GpuTextureView`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn resolve_target(&mut self, val: &GpuTextureView) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("resolveTarget"),
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
    #[cfg(feature = "GpuStoreOp")]
    ///Change the `storeOp` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassColorAttachmentDescriptor`, `GpuStoreOp`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub fn store_op(&mut self, val: GpuStoreOp) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("storeOp"),
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
