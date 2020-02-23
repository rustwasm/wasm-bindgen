use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ImageBitmapFormat {
    Rgba32 = 0,
    Bgra32 = 1,
    Rgb24 = 2,
    Bgr24 = 3,
    Gray8 = 4,
    Yuv444p = 5,
    Yuv422p = 6,
    Yuv420p = 7,
    Yuv420spNv12 = 8,
    Yuv420spNv21 = 9,
    Hsv = 10,
    Lab = 11,
    Depth = 12,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ImageBitmapFormat {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ImageBitmapFormat> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "RGBA32" => Some(ImageBitmapFormat::Rgba32),
            "BGRA32" => Some(ImageBitmapFormat::Bgra32),
            "RGB24" => Some(ImageBitmapFormat::Rgb24),
            "BGR24" => Some(ImageBitmapFormat::Bgr24),
            "GRAY8" => Some(ImageBitmapFormat::Gray8),
            "YUV444P" => Some(ImageBitmapFormat::Yuv444p),
            "YUV422P" => Some(ImageBitmapFormat::Yuv422p),
            "YUV420P" => Some(ImageBitmapFormat::Yuv420p),
            "YUV420SP_NV12" => Some(ImageBitmapFormat::Yuv420spNv12),
            "YUV420SP_NV21" => Some(ImageBitmapFormat::Yuv420spNv21),
            "HSV" => Some(ImageBitmapFormat::Hsv),
            "Lab" => Some(ImageBitmapFormat::Lab),
            "DEPTH" => Some(ImageBitmapFormat::Depth),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ImageBitmapFormat {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ImageBitmapFormat {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ImageBitmapFormat {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ImageBitmapFormat::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ImageBitmapFormat::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ImageBitmapFormat {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ImageBitmapFormat {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ImageBitmapFormat> for wasm_bindgen::JsValue {
    fn from(obj: ImageBitmapFormat) -> wasm_bindgen::JsValue {
        match obj {
            ImageBitmapFormat::Rgba32 => wasm_bindgen::JsValue::from_str("RGBA32"),
            ImageBitmapFormat::Bgra32 => wasm_bindgen::JsValue::from_str("BGRA32"),
            ImageBitmapFormat::Rgb24 => wasm_bindgen::JsValue::from_str("RGB24"),
            ImageBitmapFormat::Bgr24 => wasm_bindgen::JsValue::from_str("BGR24"),
            ImageBitmapFormat::Gray8 => wasm_bindgen::JsValue::from_str("GRAY8"),
            ImageBitmapFormat::Yuv444p => wasm_bindgen::JsValue::from_str("YUV444P"),
            ImageBitmapFormat::Yuv422p => wasm_bindgen::JsValue::from_str("YUV422P"),
            ImageBitmapFormat::Yuv420p => wasm_bindgen::JsValue::from_str("YUV420P"),
            ImageBitmapFormat::Yuv420spNv12 => wasm_bindgen::JsValue::from_str("YUV420SP_NV12"),
            ImageBitmapFormat::Yuv420spNv21 => wasm_bindgen::JsValue::from_str("YUV420SP_NV21"),
            ImageBitmapFormat::Hsv => wasm_bindgen::JsValue::from_str("HSV"),
            ImageBitmapFormat::Lab => wasm_bindgen::JsValue::from_str("Lab"),
            ImageBitmapFormat::Depth => wasm_bindgen::JsValue::from_str("DEPTH"),
            ImageBitmapFormat::__Nonexhaustive => {
                panic!("attempted to convert invalid ImageBitmapFormat into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5e61086fbc2eaa6d: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
