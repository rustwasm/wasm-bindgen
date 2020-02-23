use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ReferrerPolicy {
    None = 0,
    NoReferrer = 1,
    NoReferrerWhenDowngrade = 2,
    Origin = 3,
    OriginWhenCrossOrigin = 4,
    UnsafeUrl = 5,
    SameOrigin = 6,
    StrictOrigin = 7,
    StrictOriginWhenCrossOrigin = 8,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ReferrerPolicy {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ReferrerPolicy> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "" => Some(ReferrerPolicy::None),
            "no-referrer" => Some(ReferrerPolicy::NoReferrer),
            "no-referrer-when-downgrade" => Some(ReferrerPolicy::NoReferrerWhenDowngrade),
            "origin" => Some(ReferrerPolicy::Origin),
            "origin-when-cross-origin" => Some(ReferrerPolicy::OriginWhenCrossOrigin),
            "unsafe-url" => Some(ReferrerPolicy::UnsafeUrl),
            "same-origin" => Some(ReferrerPolicy::SameOrigin),
            "strict-origin" => Some(ReferrerPolicy::StrictOrigin),
            "strict-origin-when-cross-origin" => Some(ReferrerPolicy::StrictOriginWhenCrossOrigin),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ReferrerPolicy {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ReferrerPolicy {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ReferrerPolicy {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ReferrerPolicy::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ReferrerPolicy::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ReferrerPolicy {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ReferrerPolicy {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ReferrerPolicy> for wasm_bindgen::JsValue {
    fn from(obj: ReferrerPolicy) -> wasm_bindgen::JsValue {
        match obj {
            ReferrerPolicy::None => wasm_bindgen::JsValue::from_str(""),
            ReferrerPolicy::NoReferrer => wasm_bindgen::JsValue::from_str("no-referrer"),
            ReferrerPolicy::NoReferrerWhenDowngrade => {
                wasm_bindgen::JsValue::from_str("no-referrer-when-downgrade")
            }
            ReferrerPolicy::Origin => wasm_bindgen::JsValue::from_str("origin"),
            ReferrerPolicy::OriginWhenCrossOrigin => {
                wasm_bindgen::JsValue::from_str("origin-when-cross-origin")
            }
            ReferrerPolicy::UnsafeUrl => wasm_bindgen::JsValue::from_str("unsafe-url"),
            ReferrerPolicy::SameOrigin => wasm_bindgen::JsValue::from_str("same-origin"),
            ReferrerPolicy::StrictOrigin => wasm_bindgen::JsValue::from_str("strict-origin"),
            ReferrerPolicy::StrictOriginWhenCrossOrigin => {
                wasm_bindgen::JsValue::from_str("strict-origin-when-cross-origin")
            }
            ReferrerPolicy::__Nonexhaustive => {
                panic!("attempted to convert invalid ReferrerPolicy into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_dde166f358dc2d5c: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
