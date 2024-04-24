#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoFrameBufferInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoFrameBufferInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoFrameBufferInit;
    #[wasm_bindgen(method, getter = "codedHeight")]
    fn coded_height_shim(this: &VideoFrameBufferInit) -> u32;
    #[wasm_bindgen(method, setter = "codedHeight")]
    fn set_coded_height_shim(this: &VideoFrameBufferInit, val: u32);
    #[wasm_bindgen(method, getter = "codedWidth")]
    fn coded_width_shim(this: &VideoFrameBufferInit) -> u32;
    #[wasm_bindgen(method, setter = "codedWidth")]
    fn set_coded_width_shim(this: &VideoFrameBufferInit, val: u32);
    #[cfg(feature = "VideoColorSpaceInit")]
    #[wasm_bindgen(method, getter = "colorSpace")]
    fn color_space_shim(this: &VideoFrameBufferInit) -> &VideoColorSpaceInit;
    #[cfg(feature = "VideoColorSpaceInit")]
    #[wasm_bindgen(method, setter = "colorSpace")]
    fn set_color_space_shim(this: &VideoFrameBufferInit, val: &VideoColorSpaceInit);
    #[wasm_bindgen(method, getter = "displayHeight")]
    fn display_height_shim(this: &VideoFrameBufferInit) -> u32;
    #[wasm_bindgen(method, setter = "displayHeight")]
    fn set_display_height_shim(this: &VideoFrameBufferInit, val: u32);
    #[wasm_bindgen(method, getter = "displayWidth")]
    fn display_width_shim(this: &VideoFrameBufferInit) -> u32;
    #[wasm_bindgen(method, setter = "displayWidth")]
    fn set_display_width_shim(this: &VideoFrameBufferInit, val: u32);
    #[wasm_bindgen(method, getter = "duration")]
    fn duration_shim(this: &VideoFrameBufferInit) -> f64;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration_shim(this: &VideoFrameBufferInit, val: f64);
    #[cfg(feature = "VideoPixelFormat")]
    #[wasm_bindgen(method, getter = "format")]
    fn format_shim(this: &VideoFrameBufferInit) -> VideoPixelFormat;
    #[cfg(feature = "VideoPixelFormat")]
    #[wasm_bindgen(method, setter = "format")]
    fn set_format_shim(this: &VideoFrameBufferInit, val: VideoPixelFormat);
    #[wasm_bindgen(method, getter = "layout")]
    fn layout_shim(this: &VideoFrameBufferInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "layout")]
    fn set_layout_shim(this: &VideoFrameBufferInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &VideoFrameBufferInit) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &VideoFrameBufferInit, val: f64);
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, getter = "visibleRect")]
    fn visible_rect_shim(this: &VideoFrameBufferInit) -> &DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, setter = "visibleRect")]
    fn set_visible_rect_shim(this: &VideoFrameBufferInit, val: &DomRectInit);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `VideoFrameBufferInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
pub trait VideoFrameBufferInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `codedHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn coded_height(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `codedWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn coded_width(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorSpaceInit")]
    #[doc = "Get the `colorSpace` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn color_space(&self) -> &VideoColorSpaceInit;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn display_height(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn display_width(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn duration(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoPixelFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`, `VideoPixelFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn format(&self) -> VideoPixelFormat;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn layout(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn timestamp(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `visibleRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn visible_rect(&self) -> &DomRectInit;
}
#[cfg(web_sys_unstable_apis)]
impl VideoFrameBufferInitGetters for VideoFrameBufferInit {
    #[cfg(web_sys_unstable_apis)]
    fn coded_height(&self) -> u32 {
        self.coded_height_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn coded_width(&self) -> u32 {
        self.coded_width_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorSpaceInit")]
    fn color_space(&self) -> &VideoColorSpaceInit {
        self.color_space_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn display_height(&self) -> u32 {
        self.display_height_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn display_width(&self) -> u32 {
        self.display_width_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn duration(&self) -> f64 {
        self.duration_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoPixelFormat")]
    fn format(&self) -> VideoPixelFormat {
        self.format_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn layout(&self) -> &::wasm_bindgen::JsValue {
        self.layout_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectInit")]
    fn visible_rect(&self) -> &DomRectInit {
        self.visible_rect_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl VideoFrameBufferInit {
    #[cfg(feature = "VideoPixelFormat")]
    #[doc = "Construct a new `VideoFrameBufferInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`, `VideoPixelFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        coded_height: u32,
        coded_width: u32,
        format: VideoPixelFormat,
        timestamp: f64,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.coded_height(coded_height);
        ret.coded_width(coded_width);
        ret.format(format);
        ret.timestamp(timestamp);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `codedHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn coded_height(&mut self, val: u32) -> &mut Self {
        self.set_coded_height_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `codedWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn coded_width(&mut self, val: u32) -> &mut Self {
        self.set_coded_width_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorSpaceInit")]
    #[doc = "Change the `colorSpace` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn color_space(&mut self, val: &VideoColorSpaceInit) -> &mut Self {
        self.set_color_space_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn display_height(&mut self, val: u32) -> &mut Self {
        self.set_display_height_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn display_width(&mut self, val: u32) -> &mut Self {
        self.set_display_width_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.set_duration_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoPixelFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`, `VideoPixelFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn format(&mut self, val: VideoPixelFormat) -> &mut Self {
        self.set_format_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn layout(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_layout_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `visibleRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn visible_rect(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_visible_rect_shim(val);
        self
    }
}
