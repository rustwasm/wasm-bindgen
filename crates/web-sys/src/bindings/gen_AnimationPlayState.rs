use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum AnimationPlayState {
    Idle = 0,
    Running = 1,
    Paused = 2,
    Finished = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl AnimationPlayState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<AnimationPlayState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "idle" => Some(AnimationPlayState::Idle),
            "running" => Some(AnimationPlayState::Running),
            "paused" => Some(AnimationPlayState::Paused),
            "finished" => Some(AnimationPlayState::Finished),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for AnimationPlayState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for AnimationPlayState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for AnimationPlayState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        AnimationPlayState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(AnimationPlayState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for AnimationPlayState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for AnimationPlayState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<AnimationPlayState> for wasm_bindgen::JsValue {
    fn from(obj: AnimationPlayState) -> wasm_bindgen::JsValue {
        match obj {
            AnimationPlayState::Idle => wasm_bindgen::JsValue::from_str("idle"),
            AnimationPlayState::Running => wasm_bindgen::JsValue::from_str("running"),
            AnimationPlayState::Paused => wasm_bindgen::JsValue::from_str("paused"),
            AnimationPlayState::Finished => wasm_bindgen::JsValue::from_str("finished"),
            AnimationPlayState::__Nonexhaustive => {
                panic!("attempted to convert invalid AnimationPlayState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1f819ba7048396ca: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
