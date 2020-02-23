use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WorkerGlobalScope` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WorkerGlobalScope {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WorkerGlobalScope: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WorkerGlobalScope {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
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
    impl core::ops::Deref for WorkerGlobalScope {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for WorkerGlobalScope {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WorkerGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WorkerGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WorkerGlobalScope {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WorkerGlobalScope {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WorkerGlobalScope {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WorkerGlobalScope {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WorkerGlobalScope {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WorkerGlobalScope>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WorkerGlobalScope {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WorkerGlobalScope {
        #[inline]
        fn from(obj: JsValue) -> WorkerGlobalScope {
            WorkerGlobalScope { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WorkerGlobalScope {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WorkerGlobalScope> for WorkerGlobalScope {
        #[inline]
        fn as_ref(&self) -> &WorkerGlobalScope {
            self
        }
    }
    impl From<WorkerGlobalScope> for JsValue {
        #[inline]
        fn from(obj: WorkerGlobalScope) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WorkerGlobalScope {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WorkerGlobalScope(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WorkerGlobalScope(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WorkerGlobalScope(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WorkerGlobalScope { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WorkerGlobalScope) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WorkerGlobalScope> for EventTarget {
    #[inline]
    fn from(obj: WorkerGlobalScope) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for WorkerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WorkerGlobalScope> for ::js_sys::Object {
    #[inline]
    fn from(obj: WorkerGlobalScope) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WorkerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_scripts_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `importScripts()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn import_scripts(&self, urls: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_scripts_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_scripts_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(urls);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let urls = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls);
                __widl_f_import_scripts_WorkerGlobalScope(self_, urls)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_scripts_0_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `importScripts()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn import_scripts_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_scripts_0_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_scripts_0_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_import_scripts_0_WorkerGlobalScope(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_scripts_1_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `importScripts()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn import_scripts_1(&self, urls_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_scripts_1_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_scripts_1_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(urls_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let urls_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_1);
                __widl_f_import_scripts_1_WorkerGlobalScope(self_, urls_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_scripts_2_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `importScripts()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn import_scripts_2(
        &self,
        urls_1: &str,
        urls_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_scripts_2_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_scripts_2_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(urls_1);
            drop(urls_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let urls_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_1);
                let urls_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_2);
                __widl_f_import_scripts_2_WorkerGlobalScope(self_, urls_1, urls_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_scripts_3_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `importScripts()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn import_scripts_3(
        &self,
        urls_1: &str,
        urls_2: &str,
        urls_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_scripts_3_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_scripts_3_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(urls_1);
            drop(urls_2);
            drop(urls_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let urls_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_1);
                let urls_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_2);
                let urls_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_3);
                __widl_f_import_scripts_3_WorkerGlobalScope(self_, urls_1, urls_2, urls_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_scripts_4_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `importScripts()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn import_scripts_4(
        &self,
        urls_1: &str,
        urls_2: &str,
        urls_3: &str,
        urls_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_scripts_4_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_scripts_4_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(urls_1);
            drop(urls_2);
            drop(urls_3);
            drop(urls_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let urls_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_1);
                let urls_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_2);
                let urls_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_3);
                let urls_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_4);
                __widl_f_import_scripts_4_WorkerGlobalScope(self_, urls_1, urls_2, urls_3, urls_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_scripts_5_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `importScripts()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn import_scripts_5(
        &self,
        urls_1: &str,
        urls_2: &str,
        urls_3: &str,
        urls_4: &str,
        urls_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_scripts_5_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_scripts_5_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(urls_1);
            drop(urls_2);
            drop(urls_3);
            drop(urls_4);
            drop(urls_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let urls_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_1);
                let urls_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_2);
                let urls_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_3);
                let urls_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_4);
                let urls_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_5);
                __widl_f_import_scripts_5_WorkerGlobalScope(
                    self_, urls_1, urls_2, urls_3, urls_4, urls_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_scripts_6_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `importScripts()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn import_scripts_6(
        &self,
        urls_1: &str,
        urls_2: &str,
        urls_3: &str,
        urls_4: &str,
        urls_5: &str,
        urls_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_scripts_6_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_scripts_6_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(urls_1);
            drop(urls_2);
            drop(urls_3);
            drop(urls_4);
            drop(urls_5);
            drop(urls_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let urls_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_1);
                let urls_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_2);
                let urls_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_3);
                let urls_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_4);
                let urls_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_5);
                let urls_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_6);
                __widl_f_import_scripts_6_WorkerGlobalScope(
                    self_, urls_1, urls_2, urls_3, urls_4, urls_5, urls_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_scripts_7_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `importScripts()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn import_scripts_7(
        &self,
        urls_1: &str,
        urls_2: &str,
        urls_3: &str,
        urls_4: &str,
        urls_5: &str,
        urls_6: &str,
        urls_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_scripts_7_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                urls_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_scripts_7_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            urls_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(urls_1);
            drop(urls_2);
            drop(urls_3);
            drop(urls_4);
            drop(urls_5);
            drop(urls_6);
            drop(urls_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let urls_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_1);
                let urls_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_2);
                let urls_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_3);
                let urls_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_4);
                let urls_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_5);
                let urls_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_6);
                let urls_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(urls_7);
                __widl_f_import_scripts_7_WorkerGlobalScope(
                    self_, urls_1, urls_2, urls_3, urls_4, urls_5, urls_6, urls_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_self_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <WorkerGlobalScope as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `self` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/self)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn self_(&self) -> WorkerGlobalScope {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_self_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WorkerGlobalScope as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_self_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WorkerGlobalScope as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_self_WorkerGlobalScope(self_)
            };
            <WorkerGlobalScope as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope", feature = "WorkerLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_location_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <WorkerLocation as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope", feature = "WorkerLocation",))]
    #[allow(bad_style)]
    #[doc = "The `location` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/location)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`, `WorkerLocation`*"]
    #[allow(clippy::all)]
    pub fn location(&self) -> WorkerLocation {
        #[cfg(all(feature = "WorkerGlobalScope", feature = "WorkerLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_location_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WorkerLocation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_location_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WorkerLocation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_location_WorkerGlobalScope(self_)
            };
            <WorkerLocation as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope", feature = "WorkerNavigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_navigator_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <WorkerNavigator as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope", feature = "WorkerNavigator",))]
    #[allow(bad_style)]
    #[doc = "The `navigator` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/navigator)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`, `WorkerNavigator`*"]
    #[allow(clippy::all)]
    pub fn navigator(&self) -> WorkerNavigator {
        #[cfg(all(feature = "WorkerGlobalScope", feature = "WorkerNavigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_navigator_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WorkerNavigator as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_navigator_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WorkerNavigator as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_navigator_WorkerGlobalScope(self_)
            };
            <WorkerNavigator as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onerror)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_WorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onerror)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_WorkerGlobalScope(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onoffline_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onoffline` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onoffline)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onoffline(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onoffline_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onoffline_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onoffline_WorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onoffline_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onoffline` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onoffline)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onoffline(&self, onoffline: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onoffline_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onoffline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onoffline_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onoffline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onoffline);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onoffline =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onoffline,
                    );
                __widl_f_set_onoffline_WorkerGlobalScope(self_, onoffline)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ononline_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `ononline` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/ononline)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn ononline(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ononline_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ononline_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ononline_WorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ononline_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `ononline` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/ononline)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_ononline(&self, ononline: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ononline_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ononline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ononline_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ononline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ononline);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ononline =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ononline,
                    );
                __widl_f_set_ononline_WorkerGlobalScope(self_, ononline)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Crypto", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_crypto_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <Crypto as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "Crypto", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `crypto` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/crypto)\n\n*This API requires the following crate features to be activated: `Crypto`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn crypto(&self) -> Result<Crypto, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Crypto", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_crypto_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Crypto as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_crypto_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Crypto as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_crypto_WorkerGlobalScope(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Crypto as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_atob_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `atob()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/atob)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn atob(&self, atob: &str) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_atob_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                atob: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_atob_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            atob: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(atob);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let atob = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(atob);
                __widl_f_atob_WorkerGlobalScope(self_, atob)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_btoa_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `btoa()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/btoa)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn btoa(&self, btoa: &str) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_btoa_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                btoa: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_btoa_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            btoa: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(btoa);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let btoa = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(btoa);
                __widl_f_btoa_WorkerGlobalScope(self_, btoa)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_interval_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `clearInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/clearInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn clear_interval(&self) {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_interval_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_interval_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_interval_WorkerGlobalScope(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_interval_with_handle_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `clearInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/clearInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn clear_interval_with_handle(&self, handle: i32) {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_interval_with_handle_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handle: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_interval_with_handle_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handle: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(handle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handle = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handle);
                __widl_f_clear_interval_with_handle_WorkerGlobalScope(self_, handle)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_timeout_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `clearTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/clearTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn clear_timeout(&self) {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_timeout_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_timeout_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_timeout_WorkerGlobalScope(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_timeout_with_handle_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `clearTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/clearTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn clear_timeout_with_handle(&self, handle: i32) {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_timeout_with_handle_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handle: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_timeout_with_handle_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handle: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(handle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handle = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handle);
                __widl_f_clear_timeout_with_handle_WorkerGlobalScope(self_, handle)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_html_image_element_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&HtmlImageElement as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "HtmlImageElement", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_html_image_element(
        &self,
        a_image: &HtmlImageElement,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlImageElement", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_html_image_element_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_html_image_element_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                __widl_f_create_image_bitmap_with_html_image_element_WorkerGlobalScope(
                    self_, a_image,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_html_video_element_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&HtmlVideoElement as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "HtmlVideoElement", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_html_video_element(
        &self,
        a_image: &HtmlVideoElement,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlVideoElement", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_html_video_element_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_html_video_element_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                __widl_f_create_image_bitmap_with_html_video_element_WorkerGlobalScope(
                    self_, a_image,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_html_canvas_element_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "HtmlCanvasElement", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_html_canvas_element(
        &self,
        a_image: &HtmlCanvasElement,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_html_canvas_element_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_html_canvas_element_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                __widl_f_create_image_bitmap_with_html_canvas_element_WorkerGlobalScope(
                    self_, a_image,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_blob_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "Blob", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `Blob`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_blob(
        &self,
        a_image: &Blob,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_blob_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_blob_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                __widl_f_create_image_bitmap_with_blob_WorkerGlobalScope(self_, a_image)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageData", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_image_data_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&ImageData as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "ImageData", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageData`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_image_data(
        &self,
        a_image: &ImageData,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageData", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_image_data_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_image_data_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image = <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                __widl_f_create_image_bitmap_with_image_data_WorkerGlobalScope(self_, a_image)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_canvas_rendering_context_2d_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_canvas_rendering_context_2d(
        &self,
        a_image: &CanvasRenderingContext2d,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_canvas_rendering_context_2d_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_canvas_rendering_context_2d_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        a_image,
                    );
                __widl_f_create_image_bitmap_with_canvas_rendering_context_2d_WorkerGlobalScope(
                    self_, a_image,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_image_bitmap_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&ImageBitmap as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "ImageBitmap", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_image_bitmap(
        &self,
        a_image: &ImageBitmap,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageBitmap", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_image_bitmap_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_image_bitmap_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                __widl_f_create_image_bitmap_with_image_bitmap_WorkerGlobalScope(self_, a_image)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_buffer_source_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_buffer_source(
        &self,
        a_image: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_buffer_source_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_buffer_source_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                __widl_f_create_image_bitmap_with_buffer_source_WorkerGlobalScope(self_, a_image)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_u8_array_WorkerGlobalScope()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_u8_array(
        &self,
        a_image: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_u8_array_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_u8_array_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                __widl_f_create_image_bitmap_with_u8_array_WorkerGlobalScope(self_, a_image)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlImageElement", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&HtmlImageElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "HtmlImageElement", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        &self,
        a_image: &HtmlImageElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlImageElement", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            drop(a_sx);
            drop(a_sy);
            drop(a_sw);
            drop(a_sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                let a_sx = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sx);
                let a_sy = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sy);
                let a_sw = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sw);
                let a_sh = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sh);
                __widl_f_create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope ( self_ , a_image , a_sx , a_sy , a_sw , a_sh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&HtmlVideoElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "HtmlVideoElement", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        &self,
        a_image: &HtmlVideoElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlVideoElement", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            drop(a_sx);
            drop(a_sy);
            drop(a_sw);
            drop(a_sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                let a_sx = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sx);
                let a_sy = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sy);
                let a_sw = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sw);
                let a_sh = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sh);
                __widl_f_create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope ( self_ , a_image , a_sx , a_sy , a_sw , a_sh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "HtmlCanvasElement", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        &self,
        a_image: &HtmlCanvasElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            drop(a_sx);
            drop(a_sy);
            drop(a_sw);
            drop(a_sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                let a_sx = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sx);
                let a_sy = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sy);
                let a_sw = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sw);
                let a_sh = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sh);
                __widl_f_create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope ( self_ , a_image , a_sx , a_sy , a_sw , a_sh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "Blob", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `Blob`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        &self,
        a_image: &Blob,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            drop(a_sx);
            drop(a_sy);
            drop(a_sw);
            drop(a_sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                let a_sx = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sx);
                let a_sy = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sy);
                let a_sw = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sw);
                let a_sh = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sh);
                __widl_f_create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope ( self_ , a_image , a_sx , a_sy , a_sw , a_sh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageData", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&ImageData as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "ImageData", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageData`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        &self,
        a_image: &ImageData,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageData", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            drop(a_sx);
            drop(a_sy);
            drop(a_sw);
            drop(a_sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image = <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                let a_sx = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sx);
                let a_sy = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sy);
                let a_sw = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sw);
                let a_sh = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sh);
                __widl_f_create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope ( self_ , a_image , a_sx , a_sy , a_sw , a_sh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CanvasRenderingContext2d", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_canvas_rendering_context_2d_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "CanvasRenderingContext2d", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_canvas_rendering_context_2d_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        &self,
        a_image: &CanvasRenderingContext2d,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasRenderingContext2d", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_canvas_rendering_context_2d_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_canvas_rendering_context_2d_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            drop(a_sx);
            drop(a_sy);
            drop(a_sw);
            drop(a_sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        a_image,
                    );
                let a_sx = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sx);
                let a_sy = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sy);
                let a_sw = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sw);
                let a_sh = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sh);
                __widl_f_create_image_bitmap_with_canvas_rendering_context_2d_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope ( self_ , a_image , a_sx , a_sy , a_sw , a_sh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&ImageBitmap as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "ImageBitmap", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        &self,
        a_image: &ImageBitmap,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageBitmap", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            drop(a_sx);
            drop(a_sy);
            drop(a_sw);
            drop(a_sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                let a_sx = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sx);
                let a_sy = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sy);
                let a_sw = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sw);
                let a_sh = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sh);
                __widl_f_create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope ( self_ , a_image , a_sx , a_sy , a_sw , a_sh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_buffer_source_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_buffer_source_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        &self,
        a_image: &::js_sys::Object,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_buffer_source_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_buffer_source_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            drop(a_sx);
            drop(a_sy);
            drop(a_sw);
            drop(a_sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                let a_sx = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sx);
                let a_sy = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sy);
                let a_sw = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sw);
                let a_sh = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sh);
                __widl_f_create_image_bitmap_with_buffer_source_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope ( self_ , a_image , a_sx , a_sy , a_sw , a_sh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_bitmap_with_u8_array_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_image_bitmap_with_u8_array_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        &self,
        a_image: &mut [u8],
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_bitmap_with_u8_array_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_image: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_bitmap_with_u8_array_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_image: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sx: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sy: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sw: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_sh: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_image);
            drop(a_sx);
            drop(a_sy);
            drop(a_sw);
            drop(a_sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_image = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_image);
                let a_sx = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sx);
                let a_sy = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sy);
                let a_sw = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sw);
                let a_sh = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_sh);
                __widl_f_create_image_bitmap_with_u8_array_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope ( self_ , a_image , a_sx , a_sy , a_sw , a_sh )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Request", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fetch_with_request_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "Request", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `fetch()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/fetch)\n\n*This API requires the following crate features to be activated: `Request`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn fetch_with_request(&self, input: &Request) -> ::js_sys::Promise {
        #[cfg(all(feature = "Request", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fetch_with_request_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fetch_with_request_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(input);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let input = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                __widl_f_fetch_with_request_WorkerGlobalScope(self_, input)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fetch_with_str_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `fetch()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/fetch)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn fetch_with_str(&self, input: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fetch_with_str_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fetch_with_str_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(input);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let input = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                __widl_f_fetch_with_str_WorkerGlobalScope(self_, input)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "Request",
    feature = "RequestInit",
    feature = "WorkerGlobalScope",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fetch_with_request_and_init_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&Request as WasmDescribe>::describe();
    <&RequestInit as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(
        feature = "Request",
        feature = "RequestInit",
        feature = "WorkerGlobalScope",
    ))]
    #[allow(bad_style)]
    #[doc = "The `fetch()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/fetch)\n\n*This API requires the following crate features to be activated: `Request`, `RequestInit`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn fetch_with_request_and_init(
        &self,
        input: &Request,
        init: &RequestInit,
    ) -> ::js_sys::Promise {
        #[cfg(all(
            feature = "Request",
            feature = "RequestInit",
            feature = "WorkerGlobalScope",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fetch_with_request_and_init_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fetch_with_request_and_init_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input: <&Request as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(input);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let input = <&Request as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                let init = <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_fetch_with_request_and_init_WorkerGlobalScope(self_, input, init)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RequestInit", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fetch_with_str_and_init_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&RequestInit as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "RequestInit", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `fetch()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/fetch)\n\n*This API requires the following crate features to be activated: `RequestInit`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn fetch_with_str_and_init(&self, input: &str, init: &RequestInit) -> ::js_sys::Promise {
        #[cfg(all(feature = "RequestInit", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fetch_with_str_and_init_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init: <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fetch_with_str_and_init_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init: <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(input);
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let input = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                let init = <&RequestInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_fetch_with_str_and_init_WorkerGlobalScope(self_, input, init)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_callback_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_callback(
        &self,
        handler: &::js_sys::Function,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_callback_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_callback_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                __widl_f_set_interval_with_callback_WorkerGlobalScope(self_, handler)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_callback_and_timeout_and_arguments_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_callback_and_timeout_and_arguments(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments: &::js_sys::Array,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(arguments);
                __widl_f_set_interval_with_callback_and_timeout_and_arguments_WorkerGlobalScope(
                    self_, handler, timeout, arguments,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_callback_and_timeout_and_arguments_0_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_callback_and_timeout_and_arguments_0(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_0_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_0_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                __widl_f_set_interval_with_callback_and_timeout_and_arguments_0_WorkerGlobalScope(
                    self_, handler, timeout,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_callback_and_timeout_and_arguments_1_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_callback_and_timeout_and_arguments_1(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_1_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_1_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                __widl_f_set_interval_with_callback_and_timeout_and_arguments_1_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_callback_and_timeout_and_arguments_2_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_callback_and_timeout_and_arguments_2(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_2_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_2_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                __widl_f_set_interval_with_callback_and_timeout_and_arguments_2_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_callback_and_timeout_and_arguments_3_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_callback_and_timeout_and_arguments_3(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_3_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_3_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            drop(arguments_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                let arguments_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_3,
                    );
                __widl_f_set_interval_with_callback_and_timeout_and_arguments_3_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                    arguments_3,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_callback_and_timeout_and_arguments_4_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_callback_and_timeout_and_arguments_4(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_4_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_4_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            drop(arguments_3);
            drop(arguments_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                let arguments_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_3,
                    );
                let arguments_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_4,
                    );
                __widl_f_set_interval_with_callback_and_timeout_and_arguments_4_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                    arguments_3,
                    arguments_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_callback_and_timeout_and_arguments_5_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_callback_and_timeout_and_arguments_5(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_5_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_5_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            drop(arguments_3);
            drop(arguments_4);
            drop(arguments_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                let arguments_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_3,
                    );
                let arguments_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_4,
                    );
                let arguments_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_5,
                    );
                __widl_f_set_interval_with_callback_and_timeout_and_arguments_5_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                    arguments_3,
                    arguments_4,
                    arguments_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_callback_and_timeout_and_arguments_6_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_callback_and_timeout_and_arguments_6(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_6_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_6_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            drop(arguments_3);
            drop(arguments_4);
            drop(arguments_5);
            drop(arguments_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                let arguments_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_3,
                    );
                let arguments_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_4,
                    );
                let arguments_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_5,
                    );
                let arguments_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_6,
                    );
                __widl_f_set_interval_with_callback_and_timeout_and_arguments_6_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                    arguments_3,
                    arguments_4,
                    arguments_5,
                    arguments_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_callback_and_timeout_and_arguments_7_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_callback_and_timeout_and_arguments_7(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
        arguments_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_7_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_callback_and_timeout_and_arguments_7_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            drop(arguments_3);
            drop(arguments_4);
            drop(arguments_5);
            drop(arguments_6);
            drop(arguments_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                let arguments_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_3,
                    );
                let arguments_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_4,
                    );
                let arguments_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_5,
                    );
                let arguments_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_6,
                    );
                let arguments_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_7,
                    );
                __widl_f_set_interval_with_callback_and_timeout_and_arguments_7_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                    arguments_3,
                    arguments_4,
                    arguments_5,
                    arguments_6,
                    arguments_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_str_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_str(&self, handler: &str) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_str_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_str_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                __widl_f_set_interval_with_str_WorkerGlobalScope(self_, handler)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_str_and_timeout_and_unused_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_str_and_timeout_and_unused(
        &self,
        handler: &str,
        timeout: i32,
        unused: &::js_sys::Array,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_str_and_timeout_and_unused_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_str_and_timeout_and_unused_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unused);
                __widl_f_set_interval_with_str_and_timeout_and_unused_WorkerGlobalScope(
                    self_, handler, timeout, unused,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_str_and_timeout_and_unused_0_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_str_and_timeout_and_unused_0(
        &self,
        handler: &str,
        timeout: i32,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_str_and_timeout_and_unused_0_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_str_and_timeout_and_unused_0_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                __widl_f_set_interval_with_str_and_timeout_and_unused_0_WorkerGlobalScope(
                    self_, handler, timeout,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_str_and_timeout_and_unused_1_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_str_and_timeout_and_unused_1(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_str_and_timeout_and_unused_1_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_str_and_timeout_and_unused_1_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                __widl_f_set_interval_with_str_and_timeout_and_unused_1_WorkerGlobalScope(
                    self_, handler, timeout, unused_1,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_str_and_timeout_and_unused_2_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_str_and_timeout_and_unused_2(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_str_and_timeout_and_unused_2_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_str_and_timeout_and_unused_2_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                __widl_f_set_interval_with_str_and_timeout_and_unused_2_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_str_and_timeout_and_unused_3_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_str_and_timeout_and_unused_3(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_str_and_timeout_and_unused_3_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_str_and_timeout_and_unused_3_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            drop(unused_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                let unused_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_3,
                    );
                __widl_f_set_interval_with_str_and_timeout_and_unused_3_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2, unused_3,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_str_and_timeout_and_unused_4_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_str_and_timeout_and_unused_4(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_str_and_timeout_and_unused_4_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_str_and_timeout_and_unused_4_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            drop(unused_3);
            drop(unused_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                let unused_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_3,
                    );
                let unused_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_4,
                    );
                __widl_f_set_interval_with_str_and_timeout_and_unused_4_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2, unused_3, unused_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_str_and_timeout_and_unused_5_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_str_and_timeout_and_unused_5(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_str_and_timeout_and_unused_5_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_str_and_timeout_and_unused_5_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            drop(unused_3);
            drop(unused_4);
            drop(unused_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                let unused_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_3,
                    );
                let unused_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_4,
                    );
                let unused_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_5,
                    );
                __widl_f_set_interval_with_str_and_timeout_and_unused_5_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2, unused_3, unused_4, unused_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_str_and_timeout_and_unused_6_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_str_and_timeout_and_unused_6(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_str_and_timeout_and_unused_6_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_str_and_timeout_and_unused_6_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            drop(unused_3);
            drop(unused_4);
            drop(unused_5);
            drop(unused_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                let unused_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_3,
                    );
                let unused_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_4,
                    );
                let unused_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_5,
                    );
                let unused_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_6,
                    );
                __widl_f_set_interval_with_str_and_timeout_and_unused_6_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2, unused_3, unused_4, unused_5,
                    unused_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interval_with_str_and_timeout_and_unused_7_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setInterval()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_interval_with_str_and_timeout_and_unused_7(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
        unused_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interval_with_str_and_timeout_and_unused_7_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interval_with_str_and_timeout_and_unused_7_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            drop(unused_3);
            drop(unused_4);
            drop(unused_5);
            drop(unused_6);
            drop(unused_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                let unused_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_3,
                    );
                let unused_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_4,
                    );
                let unused_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_5,
                    );
                let unused_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_6,
                    );
                let unused_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_7,
                    );
                __widl_f_set_interval_with_str_and_timeout_and_unused_7_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2, unused_3, unused_4, unused_5,
                    unused_6, unused_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_callback_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_callback(
        &self,
        handler: &::js_sys::Function,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_callback_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_callback_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                __widl_f_set_timeout_with_callback_WorkerGlobalScope(self_, handler)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_callback_and_timeout_and_arguments_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_callback_and_timeout_and_arguments(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments: &::js_sys::Array,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(arguments);
                __widl_f_set_timeout_with_callback_and_timeout_and_arguments_WorkerGlobalScope(
                    self_, handler, timeout, arguments,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_callback_and_timeout_and_arguments_0_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_0(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_0_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_0_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                __widl_f_set_timeout_with_callback_and_timeout_and_arguments_0_WorkerGlobalScope(
                    self_, handler, timeout,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_callback_and_timeout_and_arguments_1_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_1(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_1_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_1_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                __widl_f_set_timeout_with_callback_and_timeout_and_arguments_1_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_callback_and_timeout_and_arguments_2_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_2(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_2_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_2_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                __widl_f_set_timeout_with_callback_and_timeout_and_arguments_2_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_callback_and_timeout_and_arguments_3_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_3(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_3_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_3_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            drop(arguments_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                let arguments_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_3,
                    );
                __widl_f_set_timeout_with_callback_and_timeout_and_arguments_3_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                    arguments_3,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_callback_and_timeout_and_arguments_4_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_4(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_4_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_4_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            drop(arguments_3);
            drop(arguments_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                let arguments_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_3,
                    );
                let arguments_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_4,
                    );
                __widl_f_set_timeout_with_callback_and_timeout_and_arguments_4_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                    arguments_3,
                    arguments_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_callback_and_timeout_and_arguments_5_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_5(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_5_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_5_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            drop(arguments_3);
            drop(arguments_4);
            drop(arguments_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                let arguments_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_3,
                    );
                let arguments_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_4,
                    );
                let arguments_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_5,
                    );
                __widl_f_set_timeout_with_callback_and_timeout_and_arguments_5_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                    arguments_3,
                    arguments_4,
                    arguments_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_callback_and_timeout_and_arguments_6_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_6(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_6_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_6_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            drop(arguments_3);
            drop(arguments_4);
            drop(arguments_5);
            drop(arguments_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                let arguments_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_3,
                    );
                let arguments_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_4,
                    );
                let arguments_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_5,
                    );
                let arguments_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_6,
                    );
                __widl_f_set_timeout_with_callback_and_timeout_and_arguments_6_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                    arguments_3,
                    arguments_4,
                    arguments_5,
                    arguments_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_callback_and_timeout_and_arguments_7_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_7(
        &self,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
        arguments_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_7_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arguments_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_callback_and_timeout_and_arguments_7_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arguments_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(arguments_1);
            drop(arguments_2);
            drop(arguments_3);
            drop(arguments_4);
            drop(arguments_5);
            drop(arguments_6);
            drop(arguments_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let arguments_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_1,
                    );
                let arguments_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_2,
                    );
                let arguments_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_3,
                    );
                let arguments_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_4,
                    );
                let arguments_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_5,
                    );
                let arguments_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_6,
                    );
                let arguments_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        arguments_7,
                    );
                __widl_f_set_timeout_with_callback_and_timeout_and_arguments_7_WorkerGlobalScope(
                    self_,
                    handler,
                    timeout,
                    arguments_1,
                    arguments_2,
                    arguments_3,
                    arguments_4,
                    arguments_5,
                    arguments_6,
                    arguments_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_str_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_str(&self, handler: &str) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_str_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_str_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                __widl_f_set_timeout_with_str_WorkerGlobalScope(self_, handler)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_str_and_timeout_and_unused_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_str_and_timeout_and_unused(
        &self,
        handler: &str,
        timeout: i32,
        unused: &::js_sys::Array,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_str_and_timeout_and_unused_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_str_and_timeout_and_unused_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unused);
                __widl_f_set_timeout_with_str_and_timeout_and_unused_WorkerGlobalScope(
                    self_, handler, timeout, unused,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_str_and_timeout_and_unused_0_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_str_and_timeout_and_unused_0(
        &self,
        handler: &str,
        timeout: i32,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_str_and_timeout_and_unused_0_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_str_and_timeout_and_unused_0_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                __widl_f_set_timeout_with_str_and_timeout_and_unused_0_WorkerGlobalScope(
                    self_, handler, timeout,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_str_and_timeout_and_unused_1_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_str_and_timeout_and_unused_1(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_str_and_timeout_and_unused_1_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_str_and_timeout_and_unused_1_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                __widl_f_set_timeout_with_str_and_timeout_and_unused_1_WorkerGlobalScope(
                    self_, handler, timeout, unused_1,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_str_and_timeout_and_unused_2_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_str_and_timeout_and_unused_2(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_str_and_timeout_and_unused_2_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_str_and_timeout_and_unused_2_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                __widl_f_set_timeout_with_str_and_timeout_and_unused_2_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_str_and_timeout_and_unused_3_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_str_and_timeout_and_unused_3(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_str_and_timeout_and_unused_3_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_str_and_timeout_and_unused_3_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            drop(unused_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                let unused_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_3,
                    );
                __widl_f_set_timeout_with_str_and_timeout_and_unused_3_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2, unused_3,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_str_and_timeout_and_unused_4_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_str_and_timeout_and_unused_4(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_str_and_timeout_and_unused_4_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_str_and_timeout_and_unused_4_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            drop(unused_3);
            drop(unused_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                let unused_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_3,
                    );
                let unused_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_4,
                    );
                __widl_f_set_timeout_with_str_and_timeout_and_unused_4_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2, unused_3, unused_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_str_and_timeout_and_unused_5_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_str_and_timeout_and_unused_5(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_str_and_timeout_and_unused_5_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_str_and_timeout_and_unused_5_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            drop(unused_3);
            drop(unused_4);
            drop(unused_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                let unused_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_3,
                    );
                let unused_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_4,
                    );
                let unused_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_5,
                    );
                __widl_f_set_timeout_with_str_and_timeout_and_unused_5_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2, unused_3, unused_4, unused_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_str_and_timeout_and_unused_6_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_str_and_timeout_and_unused_6(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_str_and_timeout_and_unused_6_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_str_and_timeout_and_unused_6_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            drop(unused_3);
            drop(unused_4);
            drop(unused_5);
            drop(unused_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                let unused_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_3,
                    );
                let unused_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_4,
                    );
                let unused_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_5,
                    );
                let unused_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_6,
                    );
                __widl_f_set_timeout_with_str_and_timeout_and_unused_6_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2, unused_3, unused_4, unused_5,
                    unused_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeout_with_str_and_timeout_and_unused_7_WorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setTimeout()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_timeout_with_str_and_timeout_and_unused_7(
        &self,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
        unused_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeout_with_str_and_timeout_and_unused_7_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unused_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeout_with_str_and_timeout_and_unused_7_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeout: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unused_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(handler);
            drop(timeout);
            drop(unused_1);
            drop(unused_2);
            drop(unused_3);
            drop(unused_4);
            drop(unused_5);
            drop(unused_6);
            drop(unused_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handler = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                let timeout = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timeout);
                let unused_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_1,
                    );
                let unused_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_2,
                    );
                let unused_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_3,
                    );
                let unused_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_4,
                    );
                let unused_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_5,
                    );
                let unused_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_6,
                    );
                let unused_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unused_7,
                    );
                __widl_f_set_timeout_with_str_and_timeout_and_unused_7_WorkerGlobalScope(
                    self_, handler, timeout, unused_1, unused_2, unused_3, unused_4, unused_5,
                    unused_6, unused_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_origin_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `origin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/origin)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn origin(&self) -> String {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_origin_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_origin_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_origin_WorkerGlobalScope(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_secure_context_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `isSecureContext` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/isSecureContext)\n\n*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn is_secure_context(&self) -> bool {
        #[cfg(all(feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_secure_context_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_secure_context_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_secure_context_WorkerGlobalScope(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbFactory", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_indexed_db_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <Option<IdbFactory> as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "IdbFactory", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `indexedDB` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/indexedDB)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn indexed_db(&self) -> Result<Option<IdbFactory>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFactory", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_indexed_db_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFactory> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_indexed_db_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFactory> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_indexed_db_WorkerGlobalScope(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFactory> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CacheStorage", feature = "WorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_caches_WorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerGlobalScope as WasmDescribe>::describe();
    <CacheStorage as WasmDescribe>::describe();
}
impl WorkerGlobalScope {
    #[cfg(all(feature = "CacheStorage", feature = "WorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `caches` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/caches)\n\n*This API requires the following crate features to be activated: `CacheStorage`, `WorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn caches(&self) -> Result<CacheStorage, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CacheStorage", feature = "WorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_caches_WorkerGlobalScope(
                self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CacheStorage as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_caches_WorkerGlobalScope(
            self_: <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CacheStorage as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_caches_WorkerGlobalScope(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<CacheStorage as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_50d304459ce6982e: [u8; 12997usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x832\0\0\0\0\\\0\0\x02\x11WorkerGlobalScope#__widl_instanceof_WorkerGlobalScope\0\0\0\0)__widl_f_import_scripts_WorkerGlobalScope\x01\x01\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x04urls\rimportScripts\0\0\0+__widl_f_import_scripts_0_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x01\x05self_\rimportScripts\0\0\0+__widl_f_import_scripts_1_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x06urls_1\rimportScripts\0\0\0+__widl_f_import_scripts_2_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x03\x05self_\x06urls_1\x06urls_2\rimportScripts\0\0\0+__widl_f_import_scripts_3_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x04\x05self_\x06urls_1\x06urls_2\x06urls_3\rimportScripts\0\0\0+__widl_f_import_scripts_4_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x05\x05self_\x06urls_1\x06urls_2\x06urls_3\x06urls_4\rimportScripts\0\0\0+__widl_f_import_scripts_5_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x06urls_1\x06urls_2\x06urls_3\x06urls_4\x06urls_5\rimportScripts\0\0\0+__widl_f_import_scripts_6_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x07\x05self_\x06urls_1\x06urls_2\x06urls_3\x06urls_4\x06urls_5\x06urls_6\rimportScripts\0\0\0+__widl_f_import_scripts_7_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x08\x05self_\x06urls_1\x06urls_2\x06urls_3\x06urls_4\x06urls_5\x06urls_6\x06urls_7\rimportScripts\0\0\0\x1F__widl_f_self_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\x01\x04self\x01\x01\x05self_\x04self\0\0\0#__widl_f_location_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\x01\x08location\x01\x01\x05self_\x08location\0\0\0$__widl_f_navigator_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\x01\tnavigator\x01\x01\x05self_\tnavigator\0\0\0\"__widl_f_onerror_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0&__widl_f_set_onerror_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0$__widl_f_onoffline_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\x01\tonoffline\x01\x01\x05self_\tonoffline\0\0\0(__widl_f_set_onoffline_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\x02\tonoffline\x01\x02\x05self_\tonoffline\tonoffline\0\0\0#__widl_f_ononline_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\x01\x08ononline\x01\x01\x05self_\x08ononline\0\0\0'__widl_f_set_ononline_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\x02\x08ononline\x01\x02\x05self_\x08ononline\x08ononline\0\0\0!__widl_f_crypto_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\x01\x06crypto\x01\x01\x05self_\x06crypto\0\0\0\x1F__widl_f_atob_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x04atob\x04atob\0\0\0\x1F__widl_f_btoa_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x04btoa\x04btoa\0\0\0)__widl_f_clear_interval_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x01\x05self_\rclearInterval\0\0\05__widl_f_clear_interval_with_handle_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x06handle\rclearInterval\0\0\0(__widl_f_clear_timeout_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x01\x05self_\x0CclearTimeout\0\0\04__widl_f_clear_timeout_with_handle_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x06handle\x0CclearTimeout\0\0\0F__widl_f_create_image_bitmap_with_html_image_element_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07a_image\x11createImageBitmap\0\0\0F__widl_f_create_image_bitmap_with_html_video_element_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07a_image\x11createImageBitmap\0\0\0G__widl_f_create_image_bitmap_with_html_canvas_element_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07a_image\x11createImageBitmap\0\0\08__widl_f_create_image_bitmap_with_blob_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07a_image\x11createImageBitmap\0\0\0>__widl_f_create_image_bitmap_with_image_data_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07a_image\x11createImageBitmap\0\0\0O__widl_f_create_image_bitmap_with_canvas_rendering_context_2d_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07a_image\x11createImageBitmap\0\0\0@__widl_f_create_image_bitmap_with_image_bitmap_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07a_image\x11createImageBitmap\0\0\0A__widl_f_create_image_bitmap_with_buffer_source_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07a_image\x11createImageBitmap\0\0\0<__widl_f_create_image_bitmap_with_u8_array_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07a_image\x11createImageBitmap\0\0\0j__widl_f_create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07a_image\x04a_sx\x04a_sy\x04a_sw\x04a_sh\x11createImageBitmap\0\0\0j__widl_f_create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07a_image\x04a_sx\x04a_sy\x04a_sw\x04a_sh\x11createImageBitmap\0\0\0k__widl_f_create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07a_image\x04a_sx\x04a_sy\x04a_sw\x04a_sh\x11createImageBitmap\0\0\0\\__widl_f_create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07a_image\x04a_sx\x04a_sy\x04a_sw\x04a_sh\x11createImageBitmap\0\0\0b__widl_f_create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07a_image\x04a_sx\x04a_sy\x04a_sw\x04a_sh\x11createImageBitmap\0\0\0s__widl_f_create_image_bitmap_with_canvas_rendering_context_2d_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07a_image\x04a_sx\x04a_sy\x04a_sw\x04a_sh\x11createImageBitmap\0\0\0d__widl_f_create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07a_image\x04a_sx\x04a_sy\x04a_sw\x04a_sh\x11createImageBitmap\0\0\0e__widl_f_create_image_bitmap_with_buffer_source_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07a_image\x04a_sx\x04a_sy\x04a_sw\x04a_sh\x11createImageBitmap\0\0\0`__widl_f_create_image_bitmap_with_u8_array_and_a_sx_and_a_sy_and_a_sw_and_a_sh_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07a_image\x04a_sx\x04a_sy\x04a_sw\x04a_sh\x11createImageBitmap\0\0\0-__widl_f_fetch_with_request_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x05input\x05fetch\0\0\0)__widl_f_fetch_with_str_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x05input\x05fetch\0\0\06__widl_f_fetch_with_request_and_init_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x03\x05self_\x05input\x04init\x05fetch\0\0\02__widl_f_fetch_with_str_and_init_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x03\x05self_\x05input\x04init\x05fetch\0\0\05__widl_f_set_interval_with_callback_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07handler\x0BsetInterval\0\0\0O__widl_f_set_interval_with_callback_and_timeout_and_arguments_WorkerGlobalScope\x01\x01\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x04\x05self_\x07handler\x07timeout\targuments\x0BsetInterval\0\0\0Q__widl_f_set_interval_with_callback_and_timeout_and_arguments_0_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x03\x05self_\x07handler\x07timeout\x0BsetInterval\0\0\0Q__widl_f_set_interval_with_callback_and_timeout_and_arguments_1_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x04\x05self_\x07handler\x07timeout\x0Barguments_1\x0BsetInterval\0\0\0Q__widl_f_set_interval_with_callback_and_timeout_and_arguments_2_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x05\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\x0BsetInterval\0\0\0Q__widl_f_set_interval_with_callback_and_timeout_and_arguments_3_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\x0Barguments_3\x0BsetInterval\0\0\0Q__widl_f_set_interval_with_callback_and_timeout_and_arguments_4_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x07\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\x0Barguments_3\x0Barguments_4\x0BsetInterval\0\0\0Q__widl_f_set_interval_with_callback_and_timeout_and_arguments_5_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x08\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\x0Barguments_3\x0Barguments_4\x0Barguments_5\x0BsetInterval\0\0\0Q__widl_f_set_interval_with_callback_and_timeout_and_arguments_6_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\t\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\x0Barguments_3\x0Barguments_4\x0Barguments_5\x0Barguments_6\x0BsetInterval\0\0\0Q__widl_f_set_interval_with_callback_and_timeout_and_arguments_7_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\n\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\x0Barguments_3\x0Barguments_4\x0Barguments_5\x0Barguments_6\x0Barguments_7\x0BsetInterval\0\0\00__widl_f_set_interval_with_str_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07handler\x0BsetInterval\0\0\0G__widl_f_set_interval_with_str_and_timeout_and_unused_WorkerGlobalScope\x01\x01\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x04\x05self_\x07handler\x07timeout\x06unused\x0BsetInterval\0\0\0I__widl_f_set_interval_with_str_and_timeout_and_unused_0_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x03\x05self_\x07handler\x07timeout\x0BsetInterval\0\0\0I__widl_f_set_interval_with_str_and_timeout_and_unused_1_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x04\x05self_\x07handler\x07timeout\x08unused_1\x0BsetInterval\0\0\0I__widl_f_set_interval_with_str_and_timeout_and_unused_2_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x05\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\x0BsetInterval\0\0\0I__widl_f_set_interval_with_str_and_timeout_and_unused_3_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\x08unused_3\x0BsetInterval\0\0\0I__widl_f_set_interval_with_str_and_timeout_and_unused_4_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x07\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\x08unused_3\x08unused_4\x0BsetInterval\0\0\0I__widl_f_set_interval_with_str_and_timeout_and_unused_5_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x08\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\x08unused_3\x08unused_4\x08unused_5\x0BsetInterval\0\0\0I__widl_f_set_interval_with_str_and_timeout_and_unused_6_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\t\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\x08unused_3\x08unused_4\x08unused_5\x08unused_6\x0BsetInterval\0\0\0I__widl_f_set_interval_with_str_and_timeout_and_unused_7_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\n\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\x08unused_3\x08unused_4\x08unused_5\x08unused_6\x08unused_7\x0BsetInterval\0\0\04__widl_f_set_timeout_with_callback_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07handler\nsetTimeout\0\0\0N__widl_f_set_timeout_with_callback_and_timeout_and_arguments_WorkerGlobalScope\x01\x01\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x04\x05self_\x07handler\x07timeout\targuments\nsetTimeout\0\0\0P__widl_f_set_timeout_with_callback_and_timeout_and_arguments_0_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x03\x05self_\x07handler\x07timeout\nsetTimeout\0\0\0P__widl_f_set_timeout_with_callback_and_timeout_and_arguments_1_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x04\x05self_\x07handler\x07timeout\x0Barguments_1\nsetTimeout\0\0\0P__widl_f_set_timeout_with_callback_and_timeout_and_arguments_2_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x05\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\nsetTimeout\0\0\0P__widl_f_set_timeout_with_callback_and_timeout_and_arguments_3_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\x0Barguments_3\nsetTimeout\0\0\0P__widl_f_set_timeout_with_callback_and_timeout_and_arguments_4_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x07\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\x0Barguments_3\x0Barguments_4\nsetTimeout\0\0\0P__widl_f_set_timeout_with_callback_and_timeout_and_arguments_5_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x08\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\x0Barguments_3\x0Barguments_4\x0Barguments_5\nsetTimeout\0\0\0P__widl_f_set_timeout_with_callback_and_timeout_and_arguments_6_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\t\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\x0Barguments_3\x0Barguments_4\x0Barguments_5\x0Barguments_6\nsetTimeout\0\0\0P__widl_f_set_timeout_with_callback_and_timeout_and_arguments_7_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\n\x05self_\x07handler\x07timeout\x0Barguments_1\x0Barguments_2\x0Barguments_3\x0Barguments_4\x0Barguments_5\x0Barguments_6\x0Barguments_7\nsetTimeout\0\0\0/__widl_f_set_timeout_with_str_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07handler\nsetTimeout\0\0\0F__widl_f_set_timeout_with_str_and_timeout_and_unused_WorkerGlobalScope\x01\x01\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x04\x05self_\x07handler\x07timeout\x06unused\nsetTimeout\0\0\0H__widl_f_set_timeout_with_str_and_timeout_and_unused_0_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x03\x05self_\x07handler\x07timeout\nsetTimeout\0\0\0H__widl_f_set_timeout_with_str_and_timeout_and_unused_1_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x04\x05self_\x07handler\x07timeout\x08unused_1\nsetTimeout\0\0\0H__widl_f_set_timeout_with_str_and_timeout_and_unused_2_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x05\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\nsetTimeout\0\0\0H__widl_f_set_timeout_with_str_and_timeout_and_unused_3_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x06\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\x08unused_3\nsetTimeout\0\0\0H__widl_f_set_timeout_with_str_and_timeout_and_unused_4_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x07\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\x08unused_3\x08unused_4\nsetTimeout\0\0\0H__widl_f_set_timeout_with_str_and_timeout_and_unused_5_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\x08\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\x08unused_3\x08unused_4\x08unused_5\nsetTimeout\0\0\0H__widl_f_set_timeout_with_str_and_timeout_and_unused_6_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\t\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\x08unused_3\x08unused_4\x08unused_5\x08unused_6\nsetTimeout\0\0\0H__widl_f_set_timeout_with_str_and_timeout_and_unused_7_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\0\x01\n\x05self_\x07handler\x07timeout\x08unused_1\x08unused_2\x08unused_3\x08unused_4\x08unused_5\x08unused_6\x08unused_7\nsetTimeout\0\0\0!__widl_f_origin_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\x01\x06origin\x01\x01\x05self_\x06origin\0\0\0,__widl_f_is_secure_context_WorkerGlobalScope\0\0\0\x01\x11WorkerGlobalScope\x01\0\x01\x0FisSecureContext\x01\x01\x05self_\x0FisSecureContext\0\0\0%__widl_f_indexed_db_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\x01\tindexedDB\x01\x01\x05self_\tindexedDB\0\0\0!__widl_f_caches_WorkerGlobalScope\x01\0\0\x01\x11WorkerGlobalScope\x01\0\x01\x06caches\x01\x01\x05self_\x06caches\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
