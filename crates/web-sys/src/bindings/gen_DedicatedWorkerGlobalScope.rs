use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DedicatedWorkerGlobalScope` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope)\n\n*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DedicatedWorkerGlobalScope {
    obj: WorkerGlobalScope,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DedicatedWorkerGlobalScope: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DedicatedWorkerGlobalScope {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(26u32);
            inform(68u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(99u32);
            inform(97u32);
            inform(116u32);
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
    impl core::ops::Deref for DedicatedWorkerGlobalScope {
        type Target = WorkerGlobalScope;
        #[inline]
        fn deref(&self) -> &WorkerGlobalScope {
            &self.obj
        }
    }
    impl IntoWasmAbi for DedicatedWorkerGlobalScope {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DedicatedWorkerGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DedicatedWorkerGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DedicatedWorkerGlobalScope {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DedicatedWorkerGlobalScope {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DedicatedWorkerGlobalScope {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DedicatedWorkerGlobalScope {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DedicatedWorkerGlobalScope {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DedicatedWorkerGlobalScope>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DedicatedWorkerGlobalScope {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DedicatedWorkerGlobalScope {
        #[inline]
        fn from(obj: JsValue) -> DedicatedWorkerGlobalScope {
            DedicatedWorkerGlobalScope { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DedicatedWorkerGlobalScope {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DedicatedWorkerGlobalScope> for DedicatedWorkerGlobalScope {
        #[inline]
        fn as_ref(&self) -> &DedicatedWorkerGlobalScope {
            self
        }
    }
    impl From<DedicatedWorkerGlobalScope> for JsValue {
        #[inline]
        fn from(obj: DedicatedWorkerGlobalScope) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DedicatedWorkerGlobalScope {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DedicatedWorkerGlobalScope(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DedicatedWorkerGlobalScope(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DedicatedWorkerGlobalScope(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DedicatedWorkerGlobalScope { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DedicatedWorkerGlobalScope) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DedicatedWorkerGlobalScope> for WorkerGlobalScope {
    #[inline]
    fn from(obj: DedicatedWorkerGlobalScope) -> WorkerGlobalScope {
        use wasm_bindgen::JsCast;
        WorkerGlobalScope::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<WorkerGlobalScope> for DedicatedWorkerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &WorkerGlobalScope {
        use wasm_bindgen::JsCast;
        WorkerGlobalScope::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DedicatedWorkerGlobalScope> for EventTarget {
    #[inline]
    fn from(obj: DedicatedWorkerGlobalScope) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for DedicatedWorkerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DedicatedWorkerGlobalScope> for ::js_sys::Object {
    #[inline]
    fn from(obj: DedicatedWorkerGlobalScope) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DedicatedWorkerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_DedicatedWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DedicatedWorkerGlobalScope as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DedicatedWorkerGlobalScope {
    #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/close)\n\n*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_DedicatedWorkerGlobalScope(
                self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_DedicatedWorkerGlobalScope(
            self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_close_DedicatedWorkerGlobalScope(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_post_message_DedicatedWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DedicatedWorkerGlobalScope as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DedicatedWorkerGlobalScope {
    #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `postMessage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/postMessage)\n\n*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn post_message(
        &self,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_post_message_DedicatedWorkerGlobalScope(
                self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_post_message_DedicatedWorkerGlobalScope(
            self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let message =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        message,
                    );
                __widl_f_post_message_DedicatedWorkerGlobalScope(self_, message)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_post_message_with_transfer_DedicatedWorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DedicatedWorkerGlobalScope as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DedicatedWorkerGlobalScope {
    #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `postMessage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/postMessage)\n\n*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn post_message_with_transfer(
        &self,
        message: &::wasm_bindgen::JsValue,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_post_message_with_transfer_DedicatedWorkerGlobalScope(
                self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                transfer: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_post_message_with_transfer_DedicatedWorkerGlobalScope(
            self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let message =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        message,
                    );
                let transfer =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        transfer,
                    );
                __widl_f_post_message_with_transfer_DedicatedWorkerGlobalScope(
                    self_, message, transfer,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_DedicatedWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DedicatedWorkerGlobalScope as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DedicatedWorkerGlobalScope {
    #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/name)\n\n*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_DedicatedWorkerGlobalScope(
                self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_DedicatedWorkerGlobalScope(
            self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_name_DedicatedWorkerGlobalScope(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_DedicatedWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DedicatedWorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl DedicatedWorkerGlobalScope {
    #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessage)\n\n*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_DedicatedWorkerGlobalScope(
                self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_DedicatedWorkerGlobalScope(
            self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onmessage_DedicatedWorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_DedicatedWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DedicatedWorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DedicatedWorkerGlobalScope {
    #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessage)\n\n*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_DedicatedWorkerGlobalScope(
                self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_DedicatedWorkerGlobalScope(
            self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_DedicatedWorkerGlobalScope(self_, onmessage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessageerror_DedicatedWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DedicatedWorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl DedicatedWorkerGlobalScope {
    #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessageerror)\n\n*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onmessageerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessageerror_DedicatedWorkerGlobalScope(
                self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessageerror_DedicatedWorkerGlobalScope(
            self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onmessageerror_DedicatedWorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessageerror_DedicatedWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DedicatedWorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DedicatedWorkerGlobalScope {
    #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessageerror)\n\n*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onmessageerror(&self, onmessageerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "DedicatedWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessageerror_DedicatedWorkerGlobalScope(
                self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessageerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessageerror_DedicatedWorkerGlobalScope(
            self_: <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&DedicatedWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onmessageerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessageerror,
                    );
                __widl_f_set_onmessageerror_DedicatedWorkerGlobalScope(self_, onmessageerror)
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
pub static __WASM_BINDGEN_GENERATED_5fe130f25fd8da12: [u8; 1128usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}&\x04\0\0\0\0\t\0\0\x02\x1ADedicatedWorkerGlobalScope,__widl_instanceof_DedicatedWorkerGlobalScope\0\0\0\0)__widl_f_close_DedicatedWorkerGlobalScope\0\0\0\x01\x1ADedicatedWorkerGlobalScope\x01\0\0\x01\x01\x05self_\x05close\0\0\00__widl_f_post_message_DedicatedWorkerGlobalScope\x01\0\0\x01\x1ADedicatedWorkerGlobalScope\x01\0\0\x01\x02\x05self_\x07message\x0BpostMessage\0\0\0>__widl_f_post_message_with_transfer_DedicatedWorkerGlobalScope\x01\0\0\x01\x1ADedicatedWorkerGlobalScope\x01\0\0\x01\x03\x05self_\x07message\x08transfer\x0BpostMessage\0\0\0(__widl_f_name_DedicatedWorkerGlobalScope\0\0\0\x01\x1ADedicatedWorkerGlobalScope\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0-__widl_f_onmessage_DedicatedWorkerGlobalScope\0\0\0\x01\x1ADedicatedWorkerGlobalScope\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\01__widl_f_set_onmessage_DedicatedWorkerGlobalScope\0\0\0\x01\x1ADedicatedWorkerGlobalScope\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\02__widl_f_onmessageerror_DedicatedWorkerGlobalScope\0\0\0\x01\x1ADedicatedWorkerGlobalScope\x01\0\x01\x0Eonmessageerror\x01\x01\x05self_\x0Eonmessageerror\0\0\06__widl_f_set_onmessageerror_DedicatedWorkerGlobalScope\0\0\0\x01\x1ADedicatedWorkerGlobalScope\x01\0\x02\x0Eonmessageerror\x01\x02\x05self_\x0Eonmessageerror\x0Eonmessageerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
