use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum LineAlignSetting {
    Start = 0,
    Center = 1,
    End = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl LineAlignSetting {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<LineAlignSetting> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "start" => Some(LineAlignSetting::Start),
            "center" => Some(LineAlignSetting::Center),
            "end" => Some(LineAlignSetting::End),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for LineAlignSetting {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for LineAlignSetting {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for LineAlignSetting {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        LineAlignSetting::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(LineAlignSetting::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for LineAlignSetting {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for LineAlignSetting {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<LineAlignSetting> for wasm_bindgen::JsValue {
    fn from(obj: LineAlignSetting) -> wasm_bindgen::JsValue {
        match obj {
            LineAlignSetting::Start => wasm_bindgen::JsValue::from_str("start"),
            LineAlignSetting::Center => wasm_bindgen::JsValue::from_str("center"),
            LineAlignSetting::End => wasm_bindgen::JsValue::from_str("end"),
            LineAlignSetting::__Nonexhaustive => {
                panic!("attempted to convert invalid LineAlignSetting into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2a120d831cf2330a: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
