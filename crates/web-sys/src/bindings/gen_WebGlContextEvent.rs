use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WebGLContextEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent)\n\n*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebGlContextEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebGlContextEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebGlContextEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(87u32);
            inform(101u32);
            inform(98u32);
            inform(71u32);
            inform(76u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for WebGlContextEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebGlContextEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebGlContextEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebGlContextEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebGlContextEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebGlContextEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebGlContextEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebGlContextEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebGlContextEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebGlContextEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebGlContextEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebGlContextEvent {
        #[inline]
        fn from(obj: JsValue) -> WebGlContextEvent {
            WebGlContextEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebGlContextEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebGlContextEvent> for WebGlContextEvent {
        #[inline]
        fn as_ref(&self) -> &WebGlContextEvent {
            self
        }
    }
    impl From<WebGlContextEvent> for JsValue {
        #[inline]
        fn from(obj: WebGlContextEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebGlContextEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WebGLContextEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WebGLContextEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WebGLContextEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebGlContextEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebGlContextEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebGlContextEvent> for Event {
    #[inline]
    fn from(obj: WebGlContextEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for WebGlContextEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WebGlContextEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebGlContextEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebGlContextEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WebGlContextEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_WebGLContextEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <WebGlContextEvent as WasmDescribe>::describe();
}
impl WebGlContextEvent {
    #[cfg(all(feature = "WebGlContextEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new WebGLContextEvent(..)` constructor, creating a new instance of `WebGLContextEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/WebGLContextEvent)\n\n*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<WebGlContextEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlContextEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_WebGLContextEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebGlContextEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_WebGLContextEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebGlContextEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_WebGLContextEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WebGlContextEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WebGlContextEvent", feature = "WebGlContextEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_WebGLContextEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&WebGlContextEventInit as WasmDescribe>::describe();
    <WebGlContextEvent as WasmDescribe>::describe();
}
impl WebGlContextEvent {
    #[cfg(all(feature = "WebGlContextEvent", feature = "WebGlContextEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new WebGLContextEvent(..)` constructor, creating a new instance of `WebGLContextEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/WebGLContextEvent)\n\n*This API requires the following crate features to be activated: `WebGlContextEvent`, `WebGlContextEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init(
        type_: &str,
        event_init: &WebGlContextEventInit,
    ) -> Result<WebGlContextEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlContextEvent", feature = "WebGlContextEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_WebGLContextEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init: <&WebGlContextEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WebGlContextEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_WebGLContextEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init: <&WebGlContextEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WebGlContextEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            drop(event_init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let event_init =
                    <&WebGlContextEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init,
                    );
                __widl_f_new_with_event_init_WebGLContextEvent(type_, event_init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WebGlContextEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WebGlContextEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_status_message_WebGLContextEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlContextEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WebGlContextEvent {
    #[cfg(all(feature = "WebGlContextEvent",))]
    #[allow(bad_style)]
    #[doc = "The `statusMessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/statusMessage)\n\n*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
    #[allow(clippy::all)]
    pub fn status_message(&self) -> String {
        #[cfg(all(feature = "WebGlContextEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_status_message_WebGLContextEvent(
                self_: <&WebGlContextEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_status_message_WebGLContextEvent(
            self_: <&WebGlContextEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WebGlContextEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_status_message_WebGLContextEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f5f91dfb9d4092e3: [u8; 434usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}p\x01\0\0\0\0\x04\0\0\x02\x11WebGLContextEvent#__widl_instanceof_WebGLContextEvent\0\0\0\0\x1E__widl_f_new_WebGLContextEvent\x01\0\0\x01\x11WebGLContextEvent\0\x01\x01\x05type_\x03new\0\0\0.__widl_f_new_with_event_init_WebGLContextEvent\x01\0\0\x01\x11WebGLContextEvent\0\x01\x02\x05type_\nevent_init\x03new\0\0\0)__widl_f_status_message_WebGLContextEvent\0\0\0\x01\x11WebGLContextEvent\x01\0\x01\rstatusMessage\x01\x01\x05self_\rstatusMessage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
