use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PeriodicWave` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicWave)\n\n*This API requires the following crate features to be activated: `PeriodicWave`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PeriodicWave {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PeriodicWave: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PeriodicWave {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(80u32);
            inform(101u32);
            inform(114u32);
            inform(105u32);
            inform(111u32);
            inform(100u32);
            inform(105u32);
            inform(99u32);
            inform(87u32);
            inform(97u32);
            inform(118u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for PeriodicWave {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PeriodicWave {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PeriodicWave {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PeriodicWave {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PeriodicWave {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PeriodicWave {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PeriodicWave {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PeriodicWave {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PeriodicWave {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PeriodicWave>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PeriodicWave {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PeriodicWave {
        #[inline]
        fn from(obj: JsValue) -> PeriodicWave {
            PeriodicWave { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PeriodicWave {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PeriodicWave> for PeriodicWave {
        #[inline]
        fn as_ref(&self) -> &PeriodicWave {
            self
        }
    }
    impl From<PeriodicWave> for JsValue {
        #[inline]
        fn from(obj: PeriodicWave) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PeriodicWave {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PeriodicWave(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PeriodicWave(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PeriodicWave(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PeriodicWave { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PeriodicWave) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PeriodicWave> for ::js_sys::Object {
    #[inline]
    fn from(obj: PeriodicWave) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PeriodicWave {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BaseAudioContext", feature = "PeriodicWave",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_PeriodicWave() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <PeriodicWave as WasmDescribe>::describe();
}
impl PeriodicWave {
    #[cfg(all(feature = "BaseAudioContext", feature = "PeriodicWave",))]
    #[allow(bad_style)]
    #[doc = "The `new PeriodicWave(..)` constructor, creating a new instance of `PeriodicWave`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicWave/PeriodicWave)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`*"]
    #[allow(clippy::all)]
    pub fn new(context: &BaseAudioContext) -> Result<PeriodicWave, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BaseAudioContext", feature = "PeriodicWave",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_PeriodicWave(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_PeriodicWave(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context =
                    <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context);
                __widl_f_new_PeriodicWave(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "BaseAudioContext",
    feature = "PeriodicWave",
    feature = "PeriodicWaveOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_PeriodicWave() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&PeriodicWaveOptions as WasmDescribe>::describe();
    <PeriodicWave as WasmDescribe>::describe();
}
impl PeriodicWave {
    #[cfg(all(
        feature = "BaseAudioContext",
        feature = "PeriodicWave",
        feature = "PeriodicWaveOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new PeriodicWave(..)` constructor, creating a new instance of `PeriodicWave`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicWave/PeriodicWave)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`, `PeriodicWaveOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &PeriodicWaveOptions,
    ) -> Result<PeriodicWave, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "BaseAudioContext",
            feature = "PeriodicWave",
            feature = "PeriodicWaveOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_PeriodicWave(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&PeriodicWaveOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_PeriodicWave(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&PeriodicWaveOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context =
                    <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context);
                let options =
                    <&PeriodicWaveOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_PeriodicWave(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2a9a9948b86e6dd8: [u8; 296usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE6\0\0\0\0\0\x03\0\0\x02\x0CPeriodicWave\x1E__widl_instanceof_PeriodicWave\0\0\0\0\x19__widl_f_new_PeriodicWave\x01\0\0\x01\x0CPeriodicWave\0\x01\x01\x07context\x03new\0\0\0&__widl_f_new_with_options_PeriodicWave\x01\0\0\x01\x0CPeriodicWave\0\x01\x02\x07context\x07options\x03new\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
