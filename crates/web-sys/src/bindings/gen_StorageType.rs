use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum StorageType {
    Persistent = 0,
    Temporary = 1,
    Default = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl StorageType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<StorageType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "persistent" => Some(StorageType::Persistent),
            "temporary" => Some(StorageType::Temporary),
            "default" => Some(StorageType::Default),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for StorageType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for StorageType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for StorageType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        StorageType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(StorageType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for StorageType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for StorageType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<StorageType> for wasm_bindgen::JsValue {
    fn from(obj: StorageType) -> wasm_bindgen::JsValue {
        match obj {
            StorageType::Persistent => wasm_bindgen::JsValue::from_str("persistent"),
            StorageType::Temporary => wasm_bindgen::JsValue::from_str("temporary"),
            StorageType::Default => wasm_bindgen::JsValue::from_str("default"),
            StorageType::__Nonexhaustive => {
                panic!("attempted to convert invalid StorageType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2e2fa31296260499: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
