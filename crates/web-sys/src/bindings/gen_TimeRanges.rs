use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TimeRanges` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges)\n\n*This API requires the following crate features to be activated: `TimeRanges`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TimeRanges {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TimeRanges: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TimeRanges {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(84u32);
            inform(105u32);
            inform(109u32);
            inform(101u32);
            inform(82u32);
            inform(97u32);
            inform(110u32);
            inform(103u32);
            inform(101u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for TimeRanges {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for TimeRanges {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TimeRanges {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TimeRanges {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TimeRanges {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TimeRanges {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TimeRanges {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TimeRanges {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TimeRanges {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TimeRanges>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TimeRanges {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TimeRanges {
        #[inline]
        fn from(obj: JsValue) -> TimeRanges {
            TimeRanges { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TimeRanges {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TimeRanges> for TimeRanges {
        #[inline]
        fn as_ref(&self) -> &TimeRanges {
            self
        }
    }
    impl From<TimeRanges> for JsValue {
        #[inline]
        fn from(obj: TimeRanges) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TimeRanges {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TimeRanges(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TimeRanges(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TimeRanges(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TimeRanges { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TimeRanges) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TimeRanges> for ::js_sys::Object {
    #[inline]
    fn from(obj: TimeRanges) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TimeRanges {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TimeRanges",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_end_TimeRanges() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TimeRanges as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl TimeRanges {
    #[cfg(all(feature = "TimeRanges",))]
    #[allow(bad_style)]
    #[doc = "The `end()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/end)\n\n*This API requires the following crate features to be activated: `TimeRanges`*"]
    #[allow(clippy::all)]
    pub fn end(&self, index: u32) -> Result<f64, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TimeRanges",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_end_TimeRanges(
                self_: <&TimeRanges as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_end_TimeRanges(
            self_: <&TimeRanges as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TimeRanges as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_end_TimeRanges(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TimeRanges",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_TimeRanges() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TimeRanges as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl TimeRanges {
    #[cfg(all(feature = "TimeRanges",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/start)\n\n*This API requires the following crate features to be activated: `TimeRanges`*"]
    #[allow(clippy::all)]
    pub fn start(&self, index: u32) -> Result<f64, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TimeRanges",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_TimeRanges(
                self_: <&TimeRanges as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_TimeRanges(
            self_: <&TimeRanges as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TimeRanges as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_start_TimeRanges(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TimeRanges",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_TimeRanges() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TimeRanges as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl TimeRanges {
    #[cfg(all(feature = "TimeRanges",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/length)\n\n*This API requires the following crate features to be activated: `TimeRanges`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "TimeRanges",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_TimeRanges(
                self_: <&TimeRanges as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_TimeRanges(
            self_: <&TimeRanges as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TimeRanges as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_TimeRanges(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ca5feafe1386f2e7: [u8; 349usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1B\x01\0\0\0\0\x04\0\0\x02\nTimeRanges\x1C__widl_instanceof_TimeRanges\0\0\0\0\x17__widl_f_end_TimeRanges\x01\0\0\x01\nTimeRanges\x01\0\0\x01\x02\x05self_\x05index\x03end\0\0\0\x19__widl_f_start_TimeRanges\x01\0\0\x01\nTimeRanges\x01\0\0\x01\x02\x05self_\x05index\x05start\0\0\0\x1A__widl_f_length_TimeRanges\0\0\0\x01\nTimeRanges\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
