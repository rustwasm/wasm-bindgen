use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ScreenLuminance` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance)\n\n*This API requires the following crate features to be activated: `ScreenLuminance`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ScreenLuminance {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ScreenLuminance: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ScreenLuminance {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(83u32);
            inform(99u32);
            inform(114u32);
            inform(101u32);
            inform(101u32);
            inform(110u32);
            inform(76u32);
            inform(117u32);
            inform(109u32);
            inform(105u32);
            inform(110u32);
            inform(97u32);
            inform(110u32);
            inform(99u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for ScreenLuminance {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ScreenLuminance {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ScreenLuminance {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ScreenLuminance {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ScreenLuminance {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ScreenLuminance {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ScreenLuminance {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ScreenLuminance {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ScreenLuminance {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ScreenLuminance>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ScreenLuminance {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ScreenLuminance {
        #[inline]
        fn from(obj: JsValue) -> ScreenLuminance {
            ScreenLuminance { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ScreenLuminance {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ScreenLuminance> for ScreenLuminance {
        #[inline]
        fn as_ref(&self) -> &ScreenLuminance {
            self
        }
    }
    impl From<ScreenLuminance> for JsValue {
        #[inline]
        fn from(obj: ScreenLuminance) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ScreenLuminance {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ScreenLuminance(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ScreenLuminance(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ScreenLuminance(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ScreenLuminance { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ScreenLuminance) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ScreenLuminance> for ::js_sys::Object {
    #[inline]
    fn from(obj: ScreenLuminance) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ScreenLuminance {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ScreenLuminance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_min_ScreenLuminance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScreenLuminance as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl ScreenLuminance {
    #[cfg(all(feature = "ScreenLuminance",))]
    #[allow(bad_style)]
    #[doc = "The `min` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/min)\n\n*This API requires the following crate features to be activated: `ScreenLuminance`*"]
    #[allow(clippy::all)]
    pub fn min(&self) -> f64 {
        #[cfg(all(feature = "ScreenLuminance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_min_ScreenLuminance(
                self_: <&ScreenLuminance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_min_ScreenLuminance(
            self_: <&ScreenLuminance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScreenLuminance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_min_ScreenLuminance(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ScreenLuminance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_ScreenLuminance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScreenLuminance as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl ScreenLuminance {
    #[cfg(all(feature = "ScreenLuminance",))]
    #[allow(bad_style)]
    #[doc = "The `max` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/max)\n\n*This API requires the following crate features to be activated: `ScreenLuminance`*"]
    #[allow(clippy::all)]
    pub fn max(&self) -> f64 {
        #[cfg(all(feature = "ScreenLuminance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_ScreenLuminance(
                self_: <&ScreenLuminance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_ScreenLuminance(
            self_: <&ScreenLuminance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScreenLuminance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_ScreenLuminance(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ScreenLuminance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_average_ScreenLuminance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScreenLuminance as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl ScreenLuminance {
    #[cfg(all(feature = "ScreenLuminance",))]
    #[allow(bad_style)]
    #[doc = "The `maxAverage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/maxAverage)\n\n*This API requires the following crate features to be activated: `ScreenLuminance`*"]
    #[allow(clippy::all)]
    pub fn max_average(&self) -> f64 {
        #[cfg(all(feature = "ScreenLuminance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_average_ScreenLuminance(
                self_: <&ScreenLuminance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_average_ScreenLuminance(
            self_: <&ScreenLuminance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScreenLuminance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_average_ScreenLuminance(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_228e78f22a97e1e2: [u8; 394usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}H\x01\0\0\0\0\x04\0\0\x02\x0FScreenLuminance!__widl_instanceof_ScreenLuminance\0\0\0\0\x1C__widl_f_min_ScreenLuminance\0\0\0\x01\x0FScreenLuminance\x01\0\x01\x03min\x01\x01\x05self_\x03min\0\0\0\x1C__widl_f_max_ScreenLuminance\0\0\0\x01\x0FScreenLuminance\x01\0\x01\x03max\x01\x01\x05self_\x03max\0\0\0$__widl_f_max_average_ScreenLuminance\0\0\0\x01\x0FScreenLuminance\x01\0\x01\nmaxAverage\x01\x01\x05self_\nmaxAverage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
