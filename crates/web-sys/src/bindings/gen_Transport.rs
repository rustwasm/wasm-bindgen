use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum Transport {
    Bt = 0,
    Ble = 1,
    Nfc = 2,
    Usb = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl Transport {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<Transport> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "bt" => Some(Transport::Bt),
            "ble" => Some(Transport::Ble),
            "nfc" => Some(Transport::Nfc),
            "usb" => Some(Transport::Usb),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for Transport {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for Transport {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for Transport {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        Transport::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(Transport::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for Transport {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for Transport {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<Transport> for wasm_bindgen::JsValue {
    fn from(obj: Transport) -> wasm_bindgen::JsValue {
        match obj {
            Transport::Bt => wasm_bindgen::JsValue::from_str("bt"),
            Transport::Ble => wasm_bindgen::JsValue::from_str("ble"),
            Transport::Nfc => wasm_bindgen::JsValue::from_str("nfc"),
            Transport::Usb => wasm_bindgen::JsValue::from_str("usb"),
            Transport::__Nonexhaustive => {
                panic!("attempted to convert invalid Transport into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_59b3edbc5ed73576: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
