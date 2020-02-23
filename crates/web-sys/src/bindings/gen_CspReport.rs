use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `CSPReport` dictionary\n\n\n*This API requires the following crate features to be activated: `CspReport`*"]
pub struct CspReport {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl CspReport {
    #[doc = "Construct a new `CSPReport`\n\n\n*This API requires the following crate features to be activated: `CspReport`, `CspReportProperties`*"]
    pub fn new() -> CspReport {
        let mut _ret = CspReport {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[cfg(all(feature = "CspReportProperties",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `csp-report` field of this object\n\n\n*This API requires the following crate features to be activated: `CspReport`, `CspReportProperties`*"]
    pub fn csp_report(&mut self, val: &CspReportProperties) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("csp-report"),
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
const _CONST_CspReport: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<CspReport> for JsValue {
        #[inline]
        fn from(val: CspReport) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for CspReport {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for CspReport {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for CspReport {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a CspReport {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for CspReport {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            CspReport {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for CspReport {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CspReport {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for CspReport {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for CspReport {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<CspReport>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(CspReport {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for CspReport {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CspReport {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CspReport) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8b71c2f03af9c6d7: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
