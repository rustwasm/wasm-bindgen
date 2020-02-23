use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `NotificationEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent)\n\n*This API requires the following crate features to be activated: `NotificationEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct NotificationEvent {
    obj: ExtendableEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_NotificationEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for NotificationEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
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
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for NotificationEvent {
        type Target = ExtendableEvent;
        #[inline]
        fn deref(&self) -> &ExtendableEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for NotificationEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for NotificationEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a NotificationEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for NotificationEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            NotificationEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for NotificationEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a NotificationEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for NotificationEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<NotificationEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(NotificationEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for NotificationEvent {
        #[inline]
        fn from(obj: JsValue) -> NotificationEvent {
            NotificationEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for NotificationEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<NotificationEvent> for NotificationEvent {
        #[inline]
        fn as_ref(&self) -> &NotificationEvent {
            self
        }
    }
    impl From<NotificationEvent> for JsValue {
        #[inline]
        fn from(obj: NotificationEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for NotificationEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_NotificationEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_NotificationEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_NotificationEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            NotificationEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const NotificationEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<NotificationEvent> for ExtendableEvent {
    #[inline]
    fn from(obj: NotificationEvent) -> ExtendableEvent {
        use wasm_bindgen::JsCast;
        ExtendableEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<ExtendableEvent> for NotificationEvent {
    #[inline]
    fn as_ref(&self) -> &ExtendableEvent {
        use wasm_bindgen::JsCast;
        ExtendableEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<NotificationEvent> for Event {
    #[inline]
    fn from(obj: NotificationEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for NotificationEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<NotificationEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: NotificationEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for NotificationEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "NotificationEvent", feature = "NotificationEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_NotificationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&NotificationEventInit as WasmDescribe>::describe();
    <NotificationEvent as WasmDescribe>::describe();
}
impl NotificationEvent {
    #[cfg(all(feature = "NotificationEvent", feature = "NotificationEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new NotificationEvent(..)` constructor, creating a new instance of `NotificationEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent/NotificationEvent)\n\n*This API requires the following crate features to be activated: `NotificationEvent`, `NotificationEventInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        type_: &str,
        event_init_dict: &NotificationEventInit,
    ) -> Result<NotificationEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "NotificationEvent", feature = "NotificationEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_NotificationEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & NotificationEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <NotificationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_NotificationEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&NotificationEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NotificationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            drop(event_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let event_init_dict =
                    <&NotificationEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_NotificationEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<NotificationEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Notification", feature = "NotificationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_notification_NotificationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NotificationEvent as WasmDescribe>::describe();
    <Notification as WasmDescribe>::describe();
}
impl NotificationEvent {
    #[cfg(all(feature = "Notification", feature = "NotificationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `notification` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent/notification)\n\n*This API requires the following crate features to be activated: `Notification`, `NotificationEvent`*"]
    #[allow(clippy::all)]
    pub fn notification(&self) -> Notification {
        #[cfg(all(feature = "Notification", feature = "NotificationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_notification_NotificationEvent(
                self_: <&NotificationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Notification as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_notification_NotificationEvent(
            self_: <&NotificationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Notification as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&NotificationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_notification_NotificationEvent(self_)
            };
            <Notification as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_60a42b77d80d1da8: [u8; 350usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1C\x01\0\0\0\0\x03\0\0\x02\x11NotificationEvent#__widl_instanceof_NotificationEvent\0\0\0\0\x1E__widl_f_new_NotificationEvent\x01\0\0\x01\x11NotificationEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0'__widl_f_notification_NotificationEvent\0\0\0\x01\x11NotificationEvent\x01\0\x01\x0Cnotification\x01\x01\x05self_\x0Cnotification\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
