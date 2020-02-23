use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcLifecycleEvent {
    Initialized = 0,
    Icegatheringstatechange = 1,
    Iceconnectionstatechange = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcLifecycleEvent {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcLifecycleEvent> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "initialized" => Some(RtcLifecycleEvent::Initialized),
            "icegatheringstatechange" => Some(RtcLifecycleEvent::Icegatheringstatechange),
            "iceconnectionstatechange" => Some(RtcLifecycleEvent::Iceconnectionstatechange),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcLifecycleEvent {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcLifecycleEvent {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcLifecycleEvent {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcLifecycleEvent::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcLifecycleEvent::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcLifecycleEvent {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcLifecycleEvent {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcLifecycleEvent> for wasm_bindgen::JsValue {
    fn from(obj: RtcLifecycleEvent) -> wasm_bindgen::JsValue {
        match obj {
            RtcLifecycleEvent::Initialized => wasm_bindgen::JsValue::from_str("initialized"),
            RtcLifecycleEvent::Icegatheringstatechange => {
                wasm_bindgen::JsValue::from_str("icegatheringstatechange")
            }
            RtcLifecycleEvent::Iceconnectionstatechange => {
                wasm_bindgen::JsValue::from_str("iceconnectionstatechange")
            }
            RtcLifecycleEvent::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcLifecycleEvent into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b0bc48e134b3bfd5: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
