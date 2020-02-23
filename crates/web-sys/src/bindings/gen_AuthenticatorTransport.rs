use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum AuthenticatorTransport {
    Usb = 0,
    Nfc = 1,
    Ble = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl AuthenticatorTransport {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<AuthenticatorTransport> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "usb" => Some(AuthenticatorTransport::Usb),
            "nfc" => Some(AuthenticatorTransport::Nfc),
            "ble" => Some(AuthenticatorTransport::Ble),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for AuthenticatorTransport {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for AuthenticatorTransport {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for AuthenticatorTransport {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        AuthenticatorTransport::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(AuthenticatorTransport::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for AuthenticatorTransport {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for AuthenticatorTransport {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<AuthenticatorTransport> for wasm_bindgen::JsValue {
    fn from(obj: AuthenticatorTransport) -> wasm_bindgen::JsValue {
        match obj {
            AuthenticatorTransport::Usb => wasm_bindgen::JsValue::from_str("usb"),
            AuthenticatorTransport::Nfc => wasm_bindgen::JsValue::from_str("nfc"),
            AuthenticatorTransport::Ble => wasm_bindgen::JsValue::from_str("ble"),
            AuthenticatorTransport::__Nonexhaustive => {
                panic!("attempted to convert invalid AuthenticatorTransport into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_51010991ad947a29: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
