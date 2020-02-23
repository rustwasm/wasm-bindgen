use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ErrorEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent)\n\n*This API requires the following crate features to be activated: `ErrorEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ErrorEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ErrorEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ErrorEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(69u32);
            inform(114u32);
            inform(114u32);
            inform(111u32);
            inform(114u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for ErrorEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for ErrorEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ErrorEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ErrorEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ErrorEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ErrorEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ErrorEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ErrorEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ErrorEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ErrorEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ErrorEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ErrorEvent {
        #[inline]
        fn from(obj: JsValue) -> ErrorEvent {
            ErrorEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ErrorEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ErrorEvent> for ErrorEvent {
        #[inline]
        fn as_ref(&self) -> &ErrorEvent {
            self
        }
    }
    impl From<ErrorEvent> for JsValue {
        #[inline]
        fn from(obj: ErrorEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ErrorEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ErrorEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ErrorEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ErrorEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ErrorEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ErrorEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ErrorEvent> for Event {
    #[inline]
    fn from(obj: ErrorEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for ErrorEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ErrorEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: ErrorEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ErrorEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_ErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <ErrorEvent as WasmDescribe>::describe();
}
impl ErrorEvent {
    #[cfg(all(feature = "ErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new ErrorEvent(..)` constructor, creating a new instance of `ErrorEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/ErrorEvent)\n\n*This API requires the following crate features to be activated: `ErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<ErrorEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_ErrorEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_ErrorEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_ErrorEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ErrorEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ErrorEvent", feature = "ErrorEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_ErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&ErrorEventInit as WasmDescribe>::describe();
    <ErrorEvent as WasmDescribe>::describe();
}
impl ErrorEvent {
    #[cfg(all(feature = "ErrorEvent", feature = "ErrorEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new ErrorEvent(..)` constructor, creating a new instance of `ErrorEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/ErrorEvent)\n\n*This API requires the following crate features to be activated: `ErrorEvent`, `ErrorEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ErrorEventInit,
    ) -> Result<ErrorEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ErrorEvent", feature = "ErrorEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_ErrorEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&ErrorEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_ErrorEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&ErrorEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&ErrorEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_ErrorEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ErrorEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_message_ErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ErrorEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl ErrorEvent {
    #[cfg(all(feature = "ErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `message` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/message)\n\n*This API requires the following crate features to be activated: `ErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn message(&self) -> String {
        #[cfg(all(feature = "ErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_message_ErrorEvent(
                self_: <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_message_ErrorEvent(
            self_: <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_message_ErrorEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_filename_ErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ErrorEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl ErrorEvent {
    #[cfg(all(feature = "ErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `filename` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/filename)\n\n*This API requires the following crate features to be activated: `ErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn filename(&self) -> String {
        #[cfg(all(feature = "ErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_filename_ErrorEvent(
                self_: <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_filename_ErrorEvent(
            self_: <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_filename_ErrorEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lineno_ErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ErrorEvent as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl ErrorEvent {
    #[cfg(all(feature = "ErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `lineno` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/lineno)\n\n*This API requires the following crate features to be activated: `ErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn lineno(&self) -> u32 {
        #[cfg(all(feature = "ErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lineno_ErrorEvent(
                self_: <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lineno_ErrorEvent(
            self_: <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_lineno_ErrorEvent(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_colno_ErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ErrorEvent as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl ErrorEvent {
    #[cfg(all(feature = "ErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `colno` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/colno)\n\n*This API requires the following crate features to be activated: `ErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn colno(&self) -> u32 {
        #[cfg(all(feature = "ErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_colno_ErrorEvent(
                self_: <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_colno_ErrorEvent(
            self_: <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_colno_ErrorEvent(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_error_ErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ErrorEvent as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl ErrorEvent {
    #[cfg(all(feature = "ErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `error` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/error)\n\n*This API requires the following crate features to be activated: `ErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn error(&self) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "ErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_ErrorEvent(
                self_: <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_ErrorEvent(
            self_: <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_error_ErrorEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_a9036a26e9e0e89d: [u8; 649usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}G\x02\0\0\0\0\x08\0\0\x02\nErrorEvent\x1C__widl_instanceof_ErrorEvent\0\0\0\0\x17__widl_f_new_ErrorEvent\x01\0\0\x01\nErrorEvent\0\x01\x01\x05type_\x03new\0\0\0,__widl_f_new_with_event_init_dict_ErrorEvent\x01\0\0\x01\nErrorEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x1B__widl_f_message_ErrorEvent\0\0\0\x01\nErrorEvent\x01\0\x01\x07message\x01\x01\x05self_\x07message\0\0\0\x1C__widl_f_filename_ErrorEvent\0\0\0\x01\nErrorEvent\x01\0\x01\x08filename\x01\x01\x05self_\x08filename\0\0\0\x1A__widl_f_lineno_ErrorEvent\0\0\0\x01\nErrorEvent\x01\0\x01\x06lineno\x01\x01\x05self_\x06lineno\0\0\0\x19__widl_f_colno_ErrorEvent\0\0\0\x01\nErrorEvent\x01\0\x01\x05colno\x01\x01\x05self_\x05colno\0\0\0\x19__widl_f_error_ErrorEvent\0\0\0\x01\nErrorEvent\x01\0\x01\x05error\x01\x01\x05self_\x05error\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
