use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ValidityState` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ValidityState {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ValidityState: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ValidityState {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(86u32);
            inform(97u32);
            inform(108u32);
            inform(105u32);
            inform(100u32);
            inform(105u32);
            inform(116u32);
            inform(121u32);
            inform(83u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for ValidityState {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ValidityState {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ValidityState {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ValidityState {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ValidityState {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ValidityState {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ValidityState {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ValidityState {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ValidityState {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ValidityState>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ValidityState {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ValidityState {
        #[inline]
        fn from(obj: JsValue) -> ValidityState {
            ValidityState { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ValidityState {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ValidityState> for ValidityState {
        #[inline]
        fn as_ref(&self) -> &ValidityState {
            self
        }
    }
    impl From<ValidityState> for JsValue {
        #[inline]
        fn from(obj: ValidityState) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ValidityState {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ValidityState(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ValidityState(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ValidityState(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ValidityState { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ValidityState) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ValidityState> for ::js_sys::Object {
    #[inline]
    fn from(obj: ValidityState) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ValidityState {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_missing_ValidityState() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ValidityState as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ValidityState {
    #[cfg(all(feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `valueMissing` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valueMissing)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn value_missing(&self) -> bool {
        #[cfg(all(feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_missing_ValidityState(
                self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_missing_ValidityState(
            self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_missing_ValidityState(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_mismatch_ValidityState() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ValidityState as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ValidityState {
    #[cfg(all(feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `typeMismatch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/typeMismatch)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn type_mismatch(&self) -> bool {
        #[cfg(all(feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_mismatch_ValidityState(
                self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_mismatch_ValidityState(
            self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_mismatch_ValidityState(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pattern_mismatch_ValidityState() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ValidityState as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ValidityState {
    #[cfg(all(feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `patternMismatch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/patternMismatch)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn pattern_mismatch(&self) -> bool {
        #[cfg(all(feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pattern_mismatch_ValidityState(
                self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pattern_mismatch_ValidityState(
            self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pattern_mismatch_ValidityState(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_too_long_ValidityState() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ValidityState as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ValidityState {
    #[cfg(all(feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `tooLong` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooLong)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn too_long(&self) -> bool {
        #[cfg(all(feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_too_long_ValidityState(
                self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_too_long_ValidityState(
            self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_too_long_ValidityState(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_too_short_ValidityState() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ValidityState as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ValidityState {
    #[cfg(all(feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `tooShort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooShort)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn too_short(&self) -> bool {
        #[cfg(all(feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_too_short_ValidityState(
                self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_too_short_ValidityState(
            self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_too_short_ValidityState(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_range_underflow_ValidityState() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ValidityState as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ValidityState {
    #[cfg(all(feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `rangeUnderflow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeUnderflow)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn range_underflow(&self) -> bool {
        #[cfg(all(feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_range_underflow_ValidityState(
                self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_range_underflow_ValidityState(
            self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_range_underflow_ValidityState(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_range_overflow_ValidityState() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ValidityState as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ValidityState {
    #[cfg(all(feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `rangeOverflow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeOverflow)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn range_overflow(&self) -> bool {
        #[cfg(all(feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_range_overflow_ValidityState(
                self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_range_overflow_ValidityState(
            self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_range_overflow_ValidityState(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_step_mismatch_ValidityState() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ValidityState as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ValidityState {
    #[cfg(all(feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `stepMismatch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/stepMismatch)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn step_mismatch(&self) -> bool {
        #[cfg(all(feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_step_mismatch_ValidityState(
                self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_step_mismatch_ValidityState(
            self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_step_mismatch_ValidityState(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bad_input_ValidityState() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ValidityState as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ValidityState {
    #[cfg(all(feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `badInput` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/badInput)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn bad_input(&self) -> bool {
        #[cfg(all(feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bad_input_ValidityState(
                self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bad_input_ValidityState(
            self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_bad_input_ValidityState(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_custom_error_ValidityState() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ValidityState as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ValidityState {
    #[cfg(all(feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `customError` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/customError)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn custom_error(&self) -> bool {
        #[cfg(all(feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_custom_error_ValidityState(
                self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_custom_error_ValidityState(
            self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_custom_error_ValidityState(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_valid_ValidityState() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ValidityState as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ValidityState {
    #[cfg(all(feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `valid` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valid)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn valid(&self) -> bool {
        #[cfg(all(feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_valid_ValidityState(
                self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_valid_ValidityState(
            self_: <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ValidityState as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_valid_ValidityState(self_)
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
pub static __WASM_BINDGEN_GENERATED_3eac9353f1b1a66f: [u8; 1154usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}@\x04\0\0\0\0\x0C\0\0\x02\rValidityState\x1F__widl_instanceof_ValidityState\0\0\0\0$__widl_f_value_missing_ValidityState\0\0\0\x01\rValidityState\x01\0\x01\x0CvalueMissing\x01\x01\x05self_\x0CvalueMissing\0\0\0$__widl_f_type_mismatch_ValidityState\0\0\0\x01\rValidityState\x01\0\x01\x0CtypeMismatch\x01\x01\x05self_\x0CtypeMismatch\0\0\0'__widl_f_pattern_mismatch_ValidityState\0\0\0\x01\rValidityState\x01\0\x01\x0FpatternMismatch\x01\x01\x05self_\x0FpatternMismatch\0\0\0\x1F__widl_f_too_long_ValidityState\0\0\0\x01\rValidityState\x01\0\x01\x07tooLong\x01\x01\x05self_\x07tooLong\0\0\0 __widl_f_too_short_ValidityState\0\0\0\x01\rValidityState\x01\0\x01\x08tooShort\x01\x01\x05self_\x08tooShort\0\0\0&__widl_f_range_underflow_ValidityState\0\0\0\x01\rValidityState\x01\0\x01\x0ErangeUnderflow\x01\x01\x05self_\x0ErangeUnderflow\0\0\0%__widl_f_range_overflow_ValidityState\0\0\0\x01\rValidityState\x01\0\x01\rrangeOverflow\x01\x01\x05self_\rrangeOverflow\0\0\0$__widl_f_step_mismatch_ValidityState\0\0\0\x01\rValidityState\x01\0\x01\x0CstepMismatch\x01\x01\x05self_\x0CstepMismatch\0\0\0 __widl_f_bad_input_ValidityState\0\0\0\x01\rValidityState\x01\0\x01\x08badInput\x01\x01\x05self_\x08badInput\0\0\0#__widl_f_custom_error_ValidityState\0\0\0\x01\rValidityState\x01\0\x01\x0BcustomError\x01\x01\x05self_\x0BcustomError\0\0\0\x1C__widl_f_valid_ValidityState\0\0\0\x01\rValidityState\x01\0\x01\x05valid\x01\x01\x05self_\x05valid\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
