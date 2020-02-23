use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MIDIMessageEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent)\n\n*This API requires the following crate features to be activated: `MidiMessageEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MidiMessageEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MidiMessageEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MidiMessageEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(77u32);
            inform(73u32);
            inform(68u32);
            inform(73u32);
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
    impl core::ops::Deref for MidiMessageEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for MidiMessageEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MidiMessageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MidiMessageEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MidiMessageEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MidiMessageEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MidiMessageEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MidiMessageEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MidiMessageEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MidiMessageEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MidiMessageEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MidiMessageEvent {
        #[inline]
        fn from(obj: JsValue) -> MidiMessageEvent {
            MidiMessageEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MidiMessageEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MidiMessageEvent> for MidiMessageEvent {
        #[inline]
        fn as_ref(&self) -> &MidiMessageEvent {
            self
        }
    }
    impl From<MidiMessageEvent> for JsValue {
        #[inline]
        fn from(obj: MidiMessageEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MidiMessageEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MIDIMessageEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MIDIMessageEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MIDIMessageEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MidiMessageEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MidiMessageEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MidiMessageEvent> for Event {
    #[inline]
    fn from(obj: MidiMessageEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for MidiMessageEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MidiMessageEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: MidiMessageEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MidiMessageEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MidiMessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MIDIMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <MidiMessageEvent as WasmDescribe>::describe();
}
impl MidiMessageEvent {
    #[cfg(all(feature = "MidiMessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new MIDIMessageEvent(..)` constructor, creating a new instance of `MIDIMessageEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/MIDIMessageEvent)\n\n*This API requires the following crate features to be activated: `MidiMessageEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<MidiMessageEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MidiMessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MIDIMessageEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MidiMessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MIDIMessageEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MidiMessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_MIDIMessageEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MidiMessageEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MidiMessageEvent", feature = "MidiMessageEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_MIDIMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&MidiMessageEventInit as WasmDescribe>::describe();
    <MidiMessageEvent as WasmDescribe>::describe();
}
impl MidiMessageEvent {
    #[cfg(all(feature = "MidiMessageEvent", feature = "MidiMessageEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new MIDIMessageEvent(..)` constructor, creating a new instance of `MIDIMessageEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/MIDIMessageEvent)\n\n*This API requires the following crate features to be activated: `MidiMessageEvent`, `MidiMessageEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MidiMessageEventInit,
    ) -> Result<MidiMessageEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MidiMessageEvent", feature = "MidiMessageEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_MIDIMessageEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&MidiMessageEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MidiMessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_MIDIMessageEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&MidiMessageEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MidiMessageEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&MidiMessageEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_MIDIMessageEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MidiMessageEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MidiMessageEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_MIDIMessageEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiMessageEvent as WasmDescribe>::describe();
    <Vec<u8> as WasmDescribe>::describe();
}
impl MidiMessageEvent {
    #[cfg(all(feature = "MidiMessageEvent",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/data)\n\n*This API requires the following crate features to be activated: `MidiMessageEvent`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> Result<Vec<u8>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MidiMessageEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_MIDIMessageEvent(
                self_: <&MidiMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<u8> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_MIDIMessageEvent(
            self_: <&MidiMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Vec<u8> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MidiMessageEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_MIDIMessageEvent(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Vec<u8> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e0180cb6cd760f72: [u8; 408usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}V\x01\0\0\0\0\x04\0\0\x02\x10MIDIMessageEvent\"__widl_instanceof_MIDIMessageEvent\0\0\0\0\x1D__widl_f_new_MIDIMessageEvent\x01\0\0\x01\x10MIDIMessageEvent\0\x01\x01\x05type_\x03new\0\0\02__widl_f_new_with_event_init_dict_MIDIMessageEvent\x01\0\0\x01\x10MIDIMessageEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x1E__widl_f_data_MIDIMessageEvent\x01\0\0\x01\x10MIDIMessageEvent\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
