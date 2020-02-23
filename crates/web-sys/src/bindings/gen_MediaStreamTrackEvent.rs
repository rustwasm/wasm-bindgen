use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaStreamTrackEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent)\n\n*This API requires the following crate features to be activated: `MediaStreamTrackEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaStreamTrackEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaStreamTrackEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaStreamTrackEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(83u32);
            inform(116u32);
            inform(114u32);
            inform(101u32);
            inform(97u32);
            inform(109u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MediaStreamTrackEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaStreamTrackEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaStreamTrackEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaStreamTrackEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaStreamTrackEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaStreamTrackEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaStreamTrackEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaStreamTrackEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaStreamTrackEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaStreamTrackEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaStreamTrackEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaStreamTrackEvent {
        #[inline]
        fn from(obj: JsValue) -> MediaStreamTrackEvent {
            MediaStreamTrackEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaStreamTrackEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaStreamTrackEvent> for MediaStreamTrackEvent {
        #[inline]
        fn as_ref(&self) -> &MediaStreamTrackEvent {
            self
        }
    }
    impl From<MediaStreamTrackEvent> for JsValue {
        #[inline]
        fn from(obj: MediaStreamTrackEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaStreamTrackEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaStreamTrackEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaStreamTrackEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaStreamTrackEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaStreamTrackEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaStreamTrackEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaStreamTrackEvent> for Event {
    #[inline]
    fn from(obj: MediaStreamTrackEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for MediaStreamTrackEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaStreamTrackEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaStreamTrackEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaStreamTrackEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "MediaStreamTrackEvent",
    feature = "MediaStreamTrackEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MediaStreamTrackEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&MediaStreamTrackEventInit as WasmDescribe>::describe();
    <MediaStreamTrackEvent as WasmDescribe>::describe();
}
impl MediaStreamTrackEvent {
    #[cfg(all(
        feature = "MediaStreamTrackEvent",
        feature = "MediaStreamTrackEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new MediaStreamTrackEvent(..)` constructor, creating a new instance of `MediaStreamTrackEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent/MediaStreamTrackEvent)\n\n*This API requires the following crate features to be activated: `MediaStreamTrackEvent`, `MediaStreamTrackEventInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        type_: &str,
        event_init_dict: &MediaStreamTrackEventInit,
    ) -> Result<MediaStreamTrackEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "MediaStreamTrackEvent",
            feature = "MediaStreamTrackEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MediaStreamTrackEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & MediaStreamTrackEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <MediaStreamTrackEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MediaStreamTrackEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & MediaStreamTrackEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <MediaStreamTrackEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&MediaStreamTrackEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_MediaStreamTrackEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaStreamTrackEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack", feature = "MediaStreamTrackEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_track_MediaStreamTrackEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrackEvent as WasmDescribe>::describe();
    <MediaStreamTrack as WasmDescribe>::describe();
}
impl MediaStreamTrackEvent {
    #[cfg(all(feature = "MediaStreamTrack", feature = "MediaStreamTrackEvent",))]
    #[allow(bad_style)]
    #[doc = "The `track` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent/track)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEvent`*"]
    #[allow(clippy::all)]
    pub fn track(&self) -> MediaStreamTrack {
        #[cfg(all(feature = "MediaStreamTrack", feature = "MediaStreamTrackEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_track_MediaStreamTrackEvent(
                self_: <&MediaStreamTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_track_MediaStreamTrackEvent(
            self_: <&MediaStreamTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrackEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_track_MediaStreamTrackEvent(self_)
            };
            <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_3c0ae6e6c00f8f29: [u8; 353usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1F\x01\0\0\0\0\x03\0\0\x02\x15MediaStreamTrackEvent'__widl_instanceof_MediaStreamTrackEvent\0\0\0\0\"__widl_f_new_MediaStreamTrackEvent\x01\0\0\x01\x15MediaStreamTrackEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0$__widl_f_track_MediaStreamTrackEvent\0\0\0\x01\x15MediaStreamTrackEvent\x01\0\x01\x05track\x01\x01\x05self_\x05track\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
