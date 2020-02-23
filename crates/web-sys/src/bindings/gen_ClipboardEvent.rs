use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ClipboardEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent)\n\n*This API requires the following crate features to be activated: `ClipboardEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ClipboardEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ClipboardEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ClipboardEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(67u32);
            inform(108u32);
            inform(105u32);
            inform(112u32);
            inform(98u32);
            inform(111u32);
            inform(97u32);
            inform(114u32);
            inform(100u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for ClipboardEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for ClipboardEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ClipboardEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ClipboardEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ClipboardEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ClipboardEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ClipboardEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ClipboardEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ClipboardEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ClipboardEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ClipboardEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ClipboardEvent {
        #[inline]
        fn from(obj: JsValue) -> ClipboardEvent {
            ClipboardEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ClipboardEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ClipboardEvent> for ClipboardEvent {
        #[inline]
        fn as_ref(&self) -> &ClipboardEvent {
            self
        }
    }
    impl From<ClipboardEvent> for JsValue {
        #[inline]
        fn from(obj: ClipboardEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ClipboardEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ClipboardEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ClipboardEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ClipboardEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ClipboardEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ClipboardEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ClipboardEvent> for Event {
    #[inline]
    fn from(obj: ClipboardEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for ClipboardEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ClipboardEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: ClipboardEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ClipboardEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ClipboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_ClipboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <ClipboardEvent as WasmDescribe>::describe();
}
impl ClipboardEvent {
    #[cfg(all(feature = "ClipboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new ClipboardEvent(..)` constructor, creating a new instance of `ClipboardEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent/ClipboardEvent)\n\n*This API requires the following crate features to be activated: `ClipboardEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<ClipboardEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ClipboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_ClipboardEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ClipboardEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_ClipboardEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ClipboardEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_ClipboardEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ClipboardEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ClipboardEvent", feature = "ClipboardEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_ClipboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&ClipboardEventInit as WasmDescribe>::describe();
    <ClipboardEvent as WasmDescribe>::describe();
}
impl ClipboardEvent {
    #[cfg(all(feature = "ClipboardEvent", feature = "ClipboardEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new ClipboardEvent(..)` constructor, creating a new instance of `ClipboardEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent/ClipboardEvent)\n\n*This API requires the following crate features to be activated: `ClipboardEvent`, `ClipboardEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ClipboardEventInit,
    ) -> Result<ClipboardEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ClipboardEvent", feature = "ClipboardEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_ClipboardEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&ClipboardEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ClipboardEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_ClipboardEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&ClipboardEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ClipboardEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&ClipboardEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_ClipboardEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ClipboardEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ClipboardEvent", feature = "DataTransfer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clipboard_data_ClipboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ClipboardEvent as WasmDescribe>::describe();
    <Option<DataTransfer> as WasmDescribe>::describe();
}
impl ClipboardEvent {
    #[cfg(all(feature = "ClipboardEvent", feature = "DataTransfer",))]
    #[allow(bad_style)]
    #[doc = "The `clipboardData` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent/clipboardData)\n\n*This API requires the following crate features to be activated: `ClipboardEvent`, `DataTransfer`*"]
    #[allow(clippy::all)]
    pub fn clipboard_data(&self) -> Option<DataTransfer> {
        #[cfg(all(feature = "ClipboardEvent", feature = "DataTransfer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clipboard_data_ClipboardEvent(
                self_: <&ClipboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DataTransfer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clipboard_data_ClipboardEvent(
            self_: <&ClipboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DataTransfer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ClipboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clipboard_data_ClipboardEvent(self_)
            };
            <Option<DataTransfer> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8c643319602baef3: [u8; 420usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}b\x01\0\0\0\0\x04\0\0\x02\x0EClipboardEvent __widl_instanceof_ClipboardEvent\0\0\0\0\x1B__widl_f_new_ClipboardEvent\x01\0\0\x01\x0EClipboardEvent\0\x01\x01\x05type_\x03new\0\0\00__widl_f_new_with_event_init_dict_ClipboardEvent\x01\0\0\x01\x0EClipboardEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0&__widl_f_clipboard_data_ClipboardEvent\0\0\0\x01\x0EClipboardEvent\x01\0\x01\rclipboardData\x01\x01\x05self_\rclipboardData\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
