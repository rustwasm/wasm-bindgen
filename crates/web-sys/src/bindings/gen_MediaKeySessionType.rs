use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum MediaKeySessionType {
    Temporary = 0,
    PersistentLicense = 1,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl MediaKeySessionType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<MediaKeySessionType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "temporary" => Some(MediaKeySessionType::Temporary),
            "persistent-license" => Some(MediaKeySessionType::PersistentLicense),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for MediaKeySessionType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for MediaKeySessionType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for MediaKeySessionType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        MediaKeySessionType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(MediaKeySessionType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for MediaKeySessionType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for MediaKeySessionType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<MediaKeySessionType> for wasm_bindgen::JsValue {
    fn from(obj: MediaKeySessionType) -> wasm_bindgen::JsValue {
        match obj {
            MediaKeySessionType::Temporary => wasm_bindgen::JsValue::from_str("temporary"),
            MediaKeySessionType::PersistentLicense => {
                wasm_bindgen::JsValue::from_str("persistent-license")
            }
            MediaKeySessionType::__Nonexhaustive => {
                panic!("attempted to convert invalid MediaKeySessionType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e64f4ad5b208b88a: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
