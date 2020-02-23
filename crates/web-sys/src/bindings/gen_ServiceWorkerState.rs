use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ServiceWorkerState {
    Parsed = 0,
    Installing = 1,
    Installed = 2,
    Activating = 3,
    Activated = 4,
    Redundant = 5,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ServiceWorkerState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ServiceWorkerState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "parsed" => Some(ServiceWorkerState::Parsed),
            "installing" => Some(ServiceWorkerState::Installing),
            "installed" => Some(ServiceWorkerState::Installed),
            "activating" => Some(ServiceWorkerState::Activating),
            "activated" => Some(ServiceWorkerState::Activated),
            "redundant" => Some(ServiceWorkerState::Redundant),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ServiceWorkerState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ServiceWorkerState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ServiceWorkerState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ServiceWorkerState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ServiceWorkerState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ServiceWorkerState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ServiceWorkerState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ServiceWorkerState> for wasm_bindgen::JsValue {
    fn from(obj: ServiceWorkerState) -> wasm_bindgen::JsValue {
        match obj {
            ServiceWorkerState::Parsed => wasm_bindgen::JsValue::from_str("parsed"),
            ServiceWorkerState::Installing => wasm_bindgen::JsValue::from_str("installing"),
            ServiceWorkerState::Installed => wasm_bindgen::JsValue::from_str("installed"),
            ServiceWorkerState::Activating => wasm_bindgen::JsValue::from_str("activating"),
            ServiceWorkerState::Activated => wasm_bindgen::JsValue::from_str("activated"),
            ServiceWorkerState::Redundant => wasm_bindgen::JsValue::from_str("redundant"),
            ServiceWorkerState::__Nonexhaustive => {
                panic!("attempted to convert invalid ServiceWorkerState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b7d5b93e23746ba3: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
