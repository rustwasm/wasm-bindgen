use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ServiceWorkerGlobalScope` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ServiceWorkerGlobalScope {
    obj: WorkerGlobalScope,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ServiceWorkerGlobalScope: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ServiceWorkerGlobalScope {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
            inform(83u32);
            inform(101u32);
            inform(114u32);
            inform(118u32);
            inform(105u32);
            inform(99u32);
            inform(101u32);
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
    impl core::ops::Deref for ServiceWorkerGlobalScope {
        type Target = WorkerGlobalScope;
        #[inline]
        fn deref(&self) -> &WorkerGlobalScope {
            &self.obj
        }
    }
    impl IntoWasmAbi for ServiceWorkerGlobalScope {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ServiceWorkerGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ServiceWorkerGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ServiceWorkerGlobalScope {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ServiceWorkerGlobalScope {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ServiceWorkerGlobalScope {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ServiceWorkerGlobalScope {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ServiceWorkerGlobalScope {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ServiceWorkerGlobalScope>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ServiceWorkerGlobalScope {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ServiceWorkerGlobalScope {
        #[inline]
        fn from(obj: JsValue) -> ServiceWorkerGlobalScope {
            ServiceWorkerGlobalScope { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ServiceWorkerGlobalScope {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ServiceWorkerGlobalScope> for ServiceWorkerGlobalScope {
        #[inline]
        fn as_ref(&self) -> &ServiceWorkerGlobalScope {
            self
        }
    }
    impl From<ServiceWorkerGlobalScope> for JsValue {
        #[inline]
        fn from(obj: ServiceWorkerGlobalScope) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ServiceWorkerGlobalScope {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ServiceWorkerGlobalScope(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ServiceWorkerGlobalScope(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ServiceWorkerGlobalScope(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ServiceWorkerGlobalScope { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ServiceWorkerGlobalScope) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ServiceWorkerGlobalScope> for WorkerGlobalScope {
    #[inline]
    fn from(obj: ServiceWorkerGlobalScope) -> WorkerGlobalScope {
        use wasm_bindgen::JsCast;
        WorkerGlobalScope::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<WorkerGlobalScope> for ServiceWorkerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &WorkerGlobalScope {
        use wasm_bindgen::JsCast;
        WorkerGlobalScope::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ServiceWorkerGlobalScope> for EventTarget {
    #[inline]
    fn from(obj: ServiceWorkerGlobalScope) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ServiceWorkerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ServiceWorkerGlobalScope> for ::js_sys::Object {
    #[inline]
    fn from(obj: ServiceWorkerGlobalScope) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ServiceWorkerGlobalScope {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_skip_waiting_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `skipWaiting()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/skipWaiting)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn skip_waiting(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_skip_waiting_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_skip_waiting_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_skip_waiting_ServiceWorkerGlobalScope(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Clients", feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clients_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Clients as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "Clients", feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `clients` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/clients)\n\n*This API requires the following crate features to be activated: `Clients`, `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn clients(&self) -> Clients {
        #[cfg(all(feature = "Clients", feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clients_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Clients as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clients_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Clients as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_clients_ServiceWorkerGlobalScope(self_)
            };
            <Clients as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "ServiceWorkerGlobalScope",
    feature = "ServiceWorkerRegistration",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_registration_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <ServiceWorkerRegistration as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(
        feature = "ServiceWorkerGlobalScope",
        feature = "ServiceWorkerRegistration",
    ))]
    #[allow(bad_style)]
    #[doc = "The `registration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/registration)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`, `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn registration(&self) -> ServiceWorkerRegistration {
        #[cfg(all(
            feature = "ServiceWorkerGlobalScope",
            feature = "ServiceWorkerRegistration",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_registration_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ServiceWorkerRegistration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_registration_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ServiceWorkerRegistration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_registration_ServiceWorkerGlobalScope(self_)
            };
            <ServiceWorkerRegistration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oninstall_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `oninstall` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn oninstall(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oninstall_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oninstall_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_oninstall_ServiceWorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oninstall_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `oninstall` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_oninstall(&self, oninstall: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oninstall_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oninstall: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oninstall_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oninstall: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oninstall);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let oninstall =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oninstall,
                    );
                __widl_f_set_oninstall_ServiceWorkerGlobalScope(self_, oninstall)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onactivate_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onactivate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onactivate(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onactivate_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onactivate_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onactivate_ServiceWorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onactivate_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onactivate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onactivate(&self, onactivate: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onactivate_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onactivate : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onactivate_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onactivate: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onactivate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onactivate =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onactivate,
                    );
                __widl_f_set_onactivate_ServiceWorkerGlobalScope(self_, onactivate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onfetch_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onfetch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onfetch(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onfetch_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onfetch_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onfetch_ServiceWorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onfetch_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onfetch` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onfetch(&self, onfetch: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onfetch_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onfetch: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onfetch_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onfetch: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onfetch);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onfetch =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onfetch,
                    );
                __widl_f_set_onfetch_ServiceWorkerGlobalScope(self_, onfetch)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onmessage_ServiceWorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_ServiceWorkerGlobalScope(self_, onmessage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpush_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onpush` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onpush(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpush_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpush_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onpush_ServiceWorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpush_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onpush` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onpush(&self, onpush: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpush_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpush: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpush_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpush: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpush);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onpush =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpush,
                    );
                __widl_f_set_onpush_ServiceWorkerGlobalScope(self_, onpush)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpushsubscriptionchange_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onpushsubscriptionchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onpushsubscriptionchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpushsubscriptionchange_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpushsubscriptionchange_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onpushsubscriptionchange_ServiceWorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpushsubscriptionchange_ServiceWorkerGlobalScope(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onpushsubscriptionchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onpushsubscriptionchange(
        &self,
        onpushsubscriptionchange: Option<&::js_sys::Function>,
    ) {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpushsubscriptionchange_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpushsubscriptionchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpushsubscriptionchange_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpushsubscriptionchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpushsubscriptionchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onpushsubscriptionchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpushsubscriptionchange,
                    );
                __widl_f_set_onpushsubscriptionchange_ServiceWorkerGlobalScope(
                    self_,
                    onpushsubscriptionchange,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onnotificationclick_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onnotificationclick` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onnotificationclick(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onnotificationclick_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onnotificationclick_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onnotificationclick_ServiceWorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onnotificationclick_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onnotificationclick` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onnotificationclick(&self, onnotificationclick: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onnotificationclick_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onnotificationclick : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onnotificationclick_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onnotificationclick : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onnotificationclick);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onnotificationclick =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onnotificationclick,
                    );
                __widl_f_set_onnotificationclick_ServiceWorkerGlobalScope(
                    self_,
                    onnotificationclick,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onnotificationclose_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onnotificationclose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn onnotificationclose(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onnotificationclose_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onnotificationclose_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onnotificationclose_ServiceWorkerGlobalScope(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onnotificationclose_ServiceWorkerGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerGlobalScope as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerGlobalScope {
    #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `onnotificationclose` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn set_onnotificationclose(&self, onnotificationclose: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorkerGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onnotificationclose_ServiceWorkerGlobalScope(
                self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onnotificationclose : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onnotificationclose_ServiceWorkerGlobalScope(
            self_: <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onnotificationclose : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onnotificationclose);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onnotificationclose =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onnotificationclose,
                    );
                __widl_f_set_onnotificationclose_ServiceWorkerGlobalScope(
                    self_,
                    onnotificationclose,
                )
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
pub static __WASM_BINDGEN_GENERATED_536860f86828b3a3: [u8; 2537usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA7\t\0\0\0\0\x14\0\0\x02\x18ServiceWorkerGlobalScope*__widl_instanceof_ServiceWorkerGlobalScope\0\0\0\0.__widl_f_skip_waiting_ServiceWorkerGlobalScope\x01\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\0\x01\x01\x05self_\x0BskipWaiting\0\0\0)__widl_f_clients_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x01\x07clients\x01\x01\x05self_\x07clients\0\0\0.__widl_f_registration_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x01\x0Cregistration\x01\x01\x05self_\x0Cregistration\0\0\0+__widl_f_oninstall_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x01\toninstall\x01\x01\x05self_\toninstall\0\0\0/__widl_f_set_oninstall_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x02\toninstall\x01\x02\x05self_\toninstall\toninstall\0\0\0,__widl_f_onactivate_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x01\nonactivate\x01\x01\x05self_\nonactivate\0\0\00__widl_f_set_onactivate_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x02\nonactivate\x01\x02\x05self_\nonactivate\nonactivate\0\0\0)__widl_f_onfetch_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x01\x07onfetch\x01\x01\x05self_\x07onfetch\0\0\0-__widl_f_set_onfetch_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x02\x07onfetch\x01\x02\x05self_\x07onfetch\x07onfetch\0\0\0+__widl_f_onmessage_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0/__widl_f_set_onmessage_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0(__widl_f_onpush_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x01\x06onpush\x01\x01\x05self_\x06onpush\0\0\0,__widl_f_set_onpush_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x02\x06onpush\x01\x02\x05self_\x06onpush\x06onpush\0\0\0:__widl_f_onpushsubscriptionchange_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x01\x18onpushsubscriptionchange\x01\x01\x05self_\x18onpushsubscriptionchange\0\0\0>__widl_f_set_onpushsubscriptionchange_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x02\x18onpushsubscriptionchange\x01\x02\x05self_\x18onpushsubscriptionchange\x18onpushsubscriptionchange\0\0\05__widl_f_onnotificationclick_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x01\x13onnotificationclick\x01\x01\x05self_\x13onnotificationclick\0\0\09__widl_f_set_onnotificationclick_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x02\x13onnotificationclick\x01\x02\x05self_\x13onnotificationclick\x13onnotificationclick\0\0\05__widl_f_onnotificationclose_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x01\x13onnotificationclose\x01\x01\x05self_\x13onnotificationclose\0\0\09__widl_f_set_onnotificationclose_ServiceWorkerGlobalScope\0\0\0\x01\x18ServiceWorkerGlobalScope\x01\0\x02\x13onnotificationclose\x01\x02\x05self_\x13onnotificationclose\x13onnotificationclose\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
