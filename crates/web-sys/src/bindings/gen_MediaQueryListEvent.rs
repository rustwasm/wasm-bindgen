use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaQueryListEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent)\n\n*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaQueryListEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaQueryListEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaQueryListEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(81u32);
            inform(117u32);
            inform(101u32);
            inform(114u32);
            inform(121u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MediaQueryListEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaQueryListEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaQueryListEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaQueryListEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaQueryListEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaQueryListEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaQueryListEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaQueryListEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaQueryListEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaQueryListEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaQueryListEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaQueryListEvent {
        #[inline]
        fn from(obj: JsValue) -> MediaQueryListEvent {
            MediaQueryListEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaQueryListEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaQueryListEvent> for MediaQueryListEvent {
        #[inline]
        fn as_ref(&self) -> &MediaQueryListEvent {
            self
        }
    }
    impl From<MediaQueryListEvent> for JsValue {
        #[inline]
        fn from(obj: MediaQueryListEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaQueryListEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaQueryListEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaQueryListEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaQueryListEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaQueryListEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaQueryListEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaQueryListEvent> for Event {
    #[inline]
    fn from(obj: MediaQueryListEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for MediaQueryListEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaQueryListEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaQueryListEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaQueryListEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaQueryListEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MediaQueryListEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <MediaQueryListEvent as WasmDescribe>::describe();
}
impl MediaQueryListEvent {
    #[cfg(all(feature = "MediaQueryListEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaQueryListEvent(..)` constructor, creating a new instance of `MediaQueryListEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/MediaQueryListEvent)\n\n*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<MediaQueryListEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaQueryListEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MediaQueryListEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaQueryListEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MediaQueryListEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaQueryListEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_MediaQueryListEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaQueryListEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaQueryListEvent", feature = "MediaQueryListEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_MediaQueryListEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&MediaQueryListEventInit as WasmDescribe>::describe();
    <MediaQueryListEvent as WasmDescribe>::describe();
}
impl MediaQueryListEvent {
    #[cfg(all(feature = "MediaQueryListEvent", feature = "MediaQueryListEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaQueryListEvent(..)` constructor, creating a new instance of `MediaQueryListEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/MediaQueryListEvent)\n\n*This API requires the following crate features to be activated: `MediaQueryListEvent`, `MediaQueryListEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MediaQueryListEventInit,
    ) -> Result<MediaQueryListEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaQueryListEvent", feature = "MediaQueryListEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_MediaQueryListEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & MediaQueryListEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <MediaQueryListEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_MediaQueryListEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&MediaQueryListEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaQueryListEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&MediaQueryListEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_MediaQueryListEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaQueryListEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaQueryListEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_MediaQueryListEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaQueryListEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaQueryListEvent {
    #[cfg(all(feature = "MediaQueryListEvent",))]
    #[allow(bad_style)]
    #[doc = "The `media` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/media)\n\n*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
    #[allow(clippy::all)]
    pub fn media(&self) -> String {
        #[cfg(all(feature = "MediaQueryListEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_MediaQueryListEvent(
                self_: <&MediaQueryListEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_MediaQueryListEvent(
            self_: <&MediaQueryListEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaQueryListEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_MediaQueryListEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaQueryListEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_matches_MediaQueryListEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaQueryListEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaQueryListEvent {
    #[cfg(all(feature = "MediaQueryListEvent",))]
    #[allow(bad_style)]
    #[doc = "The `matches` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/matches)\n\n*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
    #[allow(clippy::all)]
    pub fn matches(&self) -> bool {
        #[cfg(all(feature = "MediaQueryListEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_matches_MediaQueryListEvent(
                self_: <&MediaQueryListEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_matches_MediaQueryListEvent(
            self_: <&MediaQueryListEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaQueryListEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_matches_MediaQueryListEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ead21534615ac398: [u8; 526usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCC\x01\0\0\0\0\x05\0\0\x02\x13MediaQueryListEvent%__widl_instanceof_MediaQueryListEvent\0\0\0\0 __widl_f_new_MediaQueryListEvent\x01\0\0\x01\x13MediaQueryListEvent\0\x01\x01\x05type_\x03new\0\0\05__widl_f_new_with_event_init_dict_MediaQueryListEvent\x01\0\0\x01\x13MediaQueryListEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\"__widl_f_media_MediaQueryListEvent\0\0\0\x01\x13MediaQueryListEvent\x01\0\x01\x05media\x01\x01\x05self_\x05media\0\0\0$__widl_f_matches_MediaQueryListEvent\0\0\0\x01\x13MediaQueryListEvent\x01\0\x01\x07matches\x01\x01\x05self_\x07matches\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
