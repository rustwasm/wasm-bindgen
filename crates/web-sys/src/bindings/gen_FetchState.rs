use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum FetchState {
    Requesting = 0,
    Responding = 1,
    Aborted = 2,
    Errored = 3,
    Complete = 4,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl FetchState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<FetchState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "requesting" => Some(FetchState::Requesting),
            "responding" => Some(FetchState::Responding),
            "aborted" => Some(FetchState::Aborted),
            "errored" => Some(FetchState::Errored),
            "complete" => Some(FetchState::Complete),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for FetchState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for FetchState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for FetchState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        FetchState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(FetchState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for FetchState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for FetchState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<FetchState> for wasm_bindgen::JsValue {
    fn from(obj: FetchState) -> wasm_bindgen::JsValue {
        match obj {
            FetchState::Requesting => wasm_bindgen::JsValue::from_str("requesting"),
            FetchState::Responding => wasm_bindgen::JsValue::from_str("responding"),
            FetchState::Aborted => wasm_bindgen::JsValue::from_str("aborted"),
            FetchState::Errored => wasm_bindgen::JsValue::from_str("errored"),
            FetchState::Complete => wasm_bindgen::JsValue::from_str("complete"),
            FetchState::__Nonexhaustive => {
                panic!("attempted to convert invalid FetchState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7bf36883539f2ade: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
