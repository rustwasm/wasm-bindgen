use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `RTCOfferAnswerOptions` dictionary\n\n\n*This API requires the following crate features to be activated: `RtcOfferAnswerOptions`*"]
pub struct RtcOfferAnswerOptions {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl RtcOfferAnswerOptions {
    #[doc = "Construct a new `RTCOfferAnswerOptions`\n\n\n*This API requires the following crate features to be activated: `RtcOfferAnswerOptions`*"]
    pub fn new() -> RtcOfferAnswerOptions {
        let mut _ret = RtcOfferAnswerOptions {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
}
#[allow(bad_style)]
#[allow(clippy::all)]
const _CONST_RtcOfferAnswerOptions: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<RtcOfferAnswerOptions> for JsValue {
        #[inline]
        fn from(val: RtcOfferAnswerOptions) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for RtcOfferAnswerOptions {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for RtcOfferAnswerOptions {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for RtcOfferAnswerOptions {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcOfferAnswerOptions {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for RtcOfferAnswerOptions {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            RtcOfferAnswerOptions {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for RtcOfferAnswerOptions {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcOfferAnswerOptions {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for RtcOfferAnswerOptions {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for RtcOfferAnswerOptions {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<RtcOfferAnswerOptions>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(RtcOfferAnswerOptions {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for RtcOfferAnswerOptions {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcOfferAnswerOptions {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcOfferAnswerOptions) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_37c24822c76a9509: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
