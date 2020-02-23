use super::*;
use js_sys::Object;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `WebrtcGlobalStatisticsReport` dictionary\n\n\n*This API requires the following crate features to be activated: `WebrtcGlobalStatisticsReport`*"]
pub struct WebrtcGlobalStatisticsReport {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl WebrtcGlobalStatisticsReport {
    #[doc = "Construct a new `WebrtcGlobalStatisticsReport`\n\n\n*This API requires the following crate features to be activated: `WebrtcGlobalStatisticsReport`*"]
    pub fn new() -> WebrtcGlobalStatisticsReport {
        let mut _ret = WebrtcGlobalStatisticsReport {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `reports` field of this object\n\n\n*This API requires the following crate features to be activated: `WebrtcGlobalStatisticsReport`*"]
    pub fn reports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("reports"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
#[allow(bad_style)]
#[allow(clippy::all)]
const _CONST_WebrtcGlobalStatisticsReport: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<WebrtcGlobalStatisticsReport> for JsValue {
        #[inline]
        fn from(val: WebrtcGlobalStatisticsReport) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for WebrtcGlobalStatisticsReport {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for WebrtcGlobalStatisticsReport {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for WebrtcGlobalStatisticsReport {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a WebrtcGlobalStatisticsReport {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for WebrtcGlobalStatisticsReport {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            WebrtcGlobalStatisticsReport {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for WebrtcGlobalStatisticsReport {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebrtcGlobalStatisticsReport {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for WebrtcGlobalStatisticsReport {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for WebrtcGlobalStatisticsReport {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<WebrtcGlobalStatisticsReport>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(WebrtcGlobalStatisticsReport {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for WebrtcGlobalStatisticsReport {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebrtcGlobalStatisticsReport {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebrtcGlobalStatisticsReport) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_739a21710f287aa6: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
