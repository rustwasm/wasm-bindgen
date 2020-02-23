use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SpeechSynthesisVoice` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SpeechSynthesisVoice {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SpeechSynthesisVoice: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SpeechSynthesisVoice {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(83u32);
            inform(112u32);
            inform(101u32);
            inform(101u32);
            inform(99u32);
            inform(104u32);
            inform(83u32);
            inform(121u32);
            inform(110u32);
            inform(116u32);
            inform(104u32);
            inform(101u32);
            inform(115u32);
            inform(105u32);
            inform(115u32);
            inform(86u32);
            inform(111u32);
            inform(105u32);
            inform(99u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for SpeechSynthesisVoice {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SpeechSynthesisVoice {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SpeechSynthesisVoice {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SpeechSynthesisVoice {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SpeechSynthesisVoice {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SpeechSynthesisVoice {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SpeechSynthesisVoice {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SpeechSynthesisVoice {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SpeechSynthesisVoice {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SpeechSynthesisVoice>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SpeechSynthesisVoice {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SpeechSynthesisVoice {
        #[inline]
        fn from(obj: JsValue) -> SpeechSynthesisVoice {
            SpeechSynthesisVoice { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SpeechSynthesisVoice {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SpeechSynthesisVoice> for SpeechSynthesisVoice {
        #[inline]
        fn as_ref(&self) -> &SpeechSynthesisVoice {
            self
        }
    }
    impl From<SpeechSynthesisVoice> for JsValue {
        #[inline]
        fn from(obj: SpeechSynthesisVoice) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SpeechSynthesisVoice {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SpeechSynthesisVoice(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SpeechSynthesisVoice(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SpeechSynthesisVoice(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SpeechSynthesisVoice { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SpeechSynthesisVoice) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SpeechSynthesisVoice> for ::js_sys::Object {
    #[inline]
    fn from(obj: SpeechSynthesisVoice) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SpeechSynthesisVoice {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SpeechSynthesisVoice",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_voice_uri_SpeechSynthesisVoice() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisVoice as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SpeechSynthesisVoice {
    #[cfg(all(feature = "SpeechSynthesisVoice",))]
    #[allow(bad_style)]
    #[doc = "The `voiceURI` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/voiceURI)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    #[allow(clippy::all)]
    pub fn voice_uri(&self) -> String {
        #[cfg(all(feature = "SpeechSynthesisVoice",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_voice_uri_SpeechSynthesisVoice(
                self_: <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_voice_uri_SpeechSynthesisVoice(
            self_: <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_voice_uri_SpeechSynthesisVoice(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisVoice",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_SpeechSynthesisVoice() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisVoice as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SpeechSynthesisVoice {
    #[cfg(all(feature = "SpeechSynthesisVoice",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/name)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "SpeechSynthesisVoice",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_SpeechSynthesisVoice(
                self_: <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_SpeechSynthesisVoice(
            self_: <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_SpeechSynthesisVoice(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisVoice",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lang_SpeechSynthesisVoice() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisVoice as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SpeechSynthesisVoice {
    #[cfg(all(feature = "SpeechSynthesisVoice",))]
    #[allow(bad_style)]
    #[doc = "The `lang` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/lang)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    #[allow(clippy::all)]
    pub fn lang(&self) -> String {
        #[cfg(all(feature = "SpeechSynthesisVoice",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lang_SpeechSynthesisVoice(
                self_: <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lang_SpeechSynthesisVoice(
            self_: <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_lang_SpeechSynthesisVoice(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisVoice",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_local_service_SpeechSynthesisVoice() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisVoice as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SpeechSynthesisVoice {
    #[cfg(all(feature = "SpeechSynthesisVoice",))]
    #[allow(bad_style)]
    #[doc = "The `localService` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/localService)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    #[allow(clippy::all)]
    pub fn local_service(&self) -> bool {
        #[cfg(all(feature = "SpeechSynthesisVoice",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_local_service_SpeechSynthesisVoice(
                self_: <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_local_service_SpeechSynthesisVoice(
            self_: <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_local_service_SpeechSynthesisVoice(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisVoice",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_SpeechSynthesisVoice() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisVoice as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SpeechSynthesisVoice {
    #[cfg(all(feature = "SpeechSynthesisVoice",))]
    #[allow(bad_style)]
    #[doc = "The `default` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/default)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    #[allow(clippy::all)]
    pub fn default(&self) -> bool {
        #[cfg(all(feature = "SpeechSynthesisVoice",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_SpeechSynthesisVoice(
                self_: <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_SpeechSynthesisVoice(
            self_: <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisVoice as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_SpeechSynthesisVoice(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d79f5b8494ecb503: [u8; 636usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}:\x02\0\0\0\0\x06\0\0\x02\x14SpeechSynthesisVoice&__widl_instanceof_SpeechSynthesisVoice\0\0\0\0'__widl_f_voice_uri_SpeechSynthesisVoice\0\0\0\x01\x14SpeechSynthesisVoice\x01\0\x01\x08voiceURI\x01\x01\x05self_\x08voiceURI\0\0\0\"__widl_f_name_SpeechSynthesisVoice\0\0\0\x01\x14SpeechSynthesisVoice\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\"__widl_f_lang_SpeechSynthesisVoice\0\0\0\x01\x14SpeechSynthesisVoice\x01\0\x01\x04lang\x01\x01\x05self_\x04lang\0\0\0+__widl_f_local_service_SpeechSynthesisVoice\0\0\0\x01\x14SpeechSynthesisVoice\x01\0\x01\x0ClocalService\x01\x01\x05self_\x0ClocalService\0\0\0%__widl_f_default_SpeechSynthesisVoice\0\0\0\x01\x14SpeechSynthesisVoice\x01\0\x01\x07default\x01\x01\x05self_\x07default\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
