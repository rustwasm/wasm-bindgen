use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURenderPipeline , typescript_type = "GPURenderPipeline" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuRenderPipeline` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPipeline)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPipeline`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuRenderPipeline;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPURenderPipeline" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPipeline/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPipeline`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn label(this: &GpuRenderPipeline) -> Option<String>;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPURenderPipeline" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPipeline/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPipeline`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_label(this: &GpuRenderPipeline, value: Option<&str>);

}
