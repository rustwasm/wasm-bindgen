use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SpeechRecognitionAlternative` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SpeechRecognitionAlternative {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SpeechRecognitionAlternative: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SpeechRecognitionAlternative {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(28u32);
            inform(83u32);
            inform(112u32);
            inform(101u32);
            inform(101u32);
            inform(99u32);
            inform(104u32);
            inform(82u32);
            inform(101u32);
            inform(99u32);
            inform(111u32);
            inform(103u32);
            inform(110u32);
            inform(105u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(65u32);
            inform(108u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(110u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(118u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for SpeechRecognitionAlternative {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SpeechRecognitionAlternative {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SpeechRecognitionAlternative {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SpeechRecognitionAlternative {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SpeechRecognitionAlternative {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SpeechRecognitionAlternative {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SpeechRecognitionAlternative {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SpeechRecognitionAlternative {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SpeechRecognitionAlternative {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SpeechRecognitionAlternative>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SpeechRecognitionAlternative {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SpeechRecognitionAlternative {
        #[inline]
        fn from(obj: JsValue) -> SpeechRecognitionAlternative {
            SpeechRecognitionAlternative { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SpeechRecognitionAlternative {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SpeechRecognitionAlternative> for SpeechRecognitionAlternative {
        #[inline]
        fn as_ref(&self) -> &SpeechRecognitionAlternative {
            self
        }
    }
    impl From<SpeechRecognitionAlternative> for JsValue {
        #[inline]
        fn from(obj: SpeechRecognitionAlternative) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SpeechRecognitionAlternative {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SpeechRecognitionAlternative(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SpeechRecognitionAlternative(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SpeechRecognitionAlternative(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SpeechRecognitionAlternative { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SpeechRecognitionAlternative) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SpeechRecognitionAlternative> for ::js_sys::Object {
    #[inline]
    fn from(obj: SpeechRecognitionAlternative) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SpeechRecognitionAlternative {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SpeechRecognitionAlternative",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transcript_SpeechRecognitionAlternative() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognitionAlternative as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SpeechRecognitionAlternative {
    #[cfg(all(feature = "SpeechRecognitionAlternative",))]
    #[allow(bad_style)]
    #[doc = "The `transcript` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/transcript)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
    #[allow(clippy::all)]
    pub fn transcript(&self) -> String {
        #[cfg(all(feature = "SpeechRecognitionAlternative",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transcript_SpeechRecognitionAlternative(
                self_: <&SpeechRecognitionAlternative as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transcript_SpeechRecognitionAlternative(
            self_: <&SpeechRecognitionAlternative as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognitionAlternative as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_transcript_SpeechRecognitionAlternative(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognitionAlternative",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_confidence_SpeechRecognitionAlternative() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognitionAlternative as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SpeechRecognitionAlternative {
    #[cfg(all(feature = "SpeechRecognitionAlternative",))]
    #[allow(bad_style)]
    #[doc = "The `confidence` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/confidence)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
    #[allow(clippy::all)]
    pub fn confidence(&self) -> f32 {
        #[cfg(all(feature = "SpeechRecognitionAlternative",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_confidence_SpeechRecognitionAlternative(
                self_: <&SpeechRecognitionAlternative as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_confidence_SpeechRecognitionAlternative(
            self_: <&SpeechRecognitionAlternative as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognitionAlternative as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_confidence_SpeechRecognitionAlternative(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_babb3b55be22f53c: [u8; 421usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}c\x01\0\0\0\0\x03\0\0\x02\x1CSpeechRecognitionAlternative.__widl_instanceof_SpeechRecognitionAlternative\0\0\0\00__widl_f_transcript_SpeechRecognitionAlternative\0\0\0\x01\x1CSpeechRecognitionAlternative\x01\0\x01\ntranscript\x01\x01\x05self_\ntranscript\0\0\00__widl_f_confidence_SpeechRecognitionAlternative\0\0\0\x01\x1CSpeechRecognitionAlternative\x01\0\x01\nconfidence\x01\x01\x05self_\nconfidence\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
