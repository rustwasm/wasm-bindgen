use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Notification` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification)\n\n*This API requires the following crate features to be activated: `Notification`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Notification {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Notification: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Notification {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(78u32);
            inform(111u32);
            inform(116u32);
            inform(105u32);
            inform(102u32);
            inform(105u32);
            inform(99u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for Notification {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for Notification {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Notification {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Notification {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Notification {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Notification {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Notification {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Notification {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Notification {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Notification>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Notification {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Notification {
        #[inline]
        fn from(obj: JsValue) -> Notification {
            Notification { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Notification {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Notification> for Notification {
        #[inline]
        fn as_ref(&self) -> &Notification {
            self
        }
    }
    impl From<Notification> for JsValue {
        #[inline]
        fn from(obj: Notification) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Notification {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Notification(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Notification(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Notification(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Notification { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Notification) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Notification> for EventTarget {
    #[inline]
    fn from(obj: Notification) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for Notification {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Notification> for ::js_sys::Object {
    #[inline]
    fn from(obj: Notification) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Notification {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <Notification as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `new Notification(..)` constructor, creating a new instance of `Notification`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/Notification)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn new(title: &str) -> Result<Notification, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Notification(
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Notification as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Notification(
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Notification as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(title);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                __widl_f_new_Notification(title)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Notification as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Notification", feature = "NotificationOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&NotificationOptions as WasmDescribe>::describe();
    <Notification as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification", feature = "NotificationOptions",))]
    #[allow(bad_style)]
    #[doc = "The `new Notification(..)` constructor, creating a new instance of `Notification`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/Notification)\n\n*This API requires the following crate features to be activated: `Notification`, `NotificationOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        title: &str,
        options: &NotificationOptions,
    ) -> Result<Notification, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Notification", feature = "NotificationOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_Notification(
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&NotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Notification as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_Notification(
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&NotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Notification as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(title);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                let options =
                    <&NotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_Notification(title, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Notification as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/close)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_Notification(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `get()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/get)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn get() -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_Notification(
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_Notification(
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_get_Notification() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GetNotificationOptions", feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_filter_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GetNotificationOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "GetNotificationOptions", feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `get()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/get)\n\n*This API requires the following crate features to be activated: `GetNotificationOptions`, `Notification`*"]
    #[allow(clippy::all)]
    pub fn get_with_filter(
        filter: &GetNotificationOptions,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GetNotificationOptions", feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_filter_Notification(
                filter: <&GetNotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_filter_Notification(
            filter: <&GetNotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(filter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let filter =
                    <&GetNotificationOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        filter,
                    );
                __widl_f_get_with_filter_Notification(filter)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_permission_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `requestPermission()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requestPermission)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn request_permission() -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_permission_Notification(
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_permission_Notification(
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_request_permission_Notification() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_permission_with_permission_callback_Notification(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::js_sys::Function as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `requestPermission()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requestPermission)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn request_permission_with_permission_callback(
        permission_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_permission_with_permission_callback_Notification(
                permission_callback : < & :: js_sys :: Function as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_permission_with_permission_callback_Notification(
            permission_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(permission_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let permission_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        permission_callback,
                    );
                __widl_f_request_permission_with_permission_callback_Notification(
                    permission_callback,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Notification", feature = "NotificationPermission",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_permission_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <NotificationPermission as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification", feature = "NotificationPermission",))]
    #[allow(bad_style)]
    #[doc = "The `permission` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/permission)\n\n*This API requires the following crate features to be activated: `Notification`, `NotificationPermission`*"]
    #[allow(clippy::all)]
    pub fn permission() -> NotificationPermission {
        #[cfg(all(feature = "Notification", feature = "NotificationPermission",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_permission_Notification(
            ) -> <NotificationPermission as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_permission_Notification(
        ) -> <NotificationPermission as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_permission_Notification() };
            <NotificationPermission as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclick_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `onclick` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclick)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn onclick(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclick_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclick_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclick_Notification(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclick_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Notification as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `onclick` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclick)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn set_onclick(&self, onclick: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclick_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclick_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onclick);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclick =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclick,
                    );
                __widl_f_set_onclick_Notification(self_, onclick)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onshow_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `onshow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onshow)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn onshow(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onshow_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onshow_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onshow_Notification(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onshow_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Notification as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `onshow` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onshow)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn set_onshow(&self, onshow: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onshow_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onshow: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onshow_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onshow: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onshow);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onshow =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onshow,
                    );
                __widl_f_set_onshow_Notification(self_, onshow)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onerror)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_Notification(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Notification as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onerror)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_Notification(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclose_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclose)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn onclose(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclose_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclose_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclose_Notification(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclose_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Notification as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclose)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn set_onclose(&self, onclose: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclose_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclose_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onclose);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclose =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclose,
                    );
                __widl_f_set_onclose_Notification(self_, onclose)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_title_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `title` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/title)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn title(&self) -> String {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_title_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_title_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_title_Notification(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification", feature = "NotificationDirection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dir_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <NotificationDirection as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification", feature = "NotificationDirection",))]
    #[allow(bad_style)]
    #[doc = "The `dir` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/dir)\n\n*This API requires the following crate features to be activated: `Notification`, `NotificationDirection`*"]
    #[allow(clippy::all)]
    pub fn dir(&self) -> NotificationDirection {
        #[cfg(all(feature = "Notification", feature = "NotificationDirection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NotificationDirection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NotificationDirection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dir_Notification(self_)
            };
            <NotificationDirection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lang_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `lang` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/lang)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn lang(&self) -> Option<String> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lang_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lang_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_lang_Notification(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_body_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `body` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/body)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn body(&self) -> Option<String> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_body_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_body_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_body_Notification(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tag_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `tag` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/tag)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn tag(&self) -> Option<String> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tag_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tag_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_tag_Notification(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_icon_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `icon` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/icon)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn icon(&self) -> Option<String> {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_icon_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_icon_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_icon_Notification(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_require_interaction_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `requireInteraction` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requireInteraction)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn require_interaction(&self) -> bool {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_require_interaction_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_require_interaction_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_require_interaction_Notification(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Notification",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_Notification() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Notification as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl Notification {
    #[cfg(all(feature = "Notification",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/data)\n\n*This API requires the following crate features to be activated: `Notification`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "Notification",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_Notification(
                self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_Notification(
            self_: <&Notification as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Notification as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_Notification(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f069eea0b9e20580: [u8; 2020usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA2\x07\0\0\0\0\x19\0\0\x02\x0CNotification\x1E__widl_instanceof_Notification\0\0\0\0\x19__widl_f_new_Notification\x01\0\0\x01\x0CNotification\0\x01\x01\x05title\x03new\0\0\0&__widl_f_new_with_options_Notification\x01\0\0\x01\x0CNotification\0\x01\x02\x05title\x07options\x03new\0\0\0\x1B__widl_f_close_Notification\0\0\0\x01\x0CNotification\x01\0\0\x01\x01\x05self_\x05close\0\0\0\x19__widl_f_get_Notification\x01\0\0\x01\x0CNotification\x01\x01\0\x01\0\x03get\0\0\0%__widl_f_get_with_filter_Notification\x01\0\0\x01\x0CNotification\x01\x01\0\x01\x01\x06filter\x03get\0\0\0(__widl_f_request_permission_Notification\x01\0\0\x01\x0CNotification\x01\x01\0\x01\0\x11requestPermission\0\0\0A__widl_f_request_permission_with_permission_callback_Notification\x01\0\0\x01\x0CNotification\x01\x01\0\x01\x01\x13permission_callback\x11requestPermission\0\0\0 __widl_f_permission_Notification\0\0\0\x01\x0CNotification\x01\x01\x01\npermission\x01\0\npermission\0\0\0\x1D__widl_f_onclick_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x07onclick\x01\x01\x05self_\x07onclick\0\0\0!__widl_f_set_onclick_Notification\0\0\0\x01\x0CNotification\x01\0\x02\x07onclick\x01\x02\x05self_\x07onclick\x07onclick\0\0\0\x1C__widl_f_onshow_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x06onshow\x01\x01\x05self_\x06onshow\0\0\0 __widl_f_set_onshow_Notification\0\0\0\x01\x0CNotification\x01\0\x02\x06onshow\x01\x02\x05self_\x06onshow\x06onshow\0\0\0\x1D__widl_f_onerror_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0!__widl_f_set_onerror_Notification\0\0\0\x01\x0CNotification\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\x1D__widl_f_onclose_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x07onclose\x01\x01\x05self_\x07onclose\0\0\0!__widl_f_set_onclose_Notification\0\0\0\x01\x0CNotification\x01\0\x02\x07onclose\x01\x02\x05self_\x07onclose\x07onclose\0\0\0\x1B__widl_f_title_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x05title\x01\x01\x05self_\x05title\0\0\0\x19__widl_f_dir_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x03dir\x01\x01\x05self_\x03dir\0\0\0\x1A__widl_f_lang_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x04lang\x01\x01\x05self_\x04lang\0\0\0\x1A__widl_f_body_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x04body\x01\x01\x05self_\x04body\0\0\0\x19__widl_f_tag_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x03tag\x01\x01\x05self_\x03tag\0\0\0\x1A__widl_f_icon_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x04icon\x01\x01\x05self_\x04icon\0\0\0)__widl_f_require_interaction_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x12requireInteraction\x01\x01\x05self_\x12requireInteraction\0\0\0\x1A__widl_f_data_Notification\0\0\0\x01\x0CNotification\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
