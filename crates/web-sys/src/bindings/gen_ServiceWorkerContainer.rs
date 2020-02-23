use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ServiceWorkerContainer` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ServiceWorkerContainer {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ServiceWorkerContainer: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ServiceWorkerContainer {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
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
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(97u32);
            inform(105u32);
            inform(110u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for ServiceWorkerContainer {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for ServiceWorkerContainer {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ServiceWorkerContainer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ServiceWorkerContainer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ServiceWorkerContainer {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ServiceWorkerContainer {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ServiceWorkerContainer {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ServiceWorkerContainer {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ServiceWorkerContainer {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ServiceWorkerContainer>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ServiceWorkerContainer {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ServiceWorkerContainer {
        #[inline]
        fn from(obj: JsValue) -> ServiceWorkerContainer {
            ServiceWorkerContainer { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ServiceWorkerContainer {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ServiceWorkerContainer> for ServiceWorkerContainer {
        #[inline]
        fn as_ref(&self) -> &ServiceWorkerContainer {
            self
        }
    }
    impl From<ServiceWorkerContainer> for JsValue {
        #[inline]
        fn from(obj: ServiceWorkerContainer) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ServiceWorkerContainer {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ServiceWorkerContainer(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ServiceWorkerContainer(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ServiceWorkerContainer(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ServiceWorkerContainer { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ServiceWorkerContainer) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ServiceWorkerContainer> for EventTarget {
    #[inline]
    fn from(obj: ServiceWorkerContainer) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ServiceWorkerContainer {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ServiceWorkerContainer> for ::js_sys::Object {
    #[inline]
    fn from(obj: ServiceWorkerContainer) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ServiceWorkerContainer {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_registration_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `getRegistration()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn get_registration(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_registration_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_registration_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_registration_ServiceWorkerContainer(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_registration_with_document_url_ServiceWorkerContainer(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `getRegistration()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn get_registration_with_document_url(&self, document_url: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_registration_with_document_url_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                document_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_registration_with_document_url_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            document_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(document_url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let document_url =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(document_url);
                __widl_f_get_registration_with_document_url_ServiceWorkerContainer(
                    self_,
                    document_url,
                )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_registrations_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `getRegistrations()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistrations)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn get_registrations(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_registrations_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_registrations_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_registrations_ServiceWorkerContainer(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_scope_for_url_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `getScopeForUrl()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getScopeForUrl)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn get_scope_for_url(&self, url: &str) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_scope_for_url_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_scope_for_url_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_get_scope_for_url_ServiceWorkerContainer(self_, url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_register_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `register()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn register(&self, script_url: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_register_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_register_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(script_url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let script_url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(script_url);
                __widl_f_register_ServiceWorkerContainer(self_, script_url)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RegistrationOptions", feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_register_with_options_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&RegistrationOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "RegistrationOptions", feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `register()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)\n\n*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn register_with_options(
        &self,
        script_url: &str,
        options: &RegistrationOptions,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "RegistrationOptions", feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_register_with_options_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&RegistrationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_register_with_options_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&RegistrationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(script_url);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let script_url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(script_url);
                let options =
                    <&RegistrationOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_register_with_options_ServiceWorkerContainer(self_, script_url, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_controller_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <Option<ServiceWorker> as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `controller` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/controller)\n\n*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn controller(&self) -> Option<ServiceWorker> {
        #[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_controller_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<ServiceWorker> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_controller_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<ServiceWorker> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_controller_ServiceWorkerContainer(self_)
            };
            <Option<ServiceWorker> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `ready` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/ready)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn ready(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_ServiceWorkerContainer(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncontrollerchange_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `oncontrollerchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn oncontrollerchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncontrollerchange_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncontrollerchange_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncontrollerchange_ServiceWorkerContainer(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncontrollerchange_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `oncontrollerchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn set_oncontrollerchange(&self, oncontrollerchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncontrollerchange_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncontrollerchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncontrollerchange_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncontrollerchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(oncontrollerchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncontrollerchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncontrollerchange,
                    );
                __widl_f_set_oncontrollerchange_ServiceWorkerContainer(self_, oncontrollerchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onerror)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_ServiceWorkerContainer(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onerror)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_ServiceWorkerContainer(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessage_ServiceWorkerContainer(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_ServiceWorkerContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerContainer as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerContainer {
    #[cfg(all(feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_ServiceWorkerContainer(
                self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_ServiceWorkerContainer(
            self_: <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_ServiceWorkerContainer(self_, onmessage)
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
pub static __WASM_BINDGEN_GENERATED_843d636b5feae1cd: [u8; 1765usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA3\x06\0\0\0\0\x0F\0\0\x02\x16ServiceWorkerContainer(__widl_instanceof_ServiceWorkerContainer\0\0\0\00__widl_f_get_registration_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\0\x01\x01\x05self_\x0FgetRegistration\0\0\0B__widl_f_get_registration_with_document_url_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\0\x01\x02\x05self_\x0Cdocument_url\x0FgetRegistration\0\0\01__widl_f_get_registrations_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\0\x01\x01\x05self_\x10getRegistrations\0\0\01__widl_f_get_scope_for_url_ServiceWorkerContainer\x01\0\0\x01\x16ServiceWorkerContainer\x01\0\0\x01\x02\x05self_\x03url\x0EgetScopeForUrl\0\0\0(__widl_f_register_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\0\x01\x02\x05self_\nscript_url\x08register\0\0\05__widl_f_register_with_options_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\0\x01\x03\x05self_\nscript_url\x07options\x08register\0\0\0*__widl_f_controller_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\x01\ncontroller\x01\x01\x05self_\ncontroller\0\0\0%__widl_f_ready_ServiceWorkerContainer\x01\0\0\x01\x16ServiceWorkerContainer\x01\0\x01\x05ready\x01\x01\x05self_\x05ready\0\0\02__widl_f_oncontrollerchange_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\x01\x12oncontrollerchange\x01\x01\x05self_\x12oncontrollerchange\0\0\06__widl_f_set_oncontrollerchange_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\x02\x12oncontrollerchange\x01\x02\x05self_\x12oncontrollerchange\x12oncontrollerchange\0\0\0'__widl_f_onerror_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0+__widl_f_set_onerror_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0)__widl_f_onmessage_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0-__widl_f_set_onmessage_ServiceWorkerContainer\0\0\0\x01\x16ServiceWorkerContainer\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
