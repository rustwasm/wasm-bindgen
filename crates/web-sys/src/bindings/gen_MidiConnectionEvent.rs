use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MIDIConnectionEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent)\n\n*This API requires the following crate features to be activated: `MidiConnectionEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MidiConnectionEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MidiConnectionEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MidiConnectionEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(77u32);
            inform(73u32);
            inform(68u32);
            inform(73u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(110u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MidiConnectionEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for MidiConnectionEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MidiConnectionEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MidiConnectionEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MidiConnectionEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MidiConnectionEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MidiConnectionEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MidiConnectionEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MidiConnectionEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MidiConnectionEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MidiConnectionEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MidiConnectionEvent {
        #[inline]
        fn from(obj: JsValue) -> MidiConnectionEvent {
            MidiConnectionEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MidiConnectionEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MidiConnectionEvent> for MidiConnectionEvent {
        #[inline]
        fn as_ref(&self) -> &MidiConnectionEvent {
            self
        }
    }
    impl From<MidiConnectionEvent> for JsValue {
        #[inline]
        fn from(obj: MidiConnectionEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MidiConnectionEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MIDIConnectionEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MIDIConnectionEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MIDIConnectionEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MidiConnectionEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MidiConnectionEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MidiConnectionEvent> for Event {
    #[inline]
    fn from(obj: MidiConnectionEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for MidiConnectionEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MidiConnectionEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: MidiConnectionEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MidiConnectionEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MidiConnectionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MIDIConnectionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <MidiConnectionEvent as WasmDescribe>::describe();
}
impl MidiConnectionEvent {
    #[cfg(all(feature = "MidiConnectionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new MIDIConnectionEvent(..)` constructor, creating a new instance of `MIDIConnectionEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/MIDIConnectionEvent)\n\n*This API requires the following crate features to be activated: `MidiConnectionEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<MidiConnectionEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MidiConnectionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MIDIConnectionEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MidiConnectionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MIDIConnectionEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MidiConnectionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_MIDIConnectionEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MidiConnectionEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MidiConnectionEvent", feature = "MidiConnectionEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_MIDIConnectionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&MidiConnectionEventInit as WasmDescribe>::describe();
    <MidiConnectionEvent as WasmDescribe>::describe();
}
impl MidiConnectionEvent {
    #[cfg(all(feature = "MidiConnectionEvent", feature = "MidiConnectionEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new MIDIConnectionEvent(..)` constructor, creating a new instance of `MIDIConnectionEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/MIDIConnectionEvent)\n\n*This API requires the following crate features to be activated: `MidiConnectionEvent`, `MidiConnectionEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MidiConnectionEventInit,
    ) -> Result<MidiConnectionEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MidiConnectionEvent", feature = "MidiConnectionEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_MIDIConnectionEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & MidiConnectionEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <MidiConnectionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_MIDIConnectionEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&MidiConnectionEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MidiConnectionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&MidiConnectionEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_MIDIConnectionEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MidiConnectionEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MidiConnectionEvent", feature = "MidiPort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_port_MIDIConnectionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiConnectionEvent as WasmDescribe>::describe();
    <Option<MidiPort> as WasmDescribe>::describe();
}
impl MidiConnectionEvent {
    #[cfg(all(feature = "MidiConnectionEvent", feature = "MidiPort",))]
    #[allow(bad_style)]
    #[doc = "The `port` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/port)\n\n*This API requires the following crate features to be activated: `MidiConnectionEvent`, `MidiPort`*"]
    #[allow(clippy::all)]
    pub fn port(&self) -> Option<MidiPort> {
        #[cfg(all(feature = "MidiConnectionEvent", feature = "MidiPort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_port_MIDIConnectionEvent(
                self_: <&MidiConnectionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MidiPort> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_port_MIDIConnectionEvent(
            self_: <&MidiConnectionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MidiPort> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MidiConnectionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_port_MIDIConnectionEvent(self_)
            };
            <Option<MidiPort> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d038d6731f1d24b6: [u8; 432usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}n\x01\0\0\0\0\x04\0\0\x02\x13MIDIConnectionEvent%__widl_instanceof_MIDIConnectionEvent\0\0\0\0 __widl_f_new_MIDIConnectionEvent\x01\0\0\x01\x13MIDIConnectionEvent\0\x01\x01\x05type_\x03new\0\0\05__widl_f_new_with_event_init_dict_MIDIConnectionEvent\x01\0\0\x01\x13MIDIConnectionEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0!__widl_f_port_MIDIConnectionEvent\0\0\0\x01\x13MIDIConnectionEvent\x01\0\x01\x04port\x01\x01\x05self_\x04port\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
