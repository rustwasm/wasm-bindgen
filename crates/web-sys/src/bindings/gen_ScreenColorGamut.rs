use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ScreenColorGamut {
    Srgb = 0,
    P3 = 1,
    Rec2020 = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ScreenColorGamut {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ScreenColorGamut> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "srgb" => Some(ScreenColorGamut::Srgb),
            "p3" => Some(ScreenColorGamut::P3),
            "rec2020" => Some(ScreenColorGamut::Rec2020),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ScreenColorGamut {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ScreenColorGamut {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ScreenColorGamut {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ScreenColorGamut::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ScreenColorGamut::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ScreenColorGamut {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ScreenColorGamut {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ScreenColorGamut> for wasm_bindgen::JsValue {
    fn from(obj: ScreenColorGamut) -> wasm_bindgen::JsValue {
        match obj {
            ScreenColorGamut::Srgb => wasm_bindgen::JsValue::from_str("srgb"),
            ScreenColorGamut::P3 => wasm_bindgen::JsValue::from_str("p3"),
            ScreenColorGamut::Rec2020 => wasm_bindgen::JsValue::from_str("rec2020"),
            ScreenColorGamut::__Nonexhaustive => {
                panic!("attempted to convert invalid ScreenColorGamut into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8fe5bfb11d5634c7: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
