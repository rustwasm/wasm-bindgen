use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ProfileTimelineWorkerOperationType {
    SerializeDataOffMainThread = 0,
    SerializeDataOnMainThread = 1,
    DeserializeDataOffMainThread = 2,
    DeserializeDataOnMainThread = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ProfileTimelineWorkerOperationType {
    pub fn from_js_value(
        obj: &wasm_bindgen::JsValue,
    ) -> Option<ProfileTimelineWorkerOperationType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "serializeDataOffMainThread" => {
                Some(ProfileTimelineWorkerOperationType::SerializeDataOffMainThread)
            }
            "serializeDataOnMainThread" => {
                Some(ProfileTimelineWorkerOperationType::SerializeDataOnMainThread)
            }
            "deserializeDataOffMainThread" => {
                Some(ProfileTimelineWorkerOperationType::DeserializeDataOffMainThread)
            }
            "deserializeDataOnMainThread" => {
                Some(ProfileTimelineWorkerOperationType::DeserializeDataOnMainThread)
            }
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ProfileTimelineWorkerOperationType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ProfileTimelineWorkerOperationType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ProfileTimelineWorkerOperationType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ProfileTimelineWorkerOperationType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ProfileTimelineWorkerOperationType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ProfileTimelineWorkerOperationType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ProfileTimelineWorkerOperationType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ProfileTimelineWorkerOperationType> for wasm_bindgen::JsValue {
    fn from(obj: ProfileTimelineWorkerOperationType) -> wasm_bindgen::JsValue {
        match obj {
            ProfileTimelineWorkerOperationType::SerializeDataOffMainThread => {
                wasm_bindgen::JsValue::from_str("serializeDataOffMainThread")
            }
            ProfileTimelineWorkerOperationType::SerializeDataOnMainThread => {
                wasm_bindgen::JsValue::from_str("serializeDataOnMainThread")
            }
            ProfileTimelineWorkerOperationType::DeserializeDataOffMainThread => {
                wasm_bindgen::JsValue::from_str("deserializeDataOffMainThread")
            }
            ProfileTimelineWorkerOperationType::DeserializeDataOnMainThread => {
                wasm_bindgen::JsValue::from_str("deserializeDataOnMainThread")
            }
            ProfileTimelineWorkerOperationType::__Nonexhaustive => panic!(
                "attempted to convert invalid ProfileTimelineWorkerOperationType into JSValue"
            ),
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_23bbc1cd1a6ea0b4: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
