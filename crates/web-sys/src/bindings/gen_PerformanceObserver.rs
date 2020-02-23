use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PerformanceObserver` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver)\n\n*This API requires the following crate features to be activated: `PerformanceObserver`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PerformanceObserver {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PerformanceObserver: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PerformanceObserver {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
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
            inform(79u32);
            inform(98u32);
            inform(115u32);
            inform(101u32);
            inform(114u32);
            inform(118u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for PerformanceObserver {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PerformanceObserver {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PerformanceObserver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PerformanceObserver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PerformanceObserver {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PerformanceObserver {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PerformanceObserver {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PerformanceObserver {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PerformanceObserver {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PerformanceObserver>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PerformanceObserver {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PerformanceObserver {
        #[inline]
        fn from(obj: JsValue) -> PerformanceObserver {
            PerformanceObserver { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PerformanceObserver {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PerformanceObserver> for PerformanceObserver {
        #[inline]
        fn as_ref(&self) -> &PerformanceObserver {
            self
        }
    }
    impl From<PerformanceObserver> for JsValue {
        #[inline]
        fn from(obj: PerformanceObserver) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PerformanceObserver {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PerformanceObserver(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PerformanceObserver(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PerformanceObserver(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PerformanceObserver { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PerformanceObserver) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PerformanceObserver> for ::js_sys::Object {
    #[inline]
    fn from(obj: PerformanceObserver) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PerformanceObserver {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PerformanceObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_PerformanceObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::js_sys::Function as WasmDescribe>::describe();
    <PerformanceObserver as WasmDescribe>::describe();
}
impl PerformanceObserver {
    #[cfg(all(feature = "PerformanceObserver",))]
    #[allow(bad_style)]
    #[doc = "The `new PerformanceObserver(..)` constructor, creating a new instance of `PerformanceObserver`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/PerformanceObserver)\n\n*This API requires the following crate features to be activated: `PerformanceObserver`*"]
    #[allow(clippy::all)]
    pub fn new(
        callback: &::js_sys::Function,
    ) -> Result<PerformanceObserver, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PerformanceObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_PerformanceObserver(
                callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PerformanceObserver as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_PerformanceObserver(
            callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PerformanceObserver as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(callback);
                __widl_f_new_PerformanceObserver(callback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PerformanceObserver as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PerformanceObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disconnect_PerformanceObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceObserver as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PerformanceObserver {
    #[cfg(all(feature = "PerformanceObserver",))]
    #[allow(bad_style)]
    #[doc = "The `disconnect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/disconnect)\n\n*This API requires the following crate features to be activated: `PerformanceObserver`*"]
    #[allow(clippy::all)]
    pub fn disconnect(&self) {
        #[cfg(all(feature = "PerformanceObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disconnect_PerformanceObserver(
                self_: <&PerformanceObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disconnect_PerformanceObserver(
            self_: <&PerformanceObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disconnect_PerformanceObserver(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PerformanceObserver", feature = "PerformanceObserverInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_observe_PerformanceObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PerformanceObserver as WasmDescribe>::describe();
    <&PerformanceObserverInit as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PerformanceObserver {
    #[cfg(all(feature = "PerformanceObserver", feature = "PerformanceObserverInit",))]
    #[allow(bad_style)]
    #[doc = "The `observe()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/observe)\n\n*This API requires the following crate features to be activated: `PerformanceObserver`, `PerformanceObserverInit`*"]
    #[allow(clippy::all)]
    pub fn observe(&self, options: &PerformanceObserverInit) {
        #[cfg(all(feature = "PerformanceObserver", feature = "PerformanceObserverInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_observe_PerformanceObserver(
                self_: <&PerformanceObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&PerformanceObserverInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_observe_PerformanceObserver(
            self_: <&PerformanceObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&PerformanceObserverInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&PerformanceObserverInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_observe_PerformanceObserver(self_, options)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PerformanceObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_take_records_PerformanceObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceObserver as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl PerformanceObserver {
    #[cfg(all(feature = "PerformanceObserver",))]
    #[allow(bad_style)]
    #[doc = "The `takeRecords()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/takeRecords)\n\n*This API requires the following crate features to be activated: `PerformanceObserver`*"]
    #[allow(clippy::all)]
    pub fn take_records(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "PerformanceObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_take_records_PerformanceObserver(
                self_: <&PerformanceObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_take_records_PerformanceObserver(
            self_: <&PerformanceObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_take_records_PerformanceObserver(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8b7907f6145d15d2: [u8; 515usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC1\x01\0\0\0\0\x05\0\0\x02\x13PerformanceObserver%__widl_instanceof_PerformanceObserver\0\0\0\0 __widl_f_new_PerformanceObserver\x01\0\0\x01\x13PerformanceObserver\0\x01\x01\x08callback\x03new\0\0\0'__widl_f_disconnect_PerformanceObserver\0\0\0\x01\x13PerformanceObserver\x01\0\0\x01\x01\x05self_\ndisconnect\0\0\0$__widl_f_observe_PerformanceObserver\0\0\0\x01\x13PerformanceObserver\x01\0\0\x01\x02\x05self_\x07options\x07observe\0\0\0)__widl_f_take_records_PerformanceObserver\0\0\0\x01\x13PerformanceObserver\x01\0\0\x01\x01\x05self_\x0BtakeRecords\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
