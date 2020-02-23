use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ServiceWorkerRegistration` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ServiceWorkerRegistration {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ServiceWorkerRegistration: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ServiceWorkerRegistration {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
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
            inform(82u32);
            inform(101u32);
            inform(103u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
            inform(114u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for ServiceWorkerRegistration {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for ServiceWorkerRegistration {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ServiceWorkerRegistration {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ServiceWorkerRegistration {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ServiceWorkerRegistration {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ServiceWorkerRegistration {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ServiceWorkerRegistration {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ServiceWorkerRegistration {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ServiceWorkerRegistration {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ServiceWorkerRegistration>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ServiceWorkerRegistration {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ServiceWorkerRegistration {
        #[inline]
        fn from(obj: JsValue) -> ServiceWorkerRegistration {
            ServiceWorkerRegistration { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ServiceWorkerRegistration {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ServiceWorkerRegistration> for ServiceWorkerRegistration {
        #[inline]
        fn as_ref(&self) -> &ServiceWorkerRegistration {
            self
        }
    }
    impl From<ServiceWorkerRegistration> for JsValue {
        #[inline]
        fn from(obj: ServiceWorkerRegistration) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ServiceWorkerRegistration {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ServiceWorkerRegistration(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ServiceWorkerRegistration(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ServiceWorkerRegistration(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ServiceWorkerRegistration { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ServiceWorkerRegistration) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ServiceWorkerRegistration> for EventTarget {
    #[inline]
    fn from(obj: ServiceWorkerRegistration) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ServiceWorkerRegistration {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ServiceWorkerRegistration> for ::js_sys::Object {
    #[inline]
    fn from(obj: ServiceWorkerRegistration) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ServiceWorkerRegistration {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_notifications_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `getNotifications()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn get_notifications(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_notifications_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_notifications_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_get_notifications_ServiceWorkerRegistration(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "GetNotificationOptions",
    feature = "ServiceWorkerRegistration",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_notifications_with_filter_ServiceWorkerRegistration(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <&GetNotificationOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(
        feature = "GetNotificationOptions",
        feature = "ServiceWorkerRegistration",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getNotifications()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)\n\n*This API requires the following crate features to be activated: `GetNotificationOptions`, `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn get_notifications_with_filter(
        &self,
        filter: &GetNotificationOptions,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "GetNotificationOptions",
            feature = "ServiceWorkerRegistration",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_notifications_with_filter_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                filter: <&GetNotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_notifications_with_filter_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            filter: <&GetNotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(filter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let filter =
                    <&GetNotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        filter,
                    );
                __widl_f_get_notifications_with_filter_ServiceWorkerRegistration(self_, filter)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_show_notification_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `showNotification()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn show_notification(
        &self,
        title: &str,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_show_notification_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_show_notification_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(title);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                __widl_f_show_notification_ServiceWorkerRegistration(self_, title)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "NotificationOptions", feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_show_notification_with_options_ServiceWorkerRegistration(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&NotificationOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "NotificationOptions", feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `showNotification()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)\n\n*This API requires the following crate features to be activated: `NotificationOptions`, `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn show_notification_with_options(
        &self,
        title: &str,
        options: &NotificationOptions,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "NotificationOptions", feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_show_notification_with_options_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&NotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_show_notification_with_options_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&NotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(title);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                let options =
                    <&NotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_show_notification_with_options_ServiceWorkerRegistration(
                    self_, title, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unregister_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `unregister()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/unregister)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn unregister(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unregister_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unregister_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_unregister_ServiceWorkerRegistration(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_update_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `update()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/update)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn update(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_update_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_update_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_update_ServiceWorkerRegistration(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_installing_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <Option<ServiceWorker> as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `installing` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/installing)\n\n*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn installing(&self) -> Option<ServiceWorker> {
        #[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_installing_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<ServiceWorker> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_installing_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_installing_ServiceWorkerRegistration(self_)
            };
            <Option<ServiceWorker> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_waiting_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <Option<ServiceWorker> as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `waiting` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/waiting)\n\n*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn waiting(&self) -> Option<ServiceWorker> {
        #[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_waiting_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<ServiceWorker> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_waiting_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_waiting_ServiceWorkerRegistration(self_)
            };
            <Option<ServiceWorker> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_active_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <Option<ServiceWorker> as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `active` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/active)\n\n*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn active(&self) -> Option<ServiceWorker> {
        #[cfg(all(feature = "ServiceWorker", feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_active_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<ServiceWorker> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_active_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_active_ServiceWorkerRegistration(self_)
            };
            <Option<ServiceWorker> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scope_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `scope` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/scope)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn scope(&self) -> String {
        #[cfg(all(feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scope_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scope_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_scope_ServiceWorkerRegistration(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "ServiceWorkerRegistration",
    feature = "ServiceWorkerUpdateViaCache",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_update_via_cache_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <ServiceWorkerUpdateViaCache as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(
        feature = "ServiceWorkerRegistration",
        feature = "ServiceWorkerUpdateViaCache",
    ))]
    #[allow(bad_style)]
    #[doc = "The `updateViaCache` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/updateViaCache)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`, `ServiceWorkerUpdateViaCache`*"]
    #[allow(clippy::all)]
    pub fn update_via_cache(&self) -> Result<ServiceWorkerUpdateViaCache, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ServiceWorkerRegistration",
            feature = "ServiceWorkerUpdateViaCache",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_update_via_cache_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ServiceWorkerUpdateViaCache as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_update_via_cache_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ServiceWorkerUpdateViaCache as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_update_via_cache_ServiceWorkerRegistration(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ServiceWorkerUpdateViaCache as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onupdatefound_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `onupdatefound` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn onupdatefound(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onupdatefound_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onupdatefound_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onupdatefound_ServiceWorkerRegistration(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onupdatefound_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `onupdatefound` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn set_onupdatefound(&self, onupdatefound: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onupdatefound_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onupdatefound : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onupdatefound_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onupdatefound: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onupdatefound);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onupdatefound =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onupdatefound,
                    );
                __widl_f_set_onupdatefound_ServiceWorkerRegistration(self_, onupdatefound)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PushManager", feature = "ServiceWorkerRegistration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_push_manager_ServiceWorkerRegistration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ServiceWorkerRegistration as WasmDescribe>::describe();
    <PushManager as WasmDescribe>::describe();
}
impl ServiceWorkerRegistration {
    #[cfg(all(feature = "PushManager", feature = "ServiceWorkerRegistration",))]
    #[allow(bad_style)]
    #[doc = "The `pushManager` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/pushManager)\n\n*This API requires the following crate features to be activated: `PushManager`, `ServiceWorkerRegistration`*"]
    #[allow(clippy::all)]
    pub fn push_manager(&self) -> Result<PushManager, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushManager", feature = "ServiceWorkerRegistration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_push_manager_ServiceWorkerRegistration(
                self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PushManager as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_push_manager_ServiceWorkerRegistration(
            self_: <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PushManager as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ServiceWorkerRegistration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_push_manager_ServiceWorkerRegistration(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PushManager as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_3689cf79eb51fea2: [u8; 1795usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC1\x06\0\0\0\0\x0F\0\0\x02\x19ServiceWorkerRegistration+__widl_instanceof_ServiceWorkerRegistration\0\0\0\04__widl_f_get_notifications_ServiceWorkerRegistration\x01\0\0\x01\x19ServiceWorkerRegistration\x01\0\0\x01\x01\x05self_\x10getNotifications\0\0\0@__widl_f_get_notifications_with_filter_ServiceWorkerRegistration\x01\0\0\x01\x19ServiceWorkerRegistration\x01\0\0\x01\x02\x05self_\x06filter\x10getNotifications\0\0\04__widl_f_show_notification_ServiceWorkerRegistration\x01\0\0\x01\x19ServiceWorkerRegistration\x01\0\0\x01\x02\x05self_\x05title\x10showNotification\0\0\0A__widl_f_show_notification_with_options_ServiceWorkerRegistration\x01\0\0\x01\x19ServiceWorkerRegistration\x01\0\0\x01\x03\x05self_\x05title\x07options\x10showNotification\0\0\0-__widl_f_unregister_ServiceWorkerRegistration\x01\0\0\x01\x19ServiceWorkerRegistration\x01\0\0\x01\x01\x05self_\nunregister\0\0\0)__widl_f_update_ServiceWorkerRegistration\x01\0\0\x01\x19ServiceWorkerRegistration\x01\0\0\x01\x01\x05self_\x06update\0\0\0-__widl_f_installing_ServiceWorkerRegistration\0\0\0\x01\x19ServiceWorkerRegistration\x01\0\x01\ninstalling\x01\x01\x05self_\ninstalling\0\0\0*__widl_f_waiting_ServiceWorkerRegistration\0\0\0\x01\x19ServiceWorkerRegistration\x01\0\x01\x07waiting\x01\x01\x05self_\x07waiting\0\0\0)__widl_f_active_ServiceWorkerRegistration\0\0\0\x01\x19ServiceWorkerRegistration\x01\0\x01\x06active\x01\x01\x05self_\x06active\0\0\0(__widl_f_scope_ServiceWorkerRegistration\0\0\0\x01\x19ServiceWorkerRegistration\x01\0\x01\x05scope\x01\x01\x05self_\x05scope\0\0\03__widl_f_update_via_cache_ServiceWorkerRegistration\x01\0\0\x01\x19ServiceWorkerRegistration\x01\0\x01\x0EupdateViaCache\x01\x01\x05self_\x0EupdateViaCache\0\0\00__widl_f_onupdatefound_ServiceWorkerRegistration\0\0\0\x01\x19ServiceWorkerRegistration\x01\0\x01\ronupdatefound\x01\x01\x05self_\ronupdatefound\0\0\04__widl_f_set_onupdatefound_ServiceWorkerRegistration\0\0\0\x01\x19ServiceWorkerRegistration\x01\0\x02\ronupdatefound\x01\x02\x05self_\ronupdatefound\ronupdatefound\0\0\0/__widl_f_push_manager_ServiceWorkerRegistration\x01\0\0\x01\x19ServiceWorkerRegistration\x01\0\x01\x0BpushManager\x01\x01\x05self_\x0BpushManager\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
