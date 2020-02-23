use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WorkerDebuggerGlobalScope` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WorkerDebuggerGlobalScope {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WorkerDebuggerGlobalScope: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WorkerDebuggerGlobalScope {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
            inform(87u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(101u32);
            inform(114u32);
            inform(68u32);
            inform(101u32);
            inform(98u32);
            inform(117u32);
            inform(103u32);
            inform(103u32);
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
    impl core::ops::Deref for WorkerDebuggerGlobalScope {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for WorkerDebuggerGlobalScope {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WorkerDebuggerGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WorkerDebuggerGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WorkerDebuggerGlobalScope {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WorkerDebuggerGlobalScope {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WorkerDebuggerGlobalScope {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WorkerDebuggerGlobalScope {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WorkerDebuggerGlobalScope {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WorkerDebuggerGlobalScope>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WorkerDebuggerGlobalScope {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WorkerDebuggerGlobalScope {
        #[inline]
        fn from(obj: JsValue) -> WorkerDebuggerGlobalScope {
            WorkerDebuggerGlobalScope { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WorkerDebuggerGlobalScope {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WorkerDebuggerGlobalScope> for WorkerDebuggerGlobalScope {
        #[inline]
        fn as_ref(&self) -> &WorkerDebuggerGlobalScope {
            self
        }
    }
    impl From<WorkerDebuggerGlobalScope> for JsValue {
        #[inline]
        fn from(obj: WorkerDebuggerGlobalScope) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WorkerDebuggerGlobalScope {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WorkerDebuggerGlobalScope(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WorkerDebuggerGlobalScope(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WorkerDebuggerGlobalScope(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WorkerDebuggerGlobalScope { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WorkerDebuggerGlobalScope) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WorkerDebuggerGlobalScope> for EventTarget {
    #[inline]
    fn from(obj: WorkerDebuggerGlobalScope) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for WorkerDebuggerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WorkerDebuggerGlobalScope> for ::js_sys::Object {
    #[inline]
    fn from(obj: WorkerDebuggerGlobalScope) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WorkerDebuggerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_sandbox_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `createSandbox()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/createSandbox)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn create_sandbox(
        &self,
        name: &str,
        prototype: &::js_sys::Object,
    ) -> Result<::js_sys::Object, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_sandbox_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                prototype: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_sandbox_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            prototype: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(prototype);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let prototype =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(prototype);
                __widl_f_create_sandbox_WorkerDebuggerGlobalScope(self_, name, prototype)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dump_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `dump()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/dump)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn dump(&self) {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dump_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dump_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_dump_WorkerDebuggerGlobalScope(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dump_with_string_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `dump()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/dump)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn dump_with_string(&self, string: &str) {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dump_with_string_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                string: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dump_with_string_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            string: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(string);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let string = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(string);
                __widl_f_dump_with_string_WorkerDebuggerGlobalScope(self_, string)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_enter_event_loop_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `enterEventLoop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/enterEventLoop)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn enter_event_loop(&self) {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_enter_event_loop_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_enter_event_loop_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_enter_event_loop_WorkerDebuggerGlobalScope(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_leave_event_loop_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `leaveEventLoop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/leaveEventLoop)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn leave_event_loop(&self) {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_leave_event_loop_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_leave_event_loop_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_leave_event_loop_WorkerDebuggerGlobalScope(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_sub_script_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `loadSubScript()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/loadSubScript)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn load_sub_script(&self, url: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_sub_script_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_sub_script_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_load_sub_script_WorkerDebuggerGlobalScope(self_, url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_sub_script_with_sandbox_WorkerDebuggerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `loadSubScript()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/loadSubScript)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn load_sub_script_with_sandbox(
        &self,
        url: &str,
        sandbox: &::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_sub_script_with_sandbox_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sandbox: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_sub_script_with_sandbox_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sandbox: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(url);
            drop(sandbox);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let sandbox =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sandbox);
                __widl_f_load_sub_script_with_sandbox_WorkerDebuggerGlobalScope(self_, url, sandbox)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_post_message_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `postMessage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/postMessage)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn post_message(&self, message: &str) {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_post_message_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_post_message_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            message: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let message = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(message);
                __widl_f_post_message_WorkerDebuggerGlobalScope(self_, message)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_report_error_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `reportError()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/reportError)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn report_error(&self, message: &str) {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_report_error_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_report_error_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            message: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let message = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(message);
                __widl_f_report_error_WorkerDebuggerGlobalScope(self_, message)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_retrieve_console_events_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `retrieveConsoleEvents()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/retrieveConsoleEvents)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn retrieve_console_events(&self) -> Result<::js_sys::Array, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_retrieve_console_events_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_retrieve_console_events_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_retrieve_console_events_WorkerDebuggerGlobalScope(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_console_event_handler_WorkerDebuggerGlobalScope()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setConsoleEventHandler()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/setConsoleEventHandler)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_console_event_handler(
        &self,
        handler: Option<&::js_sys::Function>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_console_event_handler_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_console_event_handler_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
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
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let handler =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        handler,
                    );
                __widl_f_set_console_event_handler_WorkerDebuggerGlobalScope(self_, handler)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_immediate_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `setImmediate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/setImmediate)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_immediate(
        &self,
        handler: &::js_sys::Function,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_immediate_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_immediate_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handler: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
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
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let handler =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handler);
                __widl_f_set_immediate_WorkerDebuggerGlobalScope(self_, handler)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_global_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `global` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/global)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn global(&self) -> Result<::js_sys::Object, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_global_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_global_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_global_WorkerDebuggerGlobalScope(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/onmessage)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onmessage_WorkerDebuggerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_WorkerDebuggerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WorkerDebuggerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WorkerDebuggerGlobalScope {
    #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/onmessage)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "WorkerDebuggerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_WorkerDebuggerGlobalScope(
                self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_WorkerDebuggerGlobalScope(
            self_: <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&WorkerDebuggerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_WorkerDebuggerGlobalScope(self_, onmessage)
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
pub static __WASM_BINDGEN_GENERATED_0d9272475bd8d7c5: [u8; 1899usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"})\x07\0\0\0\0\x10\0\0\x02\x19WorkerDebuggerGlobalScope+__widl_instanceof_WorkerDebuggerGlobalScope\0\0\0\01__widl_f_create_sandbox_WorkerDebuggerGlobalScope\x01\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x03\x05self_\x04name\tprototype\rcreateSandbox\0\0\0'__widl_f_dump_WorkerDebuggerGlobalScope\0\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x01\x05self_\x04dump\0\0\03__widl_f_dump_with_string_WorkerDebuggerGlobalScope\0\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x02\x05self_\x06string\x04dump\0\0\03__widl_f_enter_event_loop_WorkerDebuggerGlobalScope\0\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x01\x05self_\x0EenterEventLoop\0\0\03__widl_f_leave_event_loop_WorkerDebuggerGlobalScope\0\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x01\x05self_\x0EleaveEventLoop\0\0\02__widl_f_load_sub_script_WorkerDebuggerGlobalScope\x01\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x02\x05self_\x03url\rloadSubScript\0\0\0?__widl_f_load_sub_script_with_sandbox_WorkerDebuggerGlobalScope\x01\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x03\x05self_\x03url\x07sandbox\rloadSubScript\0\0\0/__widl_f_post_message_WorkerDebuggerGlobalScope\0\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x02\x05self_\x07message\x0BpostMessage\0\0\0/__widl_f_report_error_WorkerDebuggerGlobalScope\0\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x02\x05self_\x07message\x0BreportError\0\0\0:__widl_f_retrieve_console_events_WorkerDebuggerGlobalScope\x01\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x01\x05self_\x15retrieveConsoleEvents\0\0\0<__widl_f_set_console_event_handler_WorkerDebuggerGlobalScope\x01\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x02\x05self_\x07handler\x16setConsoleEventHandler\0\0\00__widl_f_set_immediate_WorkerDebuggerGlobalScope\x01\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\0\x01\x02\x05self_\x07handler\x0CsetImmediate\0\0\0)__widl_f_global_WorkerDebuggerGlobalScope\x01\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\x01\x06global\x01\x01\x05self_\x06global\0\0\0,__widl_f_onmessage_WorkerDebuggerGlobalScope\0\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\00__widl_f_set_onmessage_WorkerDebuggerGlobalScope\0\0\0\x01\x19WorkerDebuggerGlobalScope\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
