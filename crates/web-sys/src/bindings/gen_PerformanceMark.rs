use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PerformanceMark` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark)\n\n*This API requires the following crate features to be activated: `PerformanceMark`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PerformanceMark {
    obj: PerformanceEntry,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PerformanceMark: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PerformanceMark {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(80u32);
            inform(101u32);
            inform(114u32);
            inform(102u32);
            inform(111u32);
            inform(114u32);
            inform(109u32);
            inform(97u32);
            inform(110u32);
            inform(99u32);
            inform(101u32);
            inform(77u32);
            inform(97u32);
            inform(114u32);
            inform(107u32);
        }
    }
    impl core::ops::Deref for PerformanceMark {
        type Target = PerformanceEntry;
        #[inline]
        fn deref(&self) -> &PerformanceEntry {
            &self.obj
        }
    }
    impl IntoWasmAbi for PerformanceMark {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PerformanceMark {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PerformanceMark {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PerformanceMark {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PerformanceMark {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PerformanceMark {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PerformanceMark {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PerformanceMark {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PerformanceMark>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PerformanceMark {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PerformanceMark {
        #[inline]
        fn from(obj: JsValue) -> PerformanceMark {
            PerformanceMark { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PerformanceMark {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PerformanceMark> for PerformanceMark {
        #[inline]
        fn as_ref(&self) -> &PerformanceMark {
            self
        }
    }
    impl From<PerformanceMark> for JsValue {
        #[inline]
        fn from(obj: PerformanceMark) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PerformanceMark {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PerformanceMark(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PerformanceMark(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PerformanceMark(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PerformanceMark { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PerformanceMark) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PerformanceMark> for PerformanceEntry {
    #[inline]
    fn from(obj: PerformanceMark) -> PerformanceEntry {
        use wasm_bindgen::JsCast;
        PerformanceEntry::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<PerformanceEntry> for PerformanceMark {
    #[inline]
    fn as_ref(&self) -> &PerformanceEntry {
        use wasm_bindgen::JsCast;
        PerformanceEntry::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PerformanceMark> for ::js_sys::Object {
    #[inline]
    fn from(obj: PerformanceMark) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PerformanceMark {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_bba5eb9cc7ef09ba: [u8; 159usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}]\0\0\0\0\0\x01\0\0\x02\x0FPerformanceMark!__widl_instanceof_PerformanceMark\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
