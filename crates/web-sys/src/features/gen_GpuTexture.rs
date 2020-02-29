use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUTexture , typescript_name = GPUTexture ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuTexture` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture)
    ///
    ///*This API requires the following crate features to be activated: `GpuTexture`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuTexture;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUTexture" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuTexture`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn label(this: &GpuTexture) -> Option<String>;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUTexture" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuTexture`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_label(this: &GpuTexture, value: Option<&str>);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUTexture" , js_name = createView ) ]
    ///The `createView()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/createView)
    ///
    ///*This API requires the following crate features to be activated: `GpuTexture`, `GpuTextureView`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_view(this: &GpuTexture) -> GpuTextureView;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuTextureView", feature = "GpuTextureViewDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUTexture" , js_name = createView ) ]
    ///The `createView()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/createView)
    ///
    ///*This API requires the following crate features to be activated: `GpuTexture`, `GpuTextureView`, `GpuTextureViewDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_view_with_descriptor(
        this: &GpuTexture,
        descriptor: &GpuTextureViewDescriptor,
    ) -> GpuTextureView;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUTexture" , js_name = destroy ) ]
    ///The `destroy()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/destroy)
    ///
    ///*This API requires the following crate features to be activated: `GpuTexture`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn destroy(this: &GpuTexture);

}
