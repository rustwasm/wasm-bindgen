use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FuzzingFunctions` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions)\n\n*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FuzzingFunctions {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FuzzingFunctions: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FuzzingFunctions {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(70u32);
            inform(117u32);
            inform(122u32);
            inform(122u32);
            inform(105u32);
            inform(110u32);
            inform(103u32);
            inform(70u32);
            inform(117u32);
            inform(110u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for FuzzingFunctions {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for FuzzingFunctions {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FuzzingFunctions {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FuzzingFunctions {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FuzzingFunctions {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FuzzingFunctions {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FuzzingFunctions {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FuzzingFunctions {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FuzzingFunctions {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FuzzingFunctions>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FuzzingFunctions {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FuzzingFunctions {
        #[inline]
        fn from(obj: JsValue) -> FuzzingFunctions {
            FuzzingFunctions { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FuzzingFunctions {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FuzzingFunctions> for FuzzingFunctions {
        #[inline]
        fn as_ref(&self) -> &FuzzingFunctions {
            self
        }
    }
    impl From<FuzzingFunctions> for JsValue {
        #[inline]
        fn from(obj: FuzzingFunctions) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FuzzingFunctions {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FuzzingFunctions(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FuzzingFunctions(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FuzzingFunctions(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FuzzingFunctions { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FuzzingFunctions) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FuzzingFunctions> for ::js_sys::Object {
    #[inline]
    fn from(obj: FuzzingFunctions) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FuzzingFunctions {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FuzzingFunctions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cycle_collect_FuzzingFunctions() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <() as WasmDescribe>::describe();
}
impl FuzzingFunctions {
    #[cfg(all(feature = "FuzzingFunctions",))]
    #[allow(bad_style)]
    #[doc = "The `cycleCollect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/cycleCollect)\n\n*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
    #[allow(clippy::all)]
    pub fn cycle_collect() {
        #[cfg(all(feature = "FuzzingFunctions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cycle_collect_FuzzingFunctions() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cycle_collect_FuzzingFunctions() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_cycle_collect_FuzzingFunctions() };
            ()
        }
    }
}
#[cfg(all(feature = "FuzzingFunctions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_enable_accessibility_FuzzingFunctions() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <() as WasmDescribe>::describe();
}
impl FuzzingFunctions {
    #[cfg(all(feature = "FuzzingFunctions",))]
    #[allow(bad_style)]
    #[doc = "The `enableAccessibility()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/enableAccessibility)\n\n*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
    #[allow(clippy::all)]
    pub fn enable_accessibility() -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FuzzingFunctions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_enable_accessibility_FuzzingFunctions() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_enable_accessibility_FuzzingFunctions() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_enable_accessibility_FuzzingFunctions() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "FuzzingFunctions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_garbage_collect_FuzzingFunctions() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <() as WasmDescribe>::describe();
}
impl FuzzingFunctions {
    #[cfg(all(feature = "FuzzingFunctions",))]
    #[allow(bad_style)]
    #[doc = "The `garbageCollect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/garbageCollect)\n\n*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
    #[allow(clippy::all)]
    pub fn garbage_collect() {
        #[cfg(all(feature = "FuzzingFunctions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_garbage_collect_FuzzingFunctions() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_garbage_collect_FuzzingFunctions() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_garbage_collect_FuzzingFunctions() };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_65ecfaef6053b968: [u8; 425usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}g\x01\0\0\0\0\x04\0\0\x02\x10FuzzingFunctions\"__widl_instanceof_FuzzingFunctions\0\0\0\0'__widl_f_cycle_collect_FuzzingFunctions\0\0\0\x01\x10FuzzingFunctions\x01\x01\0\x01\0\x0CcycleCollect\0\0\0.__widl_f_enable_accessibility_FuzzingFunctions\x01\0\0\x01\x10FuzzingFunctions\x01\x01\0\x01\0\x13enableAccessibility\0\0\0)__widl_f_garbage_collect_FuzzingFunctions\0\0\0\x01\x10FuzzingFunctions\x01\x01\0\x01\0\x0EgarbageCollect\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
