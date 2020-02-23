use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PerformanceMeasure` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMeasure)\n\n*This API requires the following crate features to be activated: `PerformanceMeasure`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PerformanceMeasure {
    obj: PerformanceEntry,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PerformanceMeasure: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PerformanceMeasure {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
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
            inform(101u32);
            inform(97u32);
            inform(115u32);
            inform(117u32);
            inform(114u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for PerformanceMeasure {
        type Target = PerformanceEntry;
        #[inline]
        fn deref(&self) -> &PerformanceEntry {
            &self.obj
        }
    }
    impl IntoWasmAbi for PerformanceMeasure {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PerformanceMeasure {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PerformanceMeasure {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PerformanceMeasure {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PerformanceMeasure {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PerformanceMeasure {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PerformanceMeasure {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PerformanceMeasure {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PerformanceMeasure>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PerformanceMeasure {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PerformanceMeasure {
        #[inline]
        fn from(obj: JsValue) -> PerformanceMeasure {
            PerformanceMeasure { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PerformanceMeasure {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PerformanceMeasure> for PerformanceMeasure {
        #[inline]
        fn as_ref(&self) -> &PerformanceMeasure {
            self
        }
    }
    impl From<PerformanceMeasure> for JsValue {
        #[inline]
        fn from(obj: PerformanceMeasure) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PerformanceMeasure {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PerformanceMeasure(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PerformanceMeasure(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PerformanceMeasure(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PerformanceMeasure { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PerformanceMeasure) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PerformanceMeasure> for PerformanceEntry {
    #[inline]
    fn from(obj: PerformanceMeasure) -> PerformanceEntry {
        use wasm_bindgen::JsCast;
        PerformanceEntry::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<PerformanceEntry> for PerformanceMeasure {
    #[inline]
    fn as_ref(&self) -> &PerformanceEntry {
        use wasm_bindgen::JsCast;
        PerformanceEntry::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PerformanceMeasure> for ::js_sys::Object {
    #[inline]
    fn from(obj: PerformanceMeasure) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PerformanceMeasure {
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
pub static __WASM_BINDGEN_GENERATED_2b377b0cd18f7774: [u8; 165usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}c\0\0\0\0\0\x01\0\0\x02\x12PerformanceMeasure$__widl_instanceof_PerformanceMeasure\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
