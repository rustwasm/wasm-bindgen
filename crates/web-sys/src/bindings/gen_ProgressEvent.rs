use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ProgressEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent)\n\n*This API requires the following crate features to be activated: `ProgressEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ProgressEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ProgressEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ProgressEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(80u32);
            inform(114u32);
            inform(111u32);
            inform(103u32);
            inform(114u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for ProgressEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for ProgressEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ProgressEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ProgressEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ProgressEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ProgressEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ProgressEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ProgressEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ProgressEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ProgressEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ProgressEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ProgressEvent {
        #[inline]
        fn from(obj: JsValue) -> ProgressEvent {
            ProgressEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ProgressEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ProgressEvent> for ProgressEvent {
        #[inline]
        fn as_ref(&self) -> &ProgressEvent {
            self
        }
    }
    impl From<ProgressEvent> for JsValue {
        #[inline]
        fn from(obj: ProgressEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ProgressEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ProgressEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ProgressEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ProgressEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ProgressEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ProgressEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ProgressEvent> for Event {
    #[inline]
    fn from(obj: ProgressEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for ProgressEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ProgressEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: ProgressEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ProgressEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ProgressEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_ProgressEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <ProgressEvent as WasmDescribe>::describe();
}
impl ProgressEvent {
    #[cfg(all(feature = "ProgressEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new ProgressEvent(..)` constructor, creating a new instance of `ProgressEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/ProgressEvent)\n\n*This API requires the following crate features to be activated: `ProgressEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<ProgressEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ProgressEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_ProgressEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ProgressEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_ProgressEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ProgressEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_ProgressEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ProgressEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ProgressEvent", feature = "ProgressEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_ProgressEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&ProgressEventInit as WasmDescribe>::describe();
    <ProgressEvent as WasmDescribe>::describe();
}
impl ProgressEvent {
    #[cfg(all(feature = "ProgressEvent", feature = "ProgressEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new ProgressEvent(..)` constructor, creating a new instance of `ProgressEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/ProgressEvent)\n\n*This API requires the following crate features to be activated: `ProgressEvent`, `ProgressEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ProgressEventInit,
    ) -> Result<ProgressEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ProgressEvent", feature = "ProgressEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_ProgressEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&ProgressEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ProgressEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_ProgressEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&ProgressEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ProgressEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&ProgressEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_ProgressEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ProgressEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ProgressEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_computable_ProgressEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ProgressEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ProgressEvent {
    #[cfg(all(feature = "ProgressEvent",))]
    #[allow(bad_style)]
    #[doc = "The `lengthComputable` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/lengthComputable)\n\n*This API requires the following crate features to be activated: `ProgressEvent`*"]
    #[allow(clippy::all)]
    pub fn length_computable(&self) -> bool {
        #[cfg(all(feature = "ProgressEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_computable_ProgressEvent(
                self_: <&ProgressEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_computable_ProgressEvent(
            self_: <&ProgressEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ProgressEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_computable_ProgressEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ProgressEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_loaded_ProgressEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ProgressEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl ProgressEvent {
    #[cfg(all(feature = "ProgressEvent",))]
    #[allow(bad_style)]
    #[doc = "The `loaded` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/loaded)\n\n*This API requires the following crate features to be activated: `ProgressEvent`*"]
    #[allow(clippy::all)]
    pub fn loaded(&self) -> f64 {
        #[cfg(all(feature = "ProgressEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_loaded_ProgressEvent(
                self_: <&ProgressEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_loaded_ProgressEvent(
            self_: <&ProgressEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ProgressEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_loaded_ProgressEvent(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ProgressEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_total_ProgressEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ProgressEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl ProgressEvent {
    #[cfg(all(feature = "ProgressEvent",))]
    #[allow(bad_style)]
    #[doc = "The `total` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/total)\n\n*This API requires the following crate features to be activated: `ProgressEvent`*"]
    #[allow(clippy::all)]
    pub fn total(&self) -> f64 {
        #[cfg(all(feature = "ProgressEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_total_ProgressEvent(
                self_: <&ProgressEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_total_ProgressEvent(
            self_: <&ProgressEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ProgressEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_total_ProgressEvent(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_77d7c2a65641cd58: [u8; 570usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xF8\x01\0\0\0\0\x06\0\0\x02\rProgressEvent\x1F__widl_instanceof_ProgressEvent\0\0\0\0\x1A__widl_f_new_ProgressEvent\x01\0\0\x01\rProgressEvent\0\x01\x01\x05type_\x03new\0\0\0/__widl_f_new_with_event_init_dict_ProgressEvent\x01\0\0\x01\rProgressEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0(__widl_f_length_computable_ProgressEvent\0\0\0\x01\rProgressEvent\x01\0\x01\x10lengthComputable\x01\x01\x05self_\x10lengthComputable\0\0\0\x1D__widl_f_loaded_ProgressEvent\0\0\0\x01\rProgressEvent\x01\0\x01\x06loaded\x01\x01\x05self_\x06loaded\0\0\0\x1C__widl_f_total_ProgressEvent\0\0\0\x01\rProgressEvent\x01\0\x01\x05total\x01\x01\x05self_\x05total\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
