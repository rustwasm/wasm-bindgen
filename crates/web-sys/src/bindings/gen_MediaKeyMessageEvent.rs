use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaKeyMessageEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent)\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaKeyMessageEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaKeyMessageEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaKeyMessageEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
            inform(77u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MediaKeyMessageEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaKeyMessageEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaKeyMessageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaKeyMessageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaKeyMessageEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaKeyMessageEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaKeyMessageEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaKeyMessageEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaKeyMessageEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaKeyMessageEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaKeyMessageEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaKeyMessageEvent {
        #[inline]
        fn from(obj: JsValue) -> MediaKeyMessageEvent {
            MediaKeyMessageEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaKeyMessageEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaKeyMessageEvent> for MediaKeyMessageEvent {
        #[inline]
        fn as_ref(&self) -> &MediaKeyMessageEvent {
            self
        }
    }
    impl From<MediaKeyMessageEvent> for JsValue {
        #[inline]
        fn from(obj: MediaKeyMessageEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaKeyMessageEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaKeyMessageEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaKeyMessageEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaKeyMessageEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaKeyMessageEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaKeyMessageEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaKeyMessageEvent> for Event {
    #[inline]
    fn from(obj: MediaKeyMessageEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for MediaKeyMessageEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaKeyMessageEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaKeyMessageEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaKeyMessageEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaKeyMessageEvent", feature = "MediaKeyMessageEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MediaKeyMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&MediaKeyMessageEventInit as WasmDescribe>::describe();
    <MediaKeyMessageEvent as WasmDescribe>::describe();
}
impl MediaKeyMessageEvent {
    #[cfg(all(feature = "MediaKeyMessageEvent", feature = "MediaKeyMessageEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaKeyMessageEvent(..)` constructor, creating a new instance of `MediaKeyMessageEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/MediaKeyMessageEvent)\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEvent`, `MediaKeyMessageEventInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        type_: &str,
        event_init_dict: &MediaKeyMessageEventInit,
    ) -> Result<MediaKeyMessageEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaKeyMessageEvent", feature = "MediaKeyMessageEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MediaKeyMessageEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & MediaKeyMessageEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <MediaKeyMessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MediaKeyMessageEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&MediaKeyMessageEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaKeyMessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&MediaKeyMessageEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_MediaKeyMessageEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaKeyMessageEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaKeyMessageEvent", feature = "MediaKeyMessageType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_message_type_MediaKeyMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeyMessageEvent as WasmDescribe>::describe();
    <MediaKeyMessageType as WasmDescribe>::describe();
}
impl MediaKeyMessageEvent {
    #[cfg(all(feature = "MediaKeyMessageEvent", feature = "MediaKeyMessageType",))]
    #[allow(bad_style)]
    #[doc = "The `messageType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/messageType)\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEvent`, `MediaKeyMessageType`*"]
    #[allow(clippy::all)]
    pub fn message_type(&self) -> MediaKeyMessageType {
        #[cfg(all(feature = "MediaKeyMessageEvent", feature = "MediaKeyMessageType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_message_type_MediaKeyMessageEvent(
                self_: <&MediaKeyMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaKeyMessageType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_message_type_MediaKeyMessageEvent(
            self_: <&MediaKeyMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaKeyMessageType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeyMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_message_type_MediaKeyMessageEvent(self_)
            };
            <MediaKeyMessageType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeyMessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_message_MediaKeyMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeyMessageEvent as WasmDescribe>::describe();
    <::js_sys::ArrayBuffer as WasmDescribe>::describe();
}
impl MediaKeyMessageEvent {
    #[cfg(all(feature = "MediaKeyMessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `message` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/message)\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEvent`*"]
    #[allow(clippy::all)]
    pub fn message(&self) -> Result<::js_sys::ArrayBuffer, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaKeyMessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_message_MediaKeyMessageEvent(
                self_: <&MediaKeyMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_message_MediaKeyMessageEvent(
            self_: <&MediaKeyMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeyMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_message_MediaKeyMessageEvent(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::ArrayBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5453155b6232f45f: [u8; 459usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x89\x01\0\0\0\0\x04\0\0\x02\x14MediaKeyMessageEvent&__widl_instanceof_MediaKeyMessageEvent\0\0\0\0!__widl_f_new_MediaKeyMessageEvent\x01\0\0\x01\x14MediaKeyMessageEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0*__widl_f_message_type_MediaKeyMessageEvent\0\0\0\x01\x14MediaKeyMessageEvent\x01\0\x01\x0BmessageType\x01\x01\x05self_\x0BmessageType\0\0\0%__widl_f_message_MediaKeyMessageEvent\x01\0\0\x01\x14MediaKeyMessageEvent\x01\0\x01\x07message\x01\x01\x05self_\x07message\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
