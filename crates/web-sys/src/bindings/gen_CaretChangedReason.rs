use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum CaretChangedReason {
    Visibilitychange = 0,
    Updateposition = 1,
    Longpressonemptycontent = 2,
    Taponcaret = 3,
    Presscaret = 4,
    Releasecaret = 5,
    Scroll = 6,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl CaretChangedReason {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<CaretChangedReason> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "visibilitychange" => Some(CaretChangedReason::Visibilitychange),
            "updateposition" => Some(CaretChangedReason::Updateposition),
            "longpressonemptycontent" => Some(CaretChangedReason::Longpressonemptycontent),
            "taponcaret" => Some(CaretChangedReason::Taponcaret),
            "presscaret" => Some(CaretChangedReason::Presscaret),
            "releasecaret" => Some(CaretChangedReason::Releasecaret),
            "scroll" => Some(CaretChangedReason::Scroll),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for CaretChangedReason {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for CaretChangedReason {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for CaretChangedReason {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        CaretChangedReason::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(CaretChangedReason::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for CaretChangedReason {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for CaretChangedReason {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<CaretChangedReason> for wasm_bindgen::JsValue {
    fn from(obj: CaretChangedReason) -> wasm_bindgen::JsValue {
        match obj {
            CaretChangedReason::Visibilitychange => {
                wasm_bindgen::JsValue::from_str("visibilitychange")
            }
            CaretChangedReason::Updateposition => wasm_bindgen::JsValue::from_str("updateposition"),
            CaretChangedReason::Longpressonemptycontent => {
                wasm_bindgen::JsValue::from_str("longpressonemptycontent")
            }
            CaretChangedReason::Taponcaret => wasm_bindgen::JsValue::from_str("taponcaret"),
            CaretChangedReason::Presscaret => wasm_bindgen::JsValue::from_str("presscaret"),
            CaretChangedReason::Releasecaret => wasm_bindgen::JsValue::from_str("releasecaret"),
            CaretChangedReason::Scroll => wasm_bindgen::JsValue::from_str("scroll"),
            CaretChangedReason::__Nonexhaustive => {
                panic!("attempted to convert invalid CaretChangedReason into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_884d3fd30e901950: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
