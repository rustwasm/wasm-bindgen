use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MIDIPort` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MidiPort {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MidiPort: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MidiPort {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(77u32);
            inform(73u32);
            inform(68u32);
            inform(73u32);
            inform(80u32);
            inform(111u32);
            inform(114u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MidiPort {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for MidiPort {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MidiPort {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MidiPort {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MidiPort {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MidiPort {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MidiPort {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MidiPort {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MidiPort {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MidiPort>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MidiPort {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MidiPort {
        #[inline]
        fn from(obj: JsValue) -> MidiPort {
            MidiPort { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MidiPort {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MidiPort> for MidiPort {
        #[inline]
        fn as_ref(&self) -> &MidiPort {
            self
        }
    }
    impl From<MidiPort> for JsValue {
        #[inline]
        fn from(obj: MidiPort) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MidiPort {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MIDIPort(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MIDIPort(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MIDIPort(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MidiPort { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MidiPort) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MidiPort> for EventTarget {
    #[inline]
    fn from(obj: MidiPort) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MidiPort {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MidiPort> for ::js_sys::Object {
    #[inline]
    fn from(obj: MidiPort) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MidiPort {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MidiPort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_MIDIPort() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiPort as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MidiPort {
    #[cfg(all(feature = "MidiPort",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/close)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    #[allow(clippy::all)]
    pub fn close(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "MidiPort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_MIDIPort(
                self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_MIDIPort(
            self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_MIDIPort(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiPort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_MIDIPort() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiPort as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MidiPort {
    #[cfg(all(feature = "MidiPort",))]
    #[allow(bad_style)]
    #[doc = "The `open()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/open)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    #[allow(clippy::all)]
    pub fn open(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "MidiPort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_MIDIPort(
                self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_MIDIPort(
            self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_open_MIDIPort(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiPort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_MIDIPort() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiPort as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MidiPort {
    #[cfg(all(feature = "MidiPort",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/id)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "MidiPort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_MIDIPort(
                self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_MIDIPort(
            self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_MIDIPort(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiPort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_manufacturer_MIDIPort() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiPort as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl MidiPort {
    #[cfg(all(feature = "MidiPort",))]
    #[allow(bad_style)]
    #[doc = "The `manufacturer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/manufacturer)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    #[allow(clippy::all)]
    pub fn manufacturer(&self) -> Option<String> {
        #[cfg(all(feature = "MidiPort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_manufacturer_MIDIPort(
                self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_manufacturer_MIDIPort(
            self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_manufacturer_MIDIPort(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiPort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_MIDIPort() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiPort as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl MidiPort {
    #[cfg(all(feature = "MidiPort",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/name)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> Option<String> {
        #[cfg(all(feature = "MidiPort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_MIDIPort(
                self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_MIDIPort(
            self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_MIDIPort(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiPort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_version_MIDIPort() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiPort as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl MidiPort {
    #[cfg(all(feature = "MidiPort",))]
    #[allow(bad_style)]
    #[doc = "The `version` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/version)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    #[allow(clippy::all)]
    pub fn version(&self) -> Option<String> {
        #[cfg(all(feature = "MidiPort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_version_MIDIPort(
                self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_version_MIDIPort(
            self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_version_MIDIPort(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiPort", feature = "MidiPortType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_MIDIPort() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiPort as WasmDescribe>::describe();
    <MidiPortType as WasmDescribe>::describe();
}
impl MidiPort {
    #[cfg(all(feature = "MidiPort", feature = "MidiPortType",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/type)\n\n*This API requires the following crate features to be activated: `MidiPort`, `MidiPortType`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> MidiPortType {
        #[cfg(all(feature = "MidiPort", feature = "MidiPortType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_MIDIPort(
                self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MidiPortType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_MIDIPort(
            self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MidiPortType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_MIDIPort(self_)
            };
            <MidiPortType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiPort", feature = "MidiPortDeviceState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_state_MIDIPort() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiPort as WasmDescribe>::describe();
    <MidiPortDeviceState as WasmDescribe>::describe();
}
impl MidiPort {
    #[cfg(all(feature = "MidiPort", feature = "MidiPortDeviceState",))]
    #[allow(bad_style)]
    #[doc = "The `state` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/state)\n\n*This API requires the following crate features to be activated: `MidiPort`, `MidiPortDeviceState`*"]
    #[allow(clippy::all)]
    pub fn state(&self) -> MidiPortDeviceState {
        #[cfg(all(feature = "MidiPort", feature = "MidiPortDeviceState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_state_MIDIPort(
                self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MidiPortDeviceState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_state_MIDIPort(
            self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MidiPortDeviceState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_state_MIDIPort(self_)
            };
            <MidiPortDeviceState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiPort", feature = "MidiPortConnectionState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connection_MIDIPort() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiPort as WasmDescribe>::describe();
    <MidiPortConnectionState as WasmDescribe>::describe();
}
impl MidiPort {
    #[cfg(all(feature = "MidiPort", feature = "MidiPortConnectionState",))]
    #[allow(bad_style)]
    #[doc = "The `connection` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/connection)\n\n*This API requires the following crate features to be activated: `MidiPort`, `MidiPortConnectionState`*"]
    #[allow(clippy::all)]
    pub fn connection(&self) -> MidiPortConnectionState {
        #[cfg(all(feature = "MidiPort", feature = "MidiPortConnectionState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connection_MIDIPort(
                self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MidiPortConnectionState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connection_MIDIPort(
            self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MidiPortConnectionState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_connection_MIDIPort(self_)
            };
            <MidiPortConnectionState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiPort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstatechange_MIDIPort() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiPort as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MidiPort {
    #[cfg(all(feature = "MidiPort",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    #[allow(clippy::all)]
    pub fn onstatechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MidiPort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstatechange_MIDIPort(
                self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstatechange_MIDIPort(
            self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstatechange_MIDIPort(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiPort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstatechange_MIDIPort() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MidiPort as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MidiPort {
    #[cfg(all(feature = "MidiPort",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    #[allow(clippy::all)]
    pub fn set_onstatechange(&self, onstatechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MidiPort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstatechange_MIDIPort(
                self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstatechange_MIDIPort(
            self_: <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstatechange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstatechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiPort as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstatechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstatechange,
                    );
                __widl_f_set_onstatechange_MIDIPort(self_, onstatechange)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_82f49ca3a9ec6d55: [u8; 917usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}S\x03\0\0\0\0\x0C\0\0\x02\x08MIDIPort\x1A__widl_instanceof_MIDIPort\0\0\0\0\x17__widl_f_close_MIDIPort\0\0\0\x01\x08MIDIPort\x01\0\0\x01\x01\x05self_\x05close\0\0\0\x16__widl_f_open_MIDIPort\0\0\0\x01\x08MIDIPort\x01\0\0\x01\x01\x05self_\x04open\0\0\0\x14__widl_f_id_MIDIPort\0\0\0\x01\x08MIDIPort\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0\x1E__widl_f_manufacturer_MIDIPort\0\0\0\x01\x08MIDIPort\x01\0\x01\x0Cmanufacturer\x01\x01\x05self_\x0Cmanufacturer\0\0\0\x16__widl_f_name_MIDIPort\0\0\0\x01\x08MIDIPort\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\x19__widl_f_version_MIDIPort\0\0\0\x01\x08MIDIPort\x01\0\x01\x07version\x01\x01\x05self_\x07version\0\0\0\x16__widl_f_type_MIDIPort\0\0\0\x01\x08MIDIPort\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\x17__widl_f_state_MIDIPort\0\0\0\x01\x08MIDIPort\x01\0\x01\x05state\x01\x01\x05self_\x05state\0\0\0\x1C__widl_f_connection_MIDIPort\0\0\0\x01\x08MIDIPort\x01\0\x01\nconnection\x01\x01\x05self_\nconnection\0\0\0\x1F__widl_f_onstatechange_MIDIPort\0\0\0\x01\x08MIDIPort\x01\0\x01\ronstatechange\x01\x01\x05self_\ronstatechange\0\0\0#__widl_f_set_onstatechange_MIDIPort\0\0\0\x01\x08MIDIPort\x01\0\x02\ronstatechange\x01\x02\x05self_\ronstatechange\ronstatechange\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
