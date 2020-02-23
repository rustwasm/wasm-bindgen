use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SpeechGrammar` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SpeechGrammar {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SpeechGrammar: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SpeechGrammar {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(83u32);
            inform(112u32);
            inform(101u32);
            inform(101u32);
            inform(99u32);
            inform(104u32);
            inform(71u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
            inform(109u32);
            inform(97u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for SpeechGrammar {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SpeechGrammar {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SpeechGrammar {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SpeechGrammar {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SpeechGrammar {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SpeechGrammar {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SpeechGrammar {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SpeechGrammar {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SpeechGrammar {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SpeechGrammar>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SpeechGrammar {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SpeechGrammar {
        #[inline]
        fn from(obj: JsValue) -> SpeechGrammar {
            SpeechGrammar { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SpeechGrammar {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SpeechGrammar> for SpeechGrammar {
        #[inline]
        fn as_ref(&self) -> &SpeechGrammar {
            self
        }
    }
    impl From<SpeechGrammar> for JsValue {
        #[inline]
        fn from(obj: SpeechGrammar) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SpeechGrammar {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SpeechGrammar(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SpeechGrammar(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SpeechGrammar(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SpeechGrammar { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SpeechGrammar) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SpeechGrammar> for ::js_sys::Object {
    #[inline]
    fn from(obj: SpeechGrammar) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SpeechGrammar {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SpeechGrammar",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_SpeechGrammar() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <SpeechGrammar as WasmDescribe>::describe();
}
impl SpeechGrammar {
    #[cfg(all(feature = "SpeechGrammar",))]
    #[allow(bad_style)]
    #[doc = "The `new SpeechGrammar(..)` constructor, creating a new instance of `SpeechGrammar`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/SpeechGrammar)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<SpeechGrammar, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammar",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_SpeechGrammar(
            ) -> <SpeechGrammar as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_SpeechGrammar(
        ) -> <SpeechGrammar as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_SpeechGrammar() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SpeechGrammar as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SpeechGrammar",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_src_SpeechGrammar() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechGrammar as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SpeechGrammar {
    #[cfg(all(feature = "SpeechGrammar",))]
    #[allow(bad_style)]
    #[doc = "The `src` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    #[allow(clippy::all)]
    pub fn src(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammar",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_src_SpeechGrammar(
                self_: <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_src_SpeechGrammar(
            self_: <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_src_SpeechGrammar(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SpeechGrammar",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_src_SpeechGrammar() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechGrammar as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechGrammar {
    #[cfg(all(feature = "SpeechGrammar",))]
    #[allow(bad_style)]
    #[doc = "The `src` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    #[allow(clippy::all)]
    pub fn set_src(&self, src: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammar",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_src_SpeechGrammar(
                self_: <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_src_SpeechGrammar(
            self_: <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(src);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_set_src_SpeechGrammar(self_, src)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SpeechGrammar",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_weight_SpeechGrammar() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechGrammar as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SpeechGrammar {
    #[cfg(all(feature = "SpeechGrammar",))]
    #[allow(bad_style)]
    #[doc = "The `weight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    #[allow(clippy::all)]
    pub fn weight(&self) -> Result<f32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammar",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_weight_SpeechGrammar(
                self_: <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_weight_SpeechGrammar(
            self_: <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_weight_SpeechGrammar(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SpeechGrammar",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_weight_SpeechGrammar() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechGrammar as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechGrammar {
    #[cfg(all(feature = "SpeechGrammar",))]
    #[allow(bad_style)]
    #[doc = "The `weight` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    #[allow(clippy::all)]
    pub fn set_weight(&self, weight: f32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammar",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_weight_SpeechGrammar(
                self_: <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                weight: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_weight_SpeechGrammar(
            self_: <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            weight: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(weight);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SpeechGrammar as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let weight = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(weight);
                __widl_f_set_weight_SpeechGrammar(self_, weight)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_042756da7a816e86: [u8; 515usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC1\x01\0\0\0\0\x06\0\0\x02\rSpeechGrammar\x1F__widl_instanceof_SpeechGrammar\0\0\0\0\x1A__widl_f_new_SpeechGrammar\x01\0\0\x01\rSpeechGrammar\0\x01\0\x03new\0\0\0\x1A__widl_f_src_SpeechGrammar\x01\0\0\x01\rSpeechGrammar\x01\0\x01\x03src\x01\x01\x05self_\x03src\0\0\0\x1E__widl_f_set_src_SpeechGrammar\x01\0\0\x01\rSpeechGrammar\x01\0\x02\x03src\x01\x02\x05self_\x03src\x03src\0\0\0\x1D__widl_f_weight_SpeechGrammar\x01\0\0\x01\rSpeechGrammar\x01\0\x01\x06weight\x01\x01\x05self_\x06weight\0\0\0!__widl_f_set_weight_SpeechGrammar\x01\0\0\x01\rSpeechGrammar\x01\0\x02\x06weight\x01\x02\x05self_\x06weight\x06weight\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
