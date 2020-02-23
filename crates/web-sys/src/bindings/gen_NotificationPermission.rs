use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum NotificationPermission {
    Default = 0,
    Denied = 1,
    Granted = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl NotificationPermission {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<NotificationPermission> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "default" => Some(NotificationPermission::Default),
            "denied" => Some(NotificationPermission::Denied),
            "granted" => Some(NotificationPermission::Granted),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for NotificationPermission {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for NotificationPermission {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for NotificationPermission {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        NotificationPermission::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(NotificationPermission::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for NotificationPermission {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for NotificationPermission {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<NotificationPermission> for wasm_bindgen::JsValue {
    fn from(obj: NotificationPermission) -> wasm_bindgen::JsValue {
        match obj {
            NotificationPermission::Default => wasm_bindgen::JsValue::from_str("default"),
            NotificationPermission::Denied => wasm_bindgen::JsValue::from_str("denied"),
            NotificationPermission::Granted => wasm_bindgen::JsValue::from_str("granted"),
            NotificationPermission::__Nonexhaustive => {
                panic!("attempted to convert invalid NotificationPermission into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7a0c482880c1d6bf: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
