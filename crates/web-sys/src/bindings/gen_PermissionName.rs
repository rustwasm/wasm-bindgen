use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum PermissionName {
    Geolocation = 0,
    Notifications = 1,
    Push = 2,
    PersistentStorage = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl PermissionName {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<PermissionName> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "geolocation" => Some(PermissionName::Geolocation),
            "notifications" => Some(PermissionName::Notifications),
            "push" => Some(PermissionName::Push),
            "persistent-storage" => Some(PermissionName::PersistentStorage),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for PermissionName {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for PermissionName {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for PermissionName {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        PermissionName::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(PermissionName::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for PermissionName {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for PermissionName {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<PermissionName> for wasm_bindgen::JsValue {
    fn from(obj: PermissionName) -> wasm_bindgen::JsValue {
        match obj {
            PermissionName::Geolocation => wasm_bindgen::JsValue::from_str("geolocation"),
            PermissionName::Notifications => wasm_bindgen::JsValue::from_str("notifications"),
            PermissionName::Push => wasm_bindgen::JsValue::from_str("push"),
            PermissionName::PersistentStorage => {
                wasm_bindgen::JsValue::from_str("persistent-storage")
            }
            PermissionName::__Nonexhaustive => {
                panic!("attempted to convert invalid PermissionName into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_35b7da49c101ae0b: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
