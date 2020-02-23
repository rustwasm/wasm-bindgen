use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ClientType {
    Window = 0,
    Worker = 1,
    Sharedworker = 2,
    Serviceworker = 3,
    All = 4,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ClientType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ClientType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "window" => Some(ClientType::Window),
            "worker" => Some(ClientType::Worker),
            "sharedworker" => Some(ClientType::Sharedworker),
            "serviceworker" => Some(ClientType::Serviceworker),
            "all" => Some(ClientType::All),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ClientType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ClientType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ClientType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ClientType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ClientType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ClientType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ClientType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ClientType> for wasm_bindgen::JsValue {
    fn from(obj: ClientType) -> wasm_bindgen::JsValue {
        match obj {
            ClientType::Window => wasm_bindgen::JsValue::from_str("window"),
            ClientType::Worker => wasm_bindgen::JsValue::from_str("worker"),
            ClientType::Sharedworker => wasm_bindgen::JsValue::from_str("sharedworker"),
            ClientType::Serviceworker => wasm_bindgen::JsValue::from_str("serviceworker"),
            ClientType::All => wasm_bindgen::JsValue::from_str("all"),
            ClientType::__Nonexhaustive => {
                panic!("attempted to convert invalid ClientType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e02f6133234dfc5b: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
