use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ServiceWorker` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ServiceWorker {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ServiceWorker: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ServiceWorker {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
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
        }
    }
    impl core::ops::Deref for ServiceWorker {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for ServiceWorker {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ServiceWorker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ServiceWorker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ServiceWorker {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ServiceWorker {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ServiceWorker {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ServiceWorker {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ServiceWorker {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ServiceWorker>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ServiceWorker {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ServiceWorker {
        #[inline]
        fn from(obj: JsValue) -> ServiceWorker {
            ServiceWorker { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ServiceWorker {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ServiceWorker> for ServiceWorker {
        #[inline]
        fn as_ref(&self) -> &ServiceWorker {
            self
        }
    }
    impl From<ServiceWorker> for JsValue {
        #[inline]
        fn from(obj: ServiceWorker) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ServiceWorker {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ServiceWorker(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ServiceWorker(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ServiceWorker(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ServiceWorker { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ServiceWorker) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ServiceWorker> for EventTarget {
    #[inline]
    fn from(obj: ServiceWorker) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ServiceWorker {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ServiceWorker> for ::js_sys::Object {
    #[inline]
    fn from(obj: ServiceWorker) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ServiceWorker {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ServiceWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_post_message_ServiceWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorker as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorker {
    #[cfg(all(feature = "ServiceWorker",))]
    #[allow(bad_style)]
    #[doc = "The `postMessage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/postMessage)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    #[allow(clippy::all)]
    pub fn post_message(
        &self,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ServiceWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_post_message_ServiceWorker(
                self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_post_message_ServiceWorker(
            self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let message =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        message,
                    );
                __widl_f_post_message_ServiceWorker(self_, message)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ServiceWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_post_message_with_transferable_ServiceWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ServiceWorker as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorker {
    #[cfg(all(feature = "ServiceWorker",))]
    #[allow(bad_style)]
    #[doc = "The `postMessage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/postMessage)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    #[allow(clippy::all)]
    pub fn post_message_with_transferable(
        &self,
        message: &::wasm_bindgen::JsValue,
        transferable: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ServiceWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_post_message_with_transferable_ServiceWorker(
                self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                transferable: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_post_message_with_transferable_ServiceWorker(
            self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            message: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transferable: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(message);
            drop(transferable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let message =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        message,
                    );
                let transferable =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        transferable,
                    );
                __widl_f_post_message_with_transferable_ServiceWorker(self_, message, transferable)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ServiceWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_script_url_ServiceWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorker as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl ServiceWorker {
    #[cfg(all(feature = "ServiceWorker",))]
    #[allow(bad_style)]
    #[doc = "The `scriptURL` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/scriptURL)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    #[allow(clippy::all)]
    pub fn script_url(&self) -> String {
        #[cfg(all(feature = "ServiceWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_script_url_ServiceWorker(
                self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_script_url_ServiceWorker(
            self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_script_url_ServiceWorker(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_state_ServiceWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorker as WasmDescribe>::describe();
    <ServiceWorkerState as WasmDescribe>::describe();
}
impl ServiceWorker {
    #[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerState",))]
    #[allow(bad_style)]
    #[doc = "The `state` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/state)\n\n*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerState`*"]
    #[allow(clippy::all)]
    pub fn state(&self) -> ServiceWorkerState {
        #[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_state_ServiceWorker(
                self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ServiceWorkerState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_state_ServiceWorker(
            self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ServiceWorkerState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_state_ServiceWorker(self_)
            };
            <ServiceWorkerState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstatechange_ServiceWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorker as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorker {
    #[cfg(all(feature = "ServiceWorker",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onstatechange)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    #[allow(clippy::all)]
    pub fn onstatechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstatechange_ServiceWorker(
                self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstatechange_ServiceWorker(
            self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstatechange_ServiceWorker(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstatechange_ServiceWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorker as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorker {
    #[cfg(all(feature = "ServiceWorker",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onstatechange)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    #[allow(clippy::all)]
    pub fn set_onstatechange(&self, onstatechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstatechange_ServiceWorker(
                self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstatechange_ServiceWorker(
            self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstatechange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstatechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstatechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstatechange,
                    );
                __widl_f_set_onstatechange_ServiceWorker(self_, onstatechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ServiceWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_ServiceWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorker as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorker {
    #[cfg(all(feature = "ServiceWorker",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onerror)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_ServiceWorker(
                self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_ServiceWorker(
            self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_ServiceWorker(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_ServiceWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorker as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorker {
    #[cfg(all(feature = "ServiceWorker",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onerror)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_ServiceWorker(
                self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_ServiceWorker(
            self_: <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&ServiceWorker as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_ServiceWorker(self_, onerror)
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
pub static __WASM_BINDGEN_GENERATED_33d6ac73308c720f: [u8; 903usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}E\x03\0\0\0\0\t\0\0\x02\rServiceWorker\x1F__widl_instanceof_ServiceWorker\0\0\0\0#__widl_f_post_message_ServiceWorker\x01\0\0\x01\rServiceWorker\x01\0\0\x01\x02\x05self_\x07message\x0BpostMessage\0\0\05__widl_f_post_message_with_transferable_ServiceWorker\x01\0\0\x01\rServiceWorker\x01\0\0\x01\x03\x05self_\x07message\x0Ctransferable\x0BpostMessage\0\0\0!__widl_f_script_url_ServiceWorker\0\0\0\x01\rServiceWorker\x01\0\x01\tscriptURL\x01\x01\x05self_\tscriptURL\0\0\0\x1C__widl_f_state_ServiceWorker\0\0\0\x01\rServiceWorker\x01\0\x01\x05state\x01\x01\x05self_\x05state\0\0\0$__widl_f_onstatechange_ServiceWorker\0\0\0\x01\rServiceWorker\x01\0\x01\ronstatechange\x01\x01\x05self_\ronstatechange\0\0\0(__widl_f_set_onstatechange_ServiceWorker\0\0\0\x01\rServiceWorker\x01\0\x02\ronstatechange\x01\x02\x05self_\ronstatechange\ronstatechange\0\0\0\x1E__widl_f_onerror_ServiceWorker\0\0\0\x01\rServiceWorker\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\"__widl_f_set_onerror_ServiceWorker\0\0\0\x01\rServiceWorker\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
