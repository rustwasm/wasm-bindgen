use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum BrowserFindCaseSensitivity {
    CaseSensitive = 0,
    CaseInsensitive = 1,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl BrowserFindCaseSensitivity {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<BrowserFindCaseSensitivity> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "case-sensitive" => Some(BrowserFindCaseSensitivity::CaseSensitive),
            "case-insensitive" => Some(BrowserFindCaseSensitivity::CaseInsensitive),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for BrowserFindCaseSensitivity {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for BrowserFindCaseSensitivity {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for BrowserFindCaseSensitivity {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        BrowserFindCaseSensitivity::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(BrowserFindCaseSensitivity::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for BrowserFindCaseSensitivity {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for BrowserFindCaseSensitivity {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<BrowserFindCaseSensitivity> for wasm_bindgen::JsValue {
    fn from(obj: BrowserFindCaseSensitivity) -> wasm_bindgen::JsValue {
        match obj {
            BrowserFindCaseSensitivity::CaseSensitive => {
                wasm_bindgen::JsValue::from_str("case-sensitive")
            }
            BrowserFindCaseSensitivity::CaseInsensitive => {
                wasm_bindgen::JsValue::from_str("case-insensitive")
            }
            BrowserFindCaseSensitivity::__Nonexhaustive => {
                panic!("attempted to convert invalid BrowserFindCaseSensitivity into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ac66503696d8892d: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
