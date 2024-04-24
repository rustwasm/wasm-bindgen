#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUStorageTextureBindingLayout)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuStorageTextureBindingLayout` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStorageTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuStorageTextureBindingLayout;
    #[cfg(feature = "GpuStorageTextureAccess")]
    #[wasm_bindgen(method, getter = "access")]
    fn access_shim(this: &GpuStorageTextureBindingLayout) -> GpuStorageTextureAccess;
    #[cfg(feature = "GpuStorageTextureAccess")]
    #[wasm_bindgen(method, setter = "access")]
    fn set_access_shim(this: &GpuStorageTextureBindingLayout, val: GpuStorageTextureAccess);
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, getter = "format")]
    fn format_shim(this: &GpuStorageTextureBindingLayout) -> GpuTextureFormat;
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, setter = "format")]
    fn set_format_shim(this: &GpuStorageTextureBindingLayout, val: GpuTextureFormat);
    #[cfg(feature = "GpuTextureViewDimension")]
    #[wasm_bindgen(method, getter = "viewDimension")]
    fn view_dimension_shim(this: &GpuStorageTextureBindingLayout) -> GpuTextureViewDimension;
    #[cfg(feature = "GpuTextureViewDimension")]
    #[wasm_bindgen(method, setter = "viewDimension")]
    fn set_view_dimension_shim(this: &GpuStorageTextureBindingLayout, val: GpuTextureViewDimension);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuStorageTextureBindingLayout` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuStorageTextureBindingLayout`*"]
pub trait GpuStorageTextureBindingLayoutGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStorageTextureAccess")]
    #[doc = "Get the `access` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStorageTextureAccess`, `GpuStorageTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn access(&self) -> GpuStorageTextureAccess;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStorageTextureBindingLayout`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn format(&self) -> GpuTextureFormat;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDimension")]
    #[doc = "Get the `viewDimension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStorageTextureBindingLayout`, `GpuTextureViewDimension`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn view_dimension(&self) -> GpuTextureViewDimension;
}
#[cfg(web_sys_unstable_apis)]
impl GpuStorageTextureBindingLayoutGetters for GpuStorageTextureBindingLayout {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStorageTextureAccess")]
    fn access(&self) -> GpuStorageTextureAccess {
        self.access_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    fn format(&self) -> GpuTextureFormat {
        self.format_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDimension")]
    fn view_dimension(&self) -> GpuTextureViewDimension {
        self.view_dimension_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuStorageTextureBindingLayout {
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Construct a new `GpuStorageTextureBindingLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStorageTextureBindingLayout`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(format: GpuTextureFormat) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.format(format);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStorageTextureAccess")]
    #[doc = "Change the `access` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStorageTextureAccess`, `GpuStorageTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn access(&mut self, val: GpuStorageTextureAccess) -> &mut Self {
        self.set_access_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStorageTextureBindingLayout`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn format(&mut self, val: GpuTextureFormat) -> &mut Self {
        self.set_format_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDimension")]
    #[doc = "Change the `viewDimension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuStorageTextureBindingLayout`, `GpuTextureViewDimension`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn view_dimension(&mut self, val: GpuTextureViewDimension) -> &mut Self {
        self.set_view_dimension_shim(val);
        self
    }
}
