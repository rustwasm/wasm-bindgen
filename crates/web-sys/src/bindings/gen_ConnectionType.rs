use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ConnectionType {
    Cellular = 0,
    Bluetooth = 1,
    Ethernet = 2,
    Wifi = 3,
    Other = 4,
    None = 5,
    Unknown = 6,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ConnectionType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ConnectionType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "cellular" => Some(ConnectionType::Cellular),
            "bluetooth" => Some(ConnectionType::Bluetooth),
            "ethernet" => Some(ConnectionType::Ethernet),
            "wifi" => Some(ConnectionType::Wifi),
            "other" => Some(ConnectionType::Other),
            "none" => Some(ConnectionType::None),
            "unknown" => Some(ConnectionType::Unknown),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ConnectionType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ConnectionType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ConnectionType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ConnectionType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ConnectionType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ConnectionType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ConnectionType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ConnectionType> for wasm_bindgen::JsValue {
    fn from(obj: ConnectionType) -> wasm_bindgen::JsValue {
        match obj {
            ConnectionType::Cellular => wasm_bindgen::JsValue::from_str("cellular"),
            ConnectionType::Bluetooth => wasm_bindgen::JsValue::from_str("bluetooth"),
            ConnectionType::Ethernet => wasm_bindgen::JsValue::from_str("ethernet"),
            ConnectionType::Wifi => wasm_bindgen::JsValue::from_str("wifi"),
            ConnectionType::Other => wasm_bindgen::JsValue::from_str("other"),
            ConnectionType::None => wasm_bindgen::JsValue::from_str("none"),
            ConnectionType::Unknown => wasm_bindgen::JsValue::from_str("unknown"),
            ConnectionType::__Nonexhaustive => {
                panic!("attempted to convert invalid ConnectionType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_654d4e6818f98052: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
