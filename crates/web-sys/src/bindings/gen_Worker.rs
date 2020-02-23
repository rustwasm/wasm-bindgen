use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Worker` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker)\n\n*This API requires the following crate features to be activated: `Worker`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Worker {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Worker: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Worker {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(6u32);
            inform(87u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for Worker {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for Worker {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Worker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Worker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Worker {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Worker {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Worker {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Worker {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Worker {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Worker>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Worker {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Worker {
        #[inline]
        fn from(obj: JsValue) -> Worker {
            Worker { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Worker {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Worker> for Worker {
        #[inline]
        fn as_ref(&self) -> &Worker {
            self
        }
    }
    impl From<Worker> for JsValue {
        #[inline]
        fn from(obj: Worker) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Worker {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Worker(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Worker(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Worker(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Worker { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Worker) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Worker> for EventTarget {
    #[inline]
    fn from(obj: Worker) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for Worker {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Worker> for ::js_sys::Object {
    #[inline]
    fn from(obj: Worker) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Worker {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Worker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <Worker as WasmDescribe>::describe();
}
impl Worker {
    #[cfg(all(feature = "Worker",))]
    #[allow(bad_style)]
    #[doc = "The `new Worker(..)` constructor, creating a new instance of `Worker`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/Worker)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn new(script_url: &str) -> Result<Worker, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Worker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Worker(
                script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Worker as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Worker(
            script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Worker as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(script_url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let script_url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(script_url);
                __widl_f_new_Worker(script_url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Worker as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Worker", feature = "WorkerOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&WorkerOptions as WasmDescribe>::describe();
    <Worker as WasmDescribe>::describe();
}
impl Worker {
    #[cfg(all(feature = "Worker", feature = "WorkerOptions",))]
    #[allow(bad_style)]
    #[doc = "The `new Worker(..)` constructor, creating a new instance of `Worker`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/Worker)\n\n*This API requires the following crate features to be activated: `Worker`, `WorkerOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        script_url: &str,
        options: &WorkerOptions,
    ) -> Result<Worker, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Worker", feature = "WorkerOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_Worker(
                script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&WorkerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Worker as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_Worker(
            script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&WorkerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Worker as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_with_options_Worker(script_url, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Worker as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Worker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_post_message_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Worker as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[cfg(all(feature = "Worker",))]
    #[allow(bad_style)]
    #[doc = "The `postMessage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn post_message(
        &self,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Worker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_post_message_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_post_message_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(message);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let message =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        message,
                    );
                __widl_f_post_message_Worker(self_, message)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Worker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_post_message_with_transfer_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Worker as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[cfg(all(feature = "Worker",))]
    #[allow(bad_style)]
    #[doc = "The `postMessage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn post_message_with_transfer(
        &self,
        message: &::wasm_bindgen::JsValue,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Worker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_post_message_with_transfer_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                transfer: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_post_message_with_transfer_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transfer: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(message);
            drop(transfer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let message =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        message,
                    );
                let transfer =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        transfer,
                    );
                __widl_f_post_message_with_transfer_Worker(self_, message, transfer)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Worker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_terminate_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Worker as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[cfg(all(feature = "Worker",))]
    #[allow(bad_style)]
    #[doc = "The `terminate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/terminate)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn terminate(&self) {
        #[cfg(all(feature = "Worker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_terminate_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_terminate_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_terminate_Worker(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Worker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Worker as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Worker {
    #[cfg(all(feature = "Worker",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Worker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessage_Worker(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Worker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Worker as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[cfg(all(feature = "Worker",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Worker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmessage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_Worker(self_, onmessage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Worker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessageerror_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Worker as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Worker {
    #[cfg(all(feature = "Worker",))]
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn onmessageerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Worker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessageerror_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessageerror_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessageerror_Worker(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Worker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessageerror_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Worker as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[cfg(all(feature = "Worker",))]
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn set_onmessageerror(&self, onmessageerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Worker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessageerror_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessageerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessageerror_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmessageerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onmessageerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessageerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessageerror,
                    );
                __widl_f_set_onmessageerror_Worker(self_, onmessageerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Worker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Worker as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Worker {
    #[cfg(all(feature = "Worker",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Worker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_Worker(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Worker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_Worker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Worker as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Worker {
    #[cfg(all(feature = "Worker",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Worker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_Worker(
                self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_Worker(
            self_: <&Worker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Worker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_Worker(self_, onerror)
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
pub static __WASM_BINDGEN_GENERATED_3751bd65d550fae6: [u8; 987usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x99\x03\0\0\0\0\x0C\0\0\x02\x06Worker\x18__widl_instanceof_Worker\0\0\0\0\x13__widl_f_new_Worker\x01\0\0\x01\x06Worker\0\x01\x01\nscript_url\x03new\0\0\0 __widl_f_new_with_options_Worker\x01\0\0\x01\x06Worker\0\x01\x02\nscript_url\x07options\x03new\0\0\0\x1C__widl_f_post_message_Worker\x01\0\0\x01\x06Worker\x01\0\0\x01\x02\x05self_\x07message\x0BpostMessage\0\0\0*__widl_f_post_message_with_transfer_Worker\x01\0\0\x01\x06Worker\x01\0\0\x01\x03\x05self_\x07message\x08transfer\x0BpostMessage\0\0\0\x19__widl_f_terminate_Worker\0\0\0\x01\x06Worker\x01\0\0\x01\x01\x05self_\tterminate\0\0\0\x19__widl_f_onmessage_Worker\0\0\0\x01\x06Worker\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0\x1D__widl_f_set_onmessage_Worker\0\0\0\x01\x06Worker\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0\x1E__widl_f_onmessageerror_Worker\0\0\0\x01\x06Worker\x01\0\x01\x0Eonmessageerror\x01\x01\x05self_\x0Eonmessageerror\0\0\0\"__widl_f_set_onmessageerror_Worker\0\0\0\x01\x06Worker\x01\0\x02\x0Eonmessageerror\x01\x02\x05self_\x0Eonmessageerror\x0Eonmessageerror\0\0\0\x17__widl_f_onerror_Worker\0\0\0\x01\x06Worker\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\x1B__widl_f_set_onerror_Worker\0\0\0\x01\x06Worker\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
