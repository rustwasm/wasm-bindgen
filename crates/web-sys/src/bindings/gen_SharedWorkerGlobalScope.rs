use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SharedWorkerGlobalScope` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope)\n\n*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SharedWorkerGlobalScope {
    obj: WorkerGlobalScope,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SharedWorkerGlobalScope: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SharedWorkerGlobalScope {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(23u32);
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
            inform(71u32);
            inform(108u32);
            inform(111u32);
            inform(98u32);
            inform(97u32);
            inform(108u32);
            inform(83u32);
            inform(99u32);
            inform(111u32);
            inform(112u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for SharedWorkerGlobalScope {
        type Target = WorkerGlobalScope;
        #[inline]
        fn deref(&self) -> &WorkerGlobalScope {
            &self.obj
        }
    }
    impl IntoWasmAbi for SharedWorkerGlobalScope {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SharedWorkerGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SharedWorkerGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SharedWorkerGlobalScope {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SharedWorkerGlobalScope {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SharedWorkerGlobalScope {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SharedWorkerGlobalScope {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SharedWorkerGlobalScope {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SharedWorkerGlobalScope>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SharedWorkerGlobalScope {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SharedWorkerGlobalScope {
        #[inline]
        fn from(obj: JsValue) -> SharedWorkerGlobalScope {
            SharedWorkerGlobalScope { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SharedWorkerGlobalScope {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SharedWorkerGlobalScope> for SharedWorkerGlobalScope {
        #[inline]
        fn as_ref(&self) -> &SharedWorkerGlobalScope {
            self
        }
    }
    impl From<SharedWorkerGlobalScope> for JsValue {
        #[inline]
        fn from(obj: SharedWorkerGlobalScope) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SharedWorkerGlobalScope {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SharedWorkerGlobalScope(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SharedWorkerGlobalScope(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SharedWorkerGlobalScope(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SharedWorkerGlobalScope { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SharedWorkerGlobalScope) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SharedWorkerGlobalScope> for WorkerGlobalScope {
    #[inline]
    fn from(obj: SharedWorkerGlobalScope) -> WorkerGlobalScope {
        use wasm_bindgen::JsCast;
        WorkerGlobalScope::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<WorkerGlobalScope> for SharedWorkerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &WorkerGlobalScope {
        use wasm_bindgen::JsCast;
        WorkerGlobalScope::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SharedWorkerGlobalScope> for EventTarget {
    #[inline]
    fn from(obj: SharedWorkerGlobalScope) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SharedWorkerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SharedWorkerGlobalScope> for ::js_sys::Object {
    #[inline]
    fn from(obj: SharedWorkerGlobalScope) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SharedWorkerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SharedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_SharedWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SharedWorkerGlobalScope as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SharedWorkerGlobalScope {
    #[cfg(all(feature = "SharedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/close)\n\n*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "SharedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_SharedWorkerGlobalScope(
                self_: <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_SharedWorkerGlobalScope(
            self_: <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_close_SharedWorkerGlobalScope(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SharedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_SharedWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SharedWorkerGlobalScope as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SharedWorkerGlobalScope {
    #[cfg(all(feature = "SharedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/name)\n\n*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "SharedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_SharedWorkerGlobalScope(
                self_: <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_SharedWorkerGlobalScope(
            self_: <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_name_SharedWorkerGlobalScope(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SharedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onconnect_SharedWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SharedWorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SharedWorkerGlobalScope {
    #[cfg(all(feature = "SharedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onconnect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/onconnect)\n\n*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onconnect(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SharedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onconnect_SharedWorkerGlobalScope(
                self_: <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onconnect_SharedWorkerGlobalScope(
            self_: <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onconnect_SharedWorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SharedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onconnect_SharedWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SharedWorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SharedWorkerGlobalScope {
    #[cfg(all(feature = "SharedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onconnect` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/onconnect)\n\n*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onconnect(&self, onconnect: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SharedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onconnect_SharedWorkerGlobalScope(
                self_: <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onconnect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onconnect_SharedWorkerGlobalScope(
            self_: <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onconnect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onconnect);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SharedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onconnect =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onconnect,
                    );
                __widl_f_set_onconnect_SharedWorkerGlobalScope(self_, onconnect)
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
pub static __WASM_BINDGEN_GENERATED_39382406c3803a7d: [u8; 576usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xFE\x01\0\0\0\0\x05\0\0\x02\x17SharedWorkerGlobalScope)__widl_instanceof_SharedWorkerGlobalScope\0\0\0\0&__widl_f_close_SharedWorkerGlobalScope\0\0\0\x01\x17SharedWorkerGlobalScope\x01\0\0\x01\x01\x05self_\x05close\0\0\0%__widl_f_name_SharedWorkerGlobalScope\0\0\0\x01\x17SharedWorkerGlobalScope\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0*__widl_f_onconnect_SharedWorkerGlobalScope\0\0\0\x01\x17SharedWorkerGlobalScope\x01\0\x01\tonconnect\x01\x01\x05self_\tonconnect\0\0\0.__widl_f_set_onconnect_SharedWorkerGlobalScope\0\0\0\x01\x17SharedWorkerGlobalScope\x01\0\x02\tonconnect\x01\x02\x05self_\tonconnect\tonconnect\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
