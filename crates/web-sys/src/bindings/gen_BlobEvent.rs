use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `BlobEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent)\n\n*This API requires the following crate features to be activated: `BlobEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct BlobEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_BlobEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for BlobEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(66u32);
            inform(108u32);
            inform(111u32);
            inform(98u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for BlobEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for BlobEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for BlobEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a BlobEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for BlobEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            BlobEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for BlobEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a BlobEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for BlobEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<BlobEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(BlobEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for BlobEvent {
        #[inline]
        fn from(obj: JsValue) -> BlobEvent {
            BlobEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for BlobEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<BlobEvent> for BlobEvent {
        #[inline]
        fn as_ref(&self) -> &BlobEvent {
            self
        }
    }
    impl From<BlobEvent> for JsValue {
        #[inline]
        fn from(obj: BlobEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for BlobEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_BlobEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_BlobEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_BlobEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            BlobEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const BlobEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<BlobEvent> for Event {
    #[inline]
    fn from(obj: BlobEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for BlobEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<BlobEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: BlobEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for BlobEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BlobEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_BlobEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <BlobEvent as WasmDescribe>::describe();
}
impl BlobEvent {
    #[cfg(all(feature = "BlobEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new BlobEvent(..)` constructor, creating a new instance of `BlobEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/BlobEvent)\n\n*This API requires the following crate features to be activated: `BlobEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<BlobEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BlobEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_BlobEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <BlobEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_BlobEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <BlobEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_BlobEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<BlobEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "BlobEvent", feature = "BlobEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_BlobEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&BlobEventInit as WasmDescribe>::describe();
    <BlobEvent as WasmDescribe>::describe();
}
impl BlobEvent {
    #[cfg(all(feature = "BlobEvent", feature = "BlobEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new BlobEvent(..)` constructor, creating a new instance of `BlobEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/BlobEvent)\n\n*This API requires the following crate features to be activated: `BlobEvent`, `BlobEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &BlobEventInit,
    ) -> Result<BlobEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BlobEvent", feature = "BlobEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_BlobEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&BlobEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <BlobEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_BlobEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&BlobEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <BlobEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&BlobEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_BlobEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<BlobEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "BlobEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_BlobEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BlobEvent as WasmDescribe>::describe();
    <Option<Blob> as WasmDescribe>::describe();
}
impl BlobEvent {
    #[cfg(all(feature = "Blob", feature = "BlobEvent",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/data)\n\n*This API requires the following crate features to be activated: `Blob`, `BlobEvent`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> Option<Blob> {
        #[cfg(all(feature = "Blob", feature = "BlobEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_BlobEvent(
                self_: <&BlobEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Blob> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_BlobEvent(
            self_: <&BlobEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Blob> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BlobEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_BlobEvent(self_)
            };
            <Option<Blob> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_6bb64fdc2d2ca090: [u8; 352usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1E\x01\0\0\0\0\x04\0\0\x02\tBlobEvent\x1B__widl_instanceof_BlobEvent\0\0\0\0\x16__widl_f_new_BlobEvent\x01\0\0\x01\tBlobEvent\0\x01\x01\x05type_\x03new\0\0\0+__widl_f_new_with_event_init_dict_BlobEvent\x01\0\0\x01\tBlobEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x17__widl_f_data_BlobEvent\0\0\0\x01\tBlobEvent\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
