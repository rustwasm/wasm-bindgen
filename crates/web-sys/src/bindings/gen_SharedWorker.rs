use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SharedWorker` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker)\n\n*This API requires the following crate features to be activated: `SharedWorker`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SharedWorker {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SharedWorker: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SharedWorker {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(83u32);
            inform(104u32);
            inform(97u32);
            inform(114u32);
            inform(101u32);
            inform(100u32);
            inform(87u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for SharedWorker {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for SharedWorker {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SharedWorker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SharedWorker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SharedWorker {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SharedWorker {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SharedWorker {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SharedWorker {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SharedWorker {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SharedWorker>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SharedWorker {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SharedWorker {
        #[inline]
        fn from(obj: JsValue) -> SharedWorker {
            SharedWorker { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SharedWorker {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SharedWorker> for SharedWorker {
        #[inline]
        fn as_ref(&self) -> &SharedWorker {
            self
        }
    }
    impl From<SharedWorker> for JsValue {
        #[inline]
        fn from(obj: SharedWorker) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SharedWorker {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SharedWorker(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SharedWorker(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SharedWorker(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SharedWorker { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SharedWorker) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SharedWorker> for EventTarget {
    #[inline]
    fn from(obj: SharedWorker) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SharedWorker {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SharedWorker> for ::js_sys::Object {
    #[inline]
    fn from(obj: SharedWorker) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SharedWorker {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SharedWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_SharedWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <SharedWorker as WasmDescribe>::describe();
}
impl SharedWorker {
    #[cfg(all(feature = "SharedWorker",))]
    #[allow(bad_style)]
    #[doc = "The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)\n\n*This API requires the following crate features to be activated: `SharedWorker`*"]
    #[allow(clippy::all)]
    pub fn new(script_url: &str) -> Result<SharedWorker, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SharedWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_SharedWorker(
                script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SharedWorker as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_SharedWorker(
            script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SharedWorker as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(script_url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let script_url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(script_url);
                __widl_f_new_SharedWorker(script_url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SharedWorker as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SharedWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_SharedWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <SharedWorker as WasmDescribe>::describe();
}
impl SharedWorker {
    #[cfg(all(feature = "SharedWorker",))]
    #[allow(bad_style)]
    #[doc = "The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)\n\n*This API requires the following crate features to be activated: `SharedWorker`*"]
    #[allow(clippy::all)]
    pub fn new_with_str(
        script_url: &str,
        options: &str,
    ) -> Result<SharedWorker, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SharedWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_SharedWorker(
                script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SharedWorker as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_SharedWorker(
            script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SharedWorker as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(script_url);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let script_url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(script_url);
                let options = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_str_SharedWorker(script_url, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SharedWorker as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SharedWorker", feature = "WorkerOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_worker_options_SharedWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&WorkerOptions as WasmDescribe>::describe();
    <SharedWorker as WasmDescribe>::describe();
}
impl SharedWorker {
    #[cfg(all(feature = "SharedWorker", feature = "WorkerOptions",))]
    #[allow(bad_style)]
    #[doc = "The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)\n\n*This API requires the following crate features to be activated: `SharedWorker`, `WorkerOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_worker_options(
        script_url: &str,
        options: &WorkerOptions,
    ) -> Result<SharedWorker, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SharedWorker", feature = "WorkerOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_worker_options_SharedWorker(
                script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&WorkerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SharedWorker as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_worker_options_SharedWorker(
            script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&WorkerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SharedWorker as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(script_url);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let script_url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(script_url);
                let options =
                    <&WorkerOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_worker_options_SharedWorker(script_url, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SharedWorker as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MessagePort", feature = "SharedWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_port_SharedWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SharedWorker as WasmDescribe>::describe();
    <MessagePort as WasmDescribe>::describe();
}
impl SharedWorker {
    #[cfg(all(feature = "MessagePort", feature = "SharedWorker",))]
    #[allow(bad_style)]
    #[doc = "The `port` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/port)\n\n*This API requires the following crate features to be activated: `MessagePort`, `SharedWorker`*"]
    #[allow(clippy::all)]
    pub fn port(&self) -> MessagePort {
        #[cfg(all(feature = "MessagePort", feature = "SharedWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_port_SharedWorker(
                self_: <&SharedWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MessagePort as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_port_SharedWorker(
            self_: <&SharedWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MessagePort as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SharedWorker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_port_SharedWorker(self_)
            };
            <MessagePort as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SharedWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_SharedWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SharedWorker as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SharedWorker {
    #[cfg(all(feature = "SharedWorker",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/onerror)\n\n*This API requires the following crate features to be activated: `SharedWorker`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SharedWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_SharedWorker(
                self_: <&SharedWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_SharedWorker(
            self_: <&SharedWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SharedWorker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_SharedWorker(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SharedWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_SharedWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SharedWorker as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SharedWorker {
    #[cfg(all(feature = "SharedWorker",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/onerror)\n\n*This API requires the following crate features to be activated: `SharedWorker`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SharedWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_SharedWorker(
                self_: <&SharedWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_SharedWorker(
            self_: <&SharedWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SharedWorker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_SharedWorker(self_, onerror)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_3fcb05eca21ffcc1: [u8; 624usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}.\x02\0\0\0\0\x07\0\0\x02\x0CSharedWorker\x1E__widl_instanceof_SharedWorker\0\0\0\0\x19__widl_f_new_SharedWorker\x01\0\0\x01\x0CSharedWorker\0\x01\x01\nscript_url\x03new\0\0\0\"__widl_f_new_with_str_SharedWorker\x01\0\0\x01\x0CSharedWorker\0\x01\x02\nscript_url\x07options\x03new\0\0\0-__widl_f_new_with_worker_options_SharedWorker\x01\0\0\x01\x0CSharedWorker\0\x01\x02\nscript_url\x07options\x03new\0\0\0\x1A__widl_f_port_SharedWorker\0\0\0\x01\x0CSharedWorker\x01\0\x01\x04port\x01\x01\x05self_\x04port\0\0\0\x1D__widl_f_onerror_SharedWorker\0\0\0\x01\x0CSharedWorker\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0!__widl_f_set_onerror_SharedWorker\0\0\0\x01\x0CSharedWorker\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
