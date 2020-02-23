use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IdleDeadline` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline)\n\n*This API requires the following crate features to be activated: `IdleDeadline`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdleDeadline {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdleDeadline: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdleDeadline {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(73u32);
            inform(100u32);
            inform(108u32);
            inform(101u32);
            inform(68u32);
            inform(101u32);
            inform(97u32);
            inform(100u32);
            inform(108u32);
            inform(105u32);
            inform(110u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for IdleDeadline {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdleDeadline {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdleDeadline {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdleDeadline {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdleDeadline {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdleDeadline {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdleDeadline {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdleDeadline {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdleDeadline {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdleDeadline>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdleDeadline {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdleDeadline {
        #[inline]
        fn from(obj: JsValue) -> IdleDeadline {
            IdleDeadline { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdleDeadline {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdleDeadline> for IdleDeadline {
        #[inline]
        fn as_ref(&self) -> &IdleDeadline {
            self
        }
    }
    impl From<IdleDeadline> for JsValue {
        #[inline]
        fn from(obj: IdleDeadline) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdleDeadline {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IdleDeadline(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IdleDeadline(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IdleDeadline(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdleDeadline { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdleDeadline) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdleDeadline> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdleDeadline) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdleDeadline {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdleDeadline",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_time_remaining_IdleDeadline() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdleDeadline as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl IdleDeadline {
    #[cfg(all(feature = "IdleDeadline",))]
    #[allow(bad_style)]
    #[doc = "The `timeRemaining()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/timeRemaining)\n\n*This API requires the following crate features to be activated: `IdleDeadline`*"]
    #[allow(clippy::all)]
    pub fn time_remaining(&self) -> f64 {
        #[cfg(all(feature = "IdleDeadline",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_remaining_IdleDeadline(
                self_: <&IdleDeadline as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_remaining_IdleDeadline(
            self_: <&IdleDeadline as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdleDeadline as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_time_remaining_IdleDeadline(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdleDeadline",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_did_timeout_IdleDeadline() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdleDeadline as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl IdleDeadline {
    #[cfg(all(feature = "IdleDeadline",))]
    #[allow(bad_style)]
    #[doc = "The `didTimeout` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/didTimeout)\n\n*This API requires the following crate features to be activated: `IdleDeadline`*"]
    #[allow(clippy::all)]
    pub fn did_timeout(&self) -> bool {
        #[cfg(all(feature = "IdleDeadline",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_did_timeout_IdleDeadline(
                self_: <&IdleDeadline as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_did_timeout_IdleDeadline(
            self_: <&IdleDeadline as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdleDeadline as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_did_timeout_IdleDeadline(self_)
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
pub static __WASM_BINDGEN_GENERATED_2dab5690871374fe: [u8; 322usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\0\x01\0\0\0\0\x03\0\0\x02\x0CIdleDeadline\x1E__widl_instanceof_IdleDeadline\0\0\0\0$__widl_f_time_remaining_IdleDeadline\0\0\0\x01\x0CIdleDeadline\x01\0\0\x01\x01\x05self_\rtimeRemaining\0\0\0!__widl_f_did_timeout_IdleDeadline\0\0\0\x01\x0CIdleDeadline\x01\0\x01\ndidTimeout\x01\x01\x05self_\ndidTimeout\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
