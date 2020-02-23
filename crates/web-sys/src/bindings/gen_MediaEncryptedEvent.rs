use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaEncryptedEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent)\n\n*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaEncryptedEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaEncryptedEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaEncryptedEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(69u32);
            inform(110u32);
            inform(99u32);
            inform(114u32);
            inform(121u32);
            inform(112u32);
            inform(116u32);
            inform(101u32);
            inform(100u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MediaEncryptedEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaEncryptedEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaEncryptedEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaEncryptedEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaEncryptedEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaEncryptedEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaEncryptedEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaEncryptedEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaEncryptedEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaEncryptedEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaEncryptedEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaEncryptedEvent {
        #[inline]
        fn from(obj: JsValue) -> MediaEncryptedEvent {
            MediaEncryptedEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaEncryptedEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaEncryptedEvent> for MediaEncryptedEvent {
        #[inline]
        fn as_ref(&self) -> &MediaEncryptedEvent {
            self
        }
    }
    impl From<MediaEncryptedEvent> for JsValue {
        #[inline]
        fn from(obj: MediaEncryptedEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaEncryptedEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaEncryptedEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaEncryptedEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaEncryptedEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaEncryptedEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaEncryptedEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaEncryptedEvent> for Event {
    #[inline]
    fn from(obj: MediaEncryptedEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for MediaEncryptedEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaEncryptedEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaEncryptedEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaEncryptedEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaEncryptedEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MediaEncryptedEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <MediaEncryptedEvent as WasmDescribe>::describe();
}
impl MediaEncryptedEvent {
    #[cfg(all(feature = "MediaEncryptedEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaEncryptedEvent(..)` constructor, creating a new instance of `MediaEncryptedEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/MediaEncryptedEvent)\n\n*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<MediaEncryptedEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaEncryptedEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MediaEncryptedEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaEncryptedEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MediaEncryptedEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaEncryptedEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_MediaEncryptedEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaEncryptedEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaEncryptedEvent", feature = "MediaKeyNeededEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_MediaEncryptedEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&MediaKeyNeededEventInit as WasmDescribe>::describe();
    <MediaEncryptedEvent as WasmDescribe>::describe();
}
impl MediaEncryptedEvent {
    #[cfg(all(feature = "MediaEncryptedEvent", feature = "MediaKeyNeededEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaEncryptedEvent(..)` constructor, creating a new instance of `MediaEncryptedEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/MediaEncryptedEvent)\n\n*This API requires the following crate features to be activated: `MediaEncryptedEvent`, `MediaKeyNeededEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MediaKeyNeededEventInit,
    ) -> Result<MediaEncryptedEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaEncryptedEvent", feature = "MediaKeyNeededEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_MediaEncryptedEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & MediaKeyNeededEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <MediaEncryptedEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_MediaEncryptedEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&MediaKeyNeededEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaEncryptedEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&MediaKeyNeededEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_MediaEncryptedEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaEncryptedEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaEncryptedEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_data_type_MediaEncryptedEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaEncryptedEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaEncryptedEvent {
    #[cfg(all(feature = "MediaEncryptedEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initDataType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initDataType)\n\n*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
    #[allow(clippy::all)]
    pub fn init_data_type(&self) -> String {
        #[cfg(all(feature = "MediaEncryptedEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_data_type_MediaEncryptedEvent(
                self_: <&MediaEncryptedEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_data_type_MediaEncryptedEvent(
            self_: <&MediaEncryptedEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaEncryptedEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_init_data_type_MediaEncryptedEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaEncryptedEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_data_MediaEncryptedEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaEncryptedEvent as WasmDescribe>::describe();
    <Option<::js_sys::ArrayBuffer> as WasmDescribe>::describe();
}
impl MediaEncryptedEvent {
    #[cfg(all(feature = "MediaEncryptedEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initData` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initData)\n\n*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
    #[allow(clippy::all)]
    pub fn init_data(&self) -> Result<Option<::js_sys::ArrayBuffer>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaEncryptedEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_data_MediaEncryptedEvent(
                self_: <&MediaEncryptedEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::ArrayBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_data_MediaEncryptedEvent(
            self_: <&MediaEncryptedEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::ArrayBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaEncryptedEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_init_data_MediaEncryptedEvent(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(
                <Option<::js_sys::ArrayBuffer> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    _ret,
                ),
            )
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d3f3583355bf511f: [u8; 553usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE7\x01\0\0\0\0\x05\0\0\x02\x13MediaEncryptedEvent%__widl_instanceof_MediaEncryptedEvent\0\0\0\0 __widl_f_new_MediaEncryptedEvent\x01\0\0\x01\x13MediaEncryptedEvent\0\x01\x01\x05type_\x03new\0\0\05__widl_f_new_with_event_init_dict_MediaEncryptedEvent\x01\0\0\x01\x13MediaEncryptedEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0+__widl_f_init_data_type_MediaEncryptedEvent\0\0\0\x01\x13MediaEncryptedEvent\x01\0\x01\x0CinitDataType\x01\x01\x05self_\x0CinitDataType\0\0\0&__widl_f_init_data_MediaEncryptedEvent\x01\0\0\x01\x13MediaEncryptedEvent\x01\0\x01\x08initData\x01\x01\x05self_\x08initData\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
