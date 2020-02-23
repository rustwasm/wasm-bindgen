use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum MediaKeysRequirement {
    Required = 0,
    Optional = 1,
    NotAllowed = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl MediaKeysRequirement {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<MediaKeysRequirement> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "required" => Some(MediaKeysRequirement::Required),
            "optional" => Some(MediaKeysRequirement::Optional),
            "not-allowed" => Some(MediaKeysRequirement::NotAllowed),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for MediaKeysRequirement {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for MediaKeysRequirement {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for MediaKeysRequirement {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        MediaKeysRequirement::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(MediaKeysRequirement::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for MediaKeysRequirement {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for MediaKeysRequirement {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<MediaKeysRequirement> for wasm_bindgen::JsValue {
    fn from(obj: MediaKeysRequirement) -> wasm_bindgen::JsValue {
        match obj {
            MediaKeysRequirement::Required => wasm_bindgen::JsValue::from_str("required"),
            MediaKeysRequirement::Optional => wasm_bindgen::JsValue::from_str("optional"),
            MediaKeysRequirement::NotAllowed => wasm_bindgen::JsValue::from_str("not-allowed"),
            MediaKeysRequirement::__Nonexhaustive => {
                panic!("attempted to convert invalid MediaKeysRequirement into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_03c183aa53feaea5: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
