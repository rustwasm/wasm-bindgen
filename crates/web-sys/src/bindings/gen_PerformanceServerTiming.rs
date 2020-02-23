use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PerformanceServerTiming` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming)\n\n*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PerformanceServerTiming {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PerformanceServerTiming: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PerformanceServerTiming {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(23u32);
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
            inform(83u32);
            inform(101u32);
            inform(114u32);
            inform(118u32);
            inform(101u32);
            inform(114u32);
            inform(84u32);
            inform(105u32);
            inform(109u32);
            inform(105u32);
            inform(110u32);
            inform(103u32);
        }
    }
    impl core::ops::Deref for PerformanceServerTiming {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PerformanceServerTiming {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PerformanceServerTiming {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PerformanceServerTiming {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PerformanceServerTiming {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PerformanceServerTiming {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PerformanceServerTiming {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PerformanceServerTiming {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PerformanceServerTiming {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PerformanceServerTiming>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PerformanceServerTiming {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PerformanceServerTiming {
        #[inline]
        fn from(obj: JsValue) -> PerformanceServerTiming {
            PerformanceServerTiming { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PerformanceServerTiming {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PerformanceServerTiming> for PerformanceServerTiming {
        #[inline]
        fn as_ref(&self) -> &PerformanceServerTiming {
            self
        }
    }
    impl From<PerformanceServerTiming> for JsValue {
        #[inline]
        fn from(obj: PerformanceServerTiming) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PerformanceServerTiming {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PerformanceServerTiming(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PerformanceServerTiming(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PerformanceServerTiming(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PerformanceServerTiming { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PerformanceServerTiming) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PerformanceServerTiming> for ::js_sys::Object {
    #[inline]
    fn from(obj: PerformanceServerTiming) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PerformanceServerTiming {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PerformanceServerTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_PerformanceServerTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceServerTiming as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl PerformanceServerTiming {
    #[cfg(all(feature = "PerformanceServerTiming",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/toJSON)\n\n*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "PerformanceServerTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_PerformanceServerTiming(
                self_: <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_PerformanceServerTiming(
            self_: <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_to_json_PerformanceServerTiming(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceServerTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_PerformanceServerTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceServerTiming as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PerformanceServerTiming {
    #[cfg(all(feature = "PerformanceServerTiming",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/name)\n\n*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "PerformanceServerTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_PerformanceServerTiming(
                self_: <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_PerformanceServerTiming(
            self_: <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_name_PerformanceServerTiming(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceServerTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_duration_PerformanceServerTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceServerTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceServerTiming {
    #[cfg(all(feature = "PerformanceServerTiming",))]
    #[allow(bad_style)]
    #[doc = "The `duration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/duration)\n\n*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    #[allow(clippy::all)]
    pub fn duration(&self) -> f64 {
        #[cfg(all(feature = "PerformanceServerTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_duration_PerformanceServerTiming(
                self_: <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_duration_PerformanceServerTiming(
            self_: <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_duration_PerformanceServerTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceServerTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_description_PerformanceServerTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceServerTiming as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PerformanceServerTiming {
    #[cfg(all(feature = "PerformanceServerTiming",))]
    #[allow(bad_style)]
    #[doc = "The `description` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/description)\n\n*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    #[allow(clippy::all)]
    pub fn description(&self) -> String {
        #[cfg(all(feature = "PerformanceServerTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_description_PerformanceServerTiming(
                self_: <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_description_PerformanceServerTiming(
            self_: <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceServerTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_description_PerformanceServerTiming(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_557f92fddf94e1ed: [u8; 568usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xF6\x01\0\0\0\0\x05\0\0\x02\x17PerformanceServerTiming)__widl_instanceof_PerformanceServerTiming\0\0\0\0(__widl_f_to_json_PerformanceServerTiming\0\0\0\x01\x17PerformanceServerTiming\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0%__widl_f_name_PerformanceServerTiming\0\0\0\x01\x17PerformanceServerTiming\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0)__widl_f_duration_PerformanceServerTiming\0\0\0\x01\x17PerformanceServerTiming\x01\0\x01\x08duration\x01\x01\x05self_\x08duration\0\0\0,__widl_f_description_PerformanceServerTiming\0\0\0\x01\x17PerformanceServerTiming\x01\0\x01\x0Bdescription\x01\x01\x05self_\x0Bdescription\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
