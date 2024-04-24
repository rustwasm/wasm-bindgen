#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUComputePassTimestampWrites)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuComputePassTimestampWrites` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuComputePassTimestampWrites;
    #[wasm_bindgen(method, getter = "beginningOfPassWriteIndex")]
    fn beginning_of_pass_write_index_shim(this: &GpuComputePassTimestampWrites) -> u32;
    #[wasm_bindgen(method, setter = "beginningOfPassWriteIndex")]
    fn set_beginning_of_pass_write_index_shim(this: &GpuComputePassTimestampWrites, val: u32);
    #[wasm_bindgen(method, getter = "endOfPassWriteIndex")]
    fn end_of_pass_write_index_shim(this: &GpuComputePassTimestampWrites) -> u32;
    #[wasm_bindgen(method, setter = "endOfPassWriteIndex")]
    fn set_end_of_pass_write_index_shim(this: &GpuComputePassTimestampWrites, val: u32);
    #[cfg(feature = "GpuQuerySet")]
    #[wasm_bindgen(method, getter = "querySet")]
    fn query_set_shim(this: &GpuComputePassTimestampWrites) -> GpuQuerySet;
    #[cfg(feature = "GpuQuerySet")]
    #[wasm_bindgen(method, setter = "querySet")]
    fn set_query_set_shim(this: &GpuComputePassTimestampWrites, val: &GpuQuerySet);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuComputePassTimestampWrites` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuComputePassTimestampWrites`*"]
pub trait GpuComputePassTimestampWritesGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `beginningOfPassWriteIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn beginning_of_pass_write_index(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `endOfPassWriteIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn end_of_pass_write_index(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    #[doc = "Get the `querySet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassTimestampWrites`, `GpuQuerySet`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn query_set(&self) -> GpuQuerySet;
}
#[cfg(web_sys_unstable_apis)]
impl GpuComputePassTimestampWritesGetters for GpuComputePassTimestampWrites {
    #[cfg(web_sys_unstable_apis)]
    fn beginning_of_pass_write_index(&self) -> u32 {
        self.beginning_of_pass_write_index_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn end_of_pass_write_index(&self) -> u32 {
        self.end_of_pass_write_index_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    fn query_set(&self) -> GpuQuerySet {
        self.query_set_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuComputePassTimestampWrites {
    #[cfg(feature = "GpuQuerySet")]
    #[doc = "Construct a new `GpuComputePassTimestampWrites`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassTimestampWrites`, `GpuQuerySet`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(query_set: &GpuQuerySet) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::query_set(&mut ret, query_set);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `beginningOfPassWriteIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn beginning_of_pass_write_index(&mut self, val: u32) -> &mut Self {
        self.set_beginning_of_pass_write_index_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `endOfPassWriteIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn end_of_pass_write_index(&mut self, val: u32) -> &mut Self {
        self.set_end_of_pass_write_index_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    #[doc = "Change the `querySet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassTimestampWrites`, `GpuQuerySet`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn query_set(&mut self, val: &GpuQuerySet) -> &mut Self {
        self.set_query_set_shim(val);
        self
    }
}
