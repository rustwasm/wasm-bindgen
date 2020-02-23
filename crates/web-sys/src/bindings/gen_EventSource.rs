use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `EventSource` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct EventSource {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_EventSource: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for EventSource {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(83u32);
            inform(111u32);
            inform(117u32);
            inform(114u32);
            inform(99u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for EventSource {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for EventSource {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for EventSource {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a EventSource {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for EventSource {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            EventSource {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for EventSource {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a EventSource {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for EventSource {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<EventSource>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(EventSource {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for EventSource {
        #[inline]
        fn from(obj: JsValue) -> EventSource {
            EventSource { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for EventSource {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<EventSource> for EventSource {
        #[inline]
        fn as_ref(&self) -> &EventSource {
            self
        }
    }
    impl From<EventSource> for JsValue {
        #[inline]
        fn from(obj: EventSource) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for EventSource {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_EventSource(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_EventSource(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_EventSource(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            EventSource { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const EventSource) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<EventSource> for EventTarget {
    #[inline]
    fn from(obj: EventSource) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for EventSource {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<EventSource> for ::js_sys::Object {
    #[inline]
    fn from(obj: EventSource) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for EventSource {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "EventSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <EventSource as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource",))]
    #[allow(bad_style)]
    #[doc = "The `new EventSource(..)` constructor, creating a new instance of `EventSource`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/EventSource)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    #[allow(clippy::all)]
    pub fn new(url: &str) -> Result<EventSource, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_EventSource(
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <EventSource as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_EventSource(
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <EventSource as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_new_EventSource(url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<EventSource as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "EventSource", feature = "EventSourceInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_source_init_dict_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&EventSourceInit as WasmDescribe>::describe();
    <EventSource as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource", feature = "EventSourceInit",))]
    #[allow(bad_style)]
    #[doc = "The `new EventSource(..)` constructor, creating a new instance of `EventSource`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/EventSource)\n\n*This API requires the following crate features to be activated: `EventSource`, `EventSourceInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_source_init_dict(
        url: &str,
        event_source_init_dict: &EventSourceInit,
    ) -> Result<EventSource, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "EventSource", feature = "EventSourceInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_source_init_dict_EventSource(
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_source_init_dict : < & EventSourceInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <EventSource as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_source_init_dict_EventSource(
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_source_init_dict: <&EventSourceInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <EventSource as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(url);
            drop(event_source_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let event_source_init_dict =
                    <&EventSourceInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_source_init_dict,
                    );
                __widl_f_new_with_event_source_init_dict_EventSource(url, event_source_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<EventSource as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "EventSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&EventSource as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/close)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "EventSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_EventSource(
                self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_EventSource(
            self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_EventSource(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "EventSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_url_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&EventSource as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource",))]
    #[allow(bad_style)]
    #[doc = "The `url` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/url)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    #[allow(clippy::all)]
    pub fn url(&self) -> String {
        #[cfg(all(feature = "EventSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_url_EventSource(
                self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_url_EventSource(
            self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_url_EventSource(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "EventSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_with_credentials_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&EventSource as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource",))]
    #[allow(bad_style)]
    #[doc = "The `withCredentials` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/withCredentials)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    #[allow(clippy::all)]
    pub fn with_credentials(&self) -> bool {
        #[cfg(all(feature = "EventSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_with_credentials_EventSource(
                self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_with_credentials_EventSource(
            self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_with_credentials_EventSource(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "EventSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&EventSource as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/readyState)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> u16 {
        #[cfg(all(feature = "EventSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_EventSource(
                self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_EventSource(
            self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_EventSource(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "EventSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onopen_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&EventSource as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource",))]
    #[allow(bad_style)]
    #[doc = "The `onopen` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    #[allow(clippy::all)]
    pub fn onopen(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "EventSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onopen_EventSource(
                self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onopen_EventSource(
            self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onopen_EventSource(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "EventSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onopen_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&EventSource as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource",))]
    #[allow(bad_style)]
    #[doc = "The `onopen` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    #[allow(clippy::all)]
    pub fn set_onopen(&self, onopen: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "EventSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onopen_EventSource(
                self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onopen: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onopen_EventSource(
            self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onopen: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onopen);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onopen =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onopen,
                    );
                __widl_f_set_onopen_EventSource(self_, onopen)
            };
            ()
        }
    }
}
#[cfg(all(feature = "EventSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&EventSource as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "EventSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_EventSource(
                self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_EventSource(
            self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessage_EventSource(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "EventSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&EventSource as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "EventSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_EventSource(
                self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_EventSource(
            self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_EventSource(self_, onmessage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "EventSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&EventSource as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "EventSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_EventSource(
                self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_EventSource(
            self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_EventSource(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "EventSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_EventSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&EventSource as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl EventSource {
    #[cfg(all(feature = "EventSource",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)\n\n*This API requires the following crate features to be activated: `EventSource`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "EventSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_EventSource(
                self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_EventSource(
            self_: <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&EventSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_EventSource(self_, onerror)
            };
            ()
        }
    }
}
impl EventSource {
    pub const CONNECTING: u16 = 0i64 as u16;
}
impl EventSource {
    pub const OPEN: u16 = 1u64 as u16;
}
impl EventSource {
    pub const CLOSED: u16 = 2u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_761a8e0a77447cb9: [u8; 1116usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1A\x04\0\0\0\0\r\0\0\x02\x0BEventSource\x1D__widl_instanceof_EventSource\0\0\0\0\x18__widl_f_new_EventSource\x01\0\0\x01\x0BEventSource\0\x01\x01\x03url\x03new\0\0\04__widl_f_new_with_event_source_init_dict_EventSource\x01\0\0\x01\x0BEventSource\0\x01\x02\x03url\x16event_source_init_dict\x03new\0\0\0\x1A__widl_f_close_EventSource\0\0\0\x01\x0BEventSource\x01\0\0\x01\x01\x05self_\x05close\0\0\0\x18__widl_f_url_EventSource\0\0\0\x01\x0BEventSource\x01\0\x01\x03url\x01\x01\x05self_\x03url\0\0\0%__widl_f_with_credentials_EventSource\0\0\0\x01\x0BEventSource\x01\0\x01\x0FwithCredentials\x01\x01\x05self_\x0FwithCredentials\0\0\0 __widl_f_ready_state_EventSource\0\0\0\x01\x0BEventSource\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0\x1B__widl_f_onopen_EventSource\0\0\0\x01\x0BEventSource\x01\0\x01\x06onopen\x01\x01\x05self_\x06onopen\0\0\0\x1F__widl_f_set_onopen_EventSource\0\0\0\x01\x0BEventSource\x01\0\x02\x06onopen\x01\x02\x05self_\x06onopen\x06onopen\0\0\0\x1E__widl_f_onmessage_EventSource\0\0\0\x01\x0BEventSource\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0\"__widl_f_set_onmessage_EventSource\0\0\0\x01\x0BEventSource\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0\x1C__widl_f_onerror_EventSource\0\0\0\x01\x0BEventSource\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0 __widl_f_set_onerror_EventSource\0\0\0\x01\x0BEventSource\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
