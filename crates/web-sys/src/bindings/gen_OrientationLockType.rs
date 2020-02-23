use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum OrientationLockType {
    Any = 0,
    Natural = 1,
    Landscape = 2,
    Portrait = 3,
    PortraitPrimary = 4,
    PortraitSecondary = 5,
    LandscapePrimary = 6,
    LandscapeSecondary = 7,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl OrientationLockType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<OrientationLockType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "any" => Some(OrientationLockType::Any),
            "natural" => Some(OrientationLockType::Natural),
            "landscape" => Some(OrientationLockType::Landscape),
            "portrait" => Some(OrientationLockType::Portrait),
            "portrait-primary" => Some(OrientationLockType::PortraitPrimary),
            "portrait-secondary" => Some(OrientationLockType::PortraitSecondary),
            "landscape-primary" => Some(OrientationLockType::LandscapePrimary),
            "landscape-secondary" => Some(OrientationLockType::LandscapeSecondary),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for OrientationLockType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for OrientationLockType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for OrientationLockType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        OrientationLockType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(OrientationLockType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for OrientationLockType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for OrientationLockType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<OrientationLockType> for wasm_bindgen::JsValue {
    fn from(obj: OrientationLockType) -> wasm_bindgen::JsValue {
        match obj {
            OrientationLockType::Any => wasm_bindgen::JsValue::from_str("any"),
            OrientationLockType::Natural => wasm_bindgen::JsValue::from_str("natural"),
            OrientationLockType::Landscape => wasm_bindgen::JsValue::from_str("landscape"),
            OrientationLockType::Portrait => wasm_bindgen::JsValue::from_str("portrait"),
            OrientationLockType::PortraitPrimary => {
                wasm_bindgen::JsValue::from_str("portrait-primary")
            }
            OrientationLockType::PortraitSecondary => {
                wasm_bindgen::JsValue::from_str("portrait-secondary")
            }
            OrientationLockType::LandscapePrimary => {
                wasm_bindgen::JsValue::from_str("landscape-primary")
            }
            OrientationLockType::LandscapeSecondary => {
                wasm_bindgen::JsValue::from_str("landscape-secondary")
            }
            OrientationLockType::__Nonexhaustive => {
                panic!("attempted to convert invalid OrientationLockType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a1025482188057c1: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
