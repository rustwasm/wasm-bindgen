use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FetchEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent)\n\n*This API requires the following crate features to be activated: `FetchEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FetchEvent {
    obj: ExtendableEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FetchEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FetchEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(70u32);
            inform(101u32);
            inform(116u32);
            inform(99u32);
            inform(104u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for FetchEvent {
        type Target = ExtendableEvent;
        #[inline]
        fn deref(&self) -> &ExtendableEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for FetchEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FetchEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FetchEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FetchEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FetchEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FetchEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FetchEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FetchEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FetchEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FetchEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FetchEvent {
        #[inline]
        fn from(obj: JsValue) -> FetchEvent {
            FetchEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FetchEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FetchEvent> for FetchEvent {
        #[inline]
        fn as_ref(&self) -> &FetchEvent {
            self
        }
    }
    impl From<FetchEvent> for JsValue {
        #[inline]
        fn from(obj: FetchEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FetchEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FetchEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FetchEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FetchEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FetchEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FetchEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FetchEvent> for ExtendableEvent {
    #[inline]
    fn from(obj: FetchEvent) -> ExtendableEvent {
        use wasm_bindgen::JsCast;
        ExtendableEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<ExtendableEvent> for FetchEvent {
    #[inline]
    fn as_ref(&self) -> &ExtendableEvent {
        use wasm_bindgen::JsCast;
        ExtendableEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<FetchEvent> for Event {
    #[inline]
    fn from(obj: FetchEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for FetchEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<FetchEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: FetchEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FetchEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FetchEvent", feature = "FetchEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_FetchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&FetchEventInit as WasmDescribe>::describe();
    <FetchEvent as WasmDescribe>::describe();
}
impl FetchEvent {
    #[cfg(all(feature = "FetchEvent", feature = "FetchEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new FetchEvent(..)` constructor, creating a new instance of `FetchEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/FetchEvent)\n\n*This API requires the following crate features to be activated: `FetchEvent`, `FetchEventInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        type_: &str,
        event_init_dict: &FetchEventInit,
    ) -> Result<FetchEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FetchEvent", feature = "FetchEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_FetchEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&FetchEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FetchEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_FetchEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&FetchEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FetchEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&FetchEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_FetchEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FetchEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "FetchEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_respond_with_FetchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FetchEvent as WasmDescribe>::describe();
    <&::js_sys::Promise as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FetchEvent {
    #[cfg(all(feature = "FetchEvent",))]
    #[allow(bad_style)]
    #[doc = "The `respondWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/respondWith)\n\n*This API requires the following crate features to be activated: `FetchEvent`*"]
    #[allow(clippy::all)]
    pub fn respond_with(&self, r: &::js_sys::Promise) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FetchEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_respond_with_FetchEvent(
                self_: <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                r: <&::js_sys::Promise as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_respond_with_FetchEvent(
            self_: <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            r: <&::js_sys::Promise as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(r);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let r = <&::js_sys::Promise as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r);
                __widl_f_respond_with_FetchEvent(self_, r)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "FetchEvent", feature = "Request",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_FetchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FetchEvent as WasmDescribe>::describe();
    <Request as WasmDescribe>::describe();
}
impl FetchEvent {
    #[cfg(all(feature = "FetchEvent", feature = "Request",))]
    #[allow(bad_style)]
    #[doc = "The `request` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/request)\n\n*This API requires the following crate features to be activated: `FetchEvent`, `Request`*"]
    #[allow(clippy::all)]
    pub fn request(&self) -> Request {
        #[cfg(all(feature = "FetchEvent", feature = "Request",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_FetchEvent(
                self_: <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_FetchEvent(
            self_: <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Request as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_request_FetchEvent(self_)
            };
            <Request as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FetchEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_client_id_FetchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FetchEvent as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl FetchEvent {
    #[cfg(all(feature = "FetchEvent",))]
    #[allow(bad_style)]
    #[doc = "The `clientId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/clientId)\n\n*This API requires the following crate features to be activated: `FetchEvent`*"]
    #[allow(clippy::all)]
    pub fn client_id(&self) -> Option<String> {
        #[cfg(all(feature = "FetchEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_client_id_FetchEvent(
                self_: <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_client_id_FetchEvent(
            self_: <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_client_id_FetchEvent(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FetchEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_reload_FetchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FetchEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl FetchEvent {
    #[cfg(all(feature = "FetchEvent",))]
    #[allow(bad_style)]
    #[doc = "The `isReload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/isReload)\n\n*This API requires the following crate features to be activated: `FetchEvent`*"]
    #[allow(clippy::all)]
    pub fn is_reload(&self) -> bool {
        #[cfg(all(feature = "FetchEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_reload_FetchEvent(
                self_: <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_reload_FetchEvent(
            self_: <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FetchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_reload_FetchEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_e477f6d232de21d7: [u8; 523usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC9\x01\0\0\0\0\x06\0\0\x02\nFetchEvent\x1C__widl_instanceof_FetchEvent\0\0\0\0\x17__widl_f_new_FetchEvent\x01\0\0\x01\nFetchEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0 __widl_f_respond_with_FetchEvent\x01\0\0\x01\nFetchEvent\x01\0\0\x01\x02\x05self_\x01r\x0BrespondWith\0\0\0\x1B__widl_f_request_FetchEvent\0\0\0\x01\nFetchEvent\x01\0\x01\x07request\x01\x01\x05self_\x07request\0\0\0\x1D__widl_f_client_id_FetchEvent\0\0\0\x01\nFetchEvent\x01\0\x01\x08clientId\x01\x01\x05self_\x08clientId\0\0\0\x1D__widl_f_is_reload_FetchEvent\0\0\0\x01\nFetchEvent\x01\0\x01\x08isReload\x01\x01\x05self_\x08isReload\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
