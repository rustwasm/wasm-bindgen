use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PerformanceNavigation` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation)\n\n*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PerformanceNavigation {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PerformanceNavigation: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PerformanceNavigation {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
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
            inform(78u32);
            inform(97u32);
            inform(118u32);
            inform(105u32);
            inform(103u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for PerformanceNavigation {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PerformanceNavigation {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PerformanceNavigation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PerformanceNavigation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PerformanceNavigation {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PerformanceNavigation {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PerformanceNavigation {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PerformanceNavigation {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PerformanceNavigation {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PerformanceNavigation>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PerformanceNavigation {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PerformanceNavigation {
        #[inline]
        fn from(obj: JsValue) -> PerformanceNavigation {
            PerformanceNavigation { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PerformanceNavigation {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PerformanceNavigation> for PerformanceNavigation {
        #[inline]
        fn as_ref(&self) -> &PerformanceNavigation {
            self
        }
    }
    impl From<PerformanceNavigation> for JsValue {
        #[inline]
        fn from(obj: PerformanceNavigation) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PerformanceNavigation {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PerformanceNavigation(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PerformanceNavigation(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PerformanceNavigation(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PerformanceNavigation { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PerformanceNavigation) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PerformanceNavigation> for ::js_sys::Object {
    #[inline]
    fn from(obj: PerformanceNavigation) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PerformanceNavigation {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PerformanceNavigation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_PerformanceNavigation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigation as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl PerformanceNavigation {
    #[cfg(all(feature = "PerformanceNavigation",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/toJSON)\n\n*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "PerformanceNavigation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_PerformanceNavigation(
                self_: <&PerformanceNavigation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_PerformanceNavigation(
            self_: <&PerformanceNavigation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceNavigation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_PerformanceNavigation(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceNavigation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_PerformanceNavigation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigation as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl PerformanceNavigation {
    #[cfg(all(feature = "PerformanceNavigation",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/type)\n\n*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> u16 {
        #[cfg(all(feature = "PerformanceNavigation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_PerformanceNavigation(
                self_: <&PerformanceNavigation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_PerformanceNavigation(
            self_: <&PerformanceNavigation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceNavigation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_PerformanceNavigation(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceNavigation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_redirect_count_PerformanceNavigation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigation as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl PerformanceNavigation {
    #[cfg(all(feature = "PerformanceNavigation",))]
    #[allow(bad_style)]
    #[doc = "The `redirectCount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/redirectCount)\n\n*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    #[allow(clippy::all)]
    pub fn redirect_count(&self) -> u16 {
        #[cfg(all(feature = "PerformanceNavigation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_redirect_count_PerformanceNavigation(
                self_: <&PerformanceNavigation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_redirect_count_PerformanceNavigation(
            self_: <&PerformanceNavigation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceNavigation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_redirect_count_PerformanceNavigation(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl PerformanceNavigation {
    pub const TYPE_NAVIGATE: u16 = 0i64 as u16;
}
impl PerformanceNavigation {
    pub const TYPE_RELOAD: u16 = 1u64 as u16;
}
impl PerformanceNavigation {
    pub const TYPE_BACK_FORWARD: u16 = 2u64 as u16;
}
impl PerformanceNavigation {
    pub const TYPE_RESERVED: u16 = 255u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a6429143542dbc64: [u8; 457usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x87\x01\0\0\0\0\x04\0\0\x02\x15PerformanceNavigation'__widl_instanceof_PerformanceNavigation\0\0\0\0&__widl_f_to_json_PerformanceNavigation\0\0\0\x01\x15PerformanceNavigation\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0#__widl_f_type_PerformanceNavigation\0\0\0\x01\x15PerformanceNavigation\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0-__widl_f_redirect_count_PerformanceNavigation\0\0\0\x01\x15PerformanceNavigation\x01\0\x01\rredirectCount\x01\x01\x05self_\rredirectCount\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
