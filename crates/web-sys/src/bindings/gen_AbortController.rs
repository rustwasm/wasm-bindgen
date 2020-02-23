use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AbortController` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController)\n\n*This API requires the following crate features to be activated: `AbortController`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AbortController {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AbortController: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AbortController {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(65u32);
            inform(98u32);
            inform(111u32);
            inform(114u32);
            inform(116u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(114u32);
            inform(111u32);
            inform(108u32);
            inform(108u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for AbortController {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for AbortController {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AbortController {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AbortController {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AbortController {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AbortController {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AbortController {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AbortController {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AbortController {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AbortController>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AbortController {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AbortController {
        #[inline]
        fn from(obj: JsValue) -> AbortController {
            AbortController { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AbortController {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AbortController> for AbortController {
        #[inline]
        fn as_ref(&self) -> &AbortController {
            self
        }
    }
    impl From<AbortController> for JsValue {
        #[inline]
        fn from(obj: AbortController) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AbortController {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AbortController(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AbortController(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AbortController(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AbortController { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AbortController) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AbortController> for ::js_sys::Object {
    #[inline]
    fn from(obj: AbortController) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AbortController {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AbortController",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_AbortController() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <AbortController as WasmDescribe>::describe();
}
impl AbortController {
    #[cfg(all(feature = "AbortController",))]
    #[allow(bad_style)]
    #[doc = "The `new AbortController(..)` constructor, creating a new instance of `AbortController`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/AbortController)\n\n*This API requires the following crate features to be activated: `AbortController`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<AbortController, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AbortController",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_AbortController(
            ) -> <AbortController as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_AbortController(
        ) -> <AbortController as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_AbortController() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AbortController as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AbortController",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_abort_AbortController() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AbortController as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AbortController {
    #[cfg(all(feature = "AbortController",))]
    #[allow(bad_style)]
    #[doc = "The `abort()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/abort)\n\n*This API requires the following crate features to be activated: `AbortController`*"]
    #[allow(clippy::all)]
    pub fn abort(&self) {
        #[cfg(all(feature = "AbortController",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_abort_AbortController(
                self_: <&AbortController as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_abort_AbortController(
            self_: <&AbortController as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AbortController as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_abort_AbortController(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AbortController", feature = "AbortSignal",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_signal_AbortController() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AbortController as WasmDescribe>::describe();
    <AbortSignal as WasmDescribe>::describe();
}
impl AbortController {
    #[cfg(all(feature = "AbortController", feature = "AbortSignal",))]
    #[allow(bad_style)]
    #[doc = "The `signal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/signal)\n\n*This API requires the following crate features to be activated: `AbortController`, `AbortSignal`*"]
    #[allow(clippy::all)]
    pub fn signal(&self) -> AbortSignal {
        #[cfg(all(feature = "AbortController", feature = "AbortSignal",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_signal_AbortController(
                self_: <&AbortController as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AbortSignal as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_signal_AbortController(
            self_: <&AbortController as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AbortSignal as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AbortController as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_signal_AbortController(self_)
            };
            <AbortSignal as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4e3b723f6996fa55: [u8; 369usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}/\x01\0\0\0\0\x04\0\0\x02\x0FAbortController!__widl_instanceof_AbortController\0\0\0\0\x1C__widl_f_new_AbortController\x01\0\0\x01\x0FAbortController\0\x01\0\x03new\0\0\0\x1E__widl_f_abort_AbortController\0\0\0\x01\x0FAbortController\x01\0\0\x01\x01\x05self_\x05abort\0\0\0\x1F__widl_f_signal_AbortController\0\0\0\x01\x0FAbortController\x01\0\x01\x06signal\x01\x01\x05self_\x06signal\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
