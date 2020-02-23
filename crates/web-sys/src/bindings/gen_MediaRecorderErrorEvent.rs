use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaRecorderErrorEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent)\n\n*This API requires the following crate features to be activated: `MediaRecorderErrorEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaRecorderErrorEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaRecorderErrorEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaRecorderErrorEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(23u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(82u32);
            inform(101u32);
            inform(99u32);
            inform(111u32);
            inform(114u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
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
    impl core::ops::Deref for MediaRecorderErrorEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaRecorderErrorEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaRecorderErrorEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaRecorderErrorEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaRecorderErrorEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaRecorderErrorEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaRecorderErrorEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaRecorderErrorEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaRecorderErrorEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaRecorderErrorEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaRecorderErrorEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaRecorderErrorEvent {
        #[inline]
        fn from(obj: JsValue) -> MediaRecorderErrorEvent {
            MediaRecorderErrorEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaRecorderErrorEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaRecorderErrorEvent> for MediaRecorderErrorEvent {
        #[inline]
        fn as_ref(&self) -> &MediaRecorderErrorEvent {
            self
        }
    }
    impl From<MediaRecorderErrorEvent> for JsValue {
        #[inline]
        fn from(obj: MediaRecorderErrorEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaRecorderErrorEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaRecorderErrorEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaRecorderErrorEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaRecorderErrorEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaRecorderErrorEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaRecorderErrorEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaRecorderErrorEvent> for Event {
    #[inline]
    fn from(obj: MediaRecorderErrorEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for MediaRecorderErrorEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaRecorderErrorEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaRecorderErrorEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaRecorderErrorEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "MediaRecorderErrorEvent",
    feature = "MediaRecorderErrorEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MediaRecorderErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&MediaRecorderErrorEventInit as WasmDescribe>::describe();
    <MediaRecorderErrorEvent as WasmDescribe>::describe();
}
impl MediaRecorderErrorEvent {
    #[cfg(all(
        feature = "MediaRecorderErrorEvent",
        feature = "MediaRecorderErrorEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new MediaRecorderErrorEvent(..)` constructor, creating a new instance of `MediaRecorderErrorEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent/MediaRecorderErrorEvent)\n\n*This API requires the following crate features to be activated: `MediaRecorderErrorEvent`, `MediaRecorderErrorEventInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        type_: &str,
        event_init_dict: &MediaRecorderErrorEventInit,
    ) -> Result<MediaRecorderErrorEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "MediaRecorderErrorEvent",
            feature = "MediaRecorderErrorEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MediaRecorderErrorEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & MediaRecorderErrorEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <MediaRecorderErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MediaRecorderErrorEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & MediaRecorderErrorEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <MediaRecorderErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&MediaRecorderErrorEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_MediaRecorderErrorEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaRecorderErrorEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomException", feature = "MediaRecorderErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_error_MediaRecorderErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorderErrorEvent as WasmDescribe>::describe();
    <DomException as WasmDescribe>::describe();
}
impl MediaRecorderErrorEvent {
    #[cfg(all(feature = "DomException", feature = "MediaRecorderErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `error` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent/error)\n\n*This API requires the following crate features to be activated: `DomException`, `MediaRecorderErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn error(&self) -> DomException {
        #[cfg(all(feature = "DomException", feature = "MediaRecorderErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_MediaRecorderErrorEvent(
                self_: <&MediaRecorderErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomException as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_MediaRecorderErrorEvent(
            self_: <&MediaRecorderErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomException as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaRecorderErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_error_MediaRecorderErrorEvent(self_)
            };
            <DomException as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d59991015a4f12c0: [u8; 365usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}+\x01\0\0\0\0\x03\0\0\x02\x17MediaRecorderErrorEvent)__widl_instanceof_MediaRecorderErrorEvent\0\0\0\0$__widl_f_new_MediaRecorderErrorEvent\x01\0\0\x01\x17MediaRecorderErrorEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0&__widl_f_error_MediaRecorderErrorEvent\0\0\0\x01\x17MediaRecorderErrorEvent\x01\0\x01\x05error\x01\x01\x05self_\x05error\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
