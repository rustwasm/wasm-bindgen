use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUVertexAttributeDescriptor ) ]
    #[doc = "The `GpuVertexAttributeDescriptor` dictionary.\n\n*This API requires the following crate features to be activated: `GpuVertexAttributeDescriptor`, `GpuVertexFormat`*"]
    pub type GpuVertexAttributeDescriptor;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuVertexAttributeDescriptor {
    #[cfg(feature = "GpuVertexFormat")]
    #[doc = "Construct a new `GpuVertexAttributeDescriptor`.\n\n*This API requires the following crate features to be activated: `GpuVertexAttributeDescriptor`, `GpuVertexFormat`*"]
    pub fn new(format: GpuVertexFormat, offset: f64, shader_location: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.format(format);
        ret.offset(offset);
        ret.shader_location(shader_location);
        ret
    }
    #[cfg(feature = "GpuVertexFormat")]
    #[doc = "Change the `format` field of this object.\n\n*This API requires the following crate features to be activated: `GpuVertexAttributeDescriptor`, `GpuVertexFormat`*"]
    pub fn format(&mut self, val: GpuVertexFormat) -> &mut Self {
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
    #[doc = "Change the `offset` field of this object.\n\n*This API requires the following crate features to be activated: `GpuVertexAttributeDescriptor`*"]
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
    #[doc = "Change the `shaderLocation` field of this object.\n\n*This API requires the following crate features to be activated: `GpuVertexAttributeDescriptor`*"]
    pub fn shader_location(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("shaderLocation"),
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
