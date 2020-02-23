use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MIDIOutput` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput)\n\n*This API requires the following crate features to be activated: `MidiOutput`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MidiOutput {
    obj: MidiPort,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MidiOutput: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MidiOutput {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(77u32);
            inform(73u32);
            inform(68u32);
            inform(73u32);
            inform(79u32);
            inform(117u32);
            inform(116u32);
            inform(112u32);
            inform(117u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MidiOutput {
        type Target = MidiPort;
        #[inline]
        fn deref(&self) -> &MidiPort {
            &self.obj
        }
    }
    impl IntoWasmAbi for MidiOutput {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MidiOutput {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MidiOutput {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MidiOutput {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MidiOutput {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MidiOutput {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MidiOutput {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MidiOutput {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MidiOutput>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MidiOutput {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MidiOutput {
        #[inline]
        fn from(obj: JsValue) -> MidiOutput {
            MidiOutput { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MidiOutput {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MidiOutput> for MidiOutput {
        #[inline]
        fn as_ref(&self) -> &MidiOutput {
            self
        }
    }
    impl From<MidiOutput> for JsValue {
        #[inline]
        fn from(obj: MidiOutput) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MidiOutput {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MIDIOutput(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MIDIOutput(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MIDIOutput(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MidiOutput { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MidiOutput) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MidiOutput> for MidiPort {
    #[inline]
    fn from(obj: MidiOutput) -> MidiPort {
        use wasm_bindgen::JsCast;
        MidiPort::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<MidiPort> for MidiOutput {
    #[inline]
    fn as_ref(&self) -> &MidiPort {
        use wasm_bindgen::JsCast;
        MidiPort::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MidiOutput> for EventTarget {
    #[inline]
    fn from(obj: MidiOutput) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MidiOutput {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MidiOutput> for ::js_sys::Object {
    #[inline]
    fn from(obj: MidiOutput) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MidiOutput {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MidiOutput",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_MIDIOutput() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiOutput as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MidiOutput {
    #[cfg(all(feature = "MidiOutput",))]
    #[allow(bad_style)]
    #[doc = "The `clear()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/clear)\n\n*This API requires the following crate features to be activated: `MidiOutput`*"]
    #[allow(clippy::all)]
    pub fn clear(&self) {
        #[cfg(all(feature = "MidiOutput",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_MIDIOutput(
                self_: <&MidiOutput as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_MIDIOutput(
            self_: <&MidiOutput as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiOutput as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_MIDIOutput(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MidiOutput",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_MIDIOutput() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MidiOutput as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MidiOutput {
    #[cfg(all(feature = "MidiOutput",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/send)\n\n*This API requires the following crate features to be activated: `MidiOutput`*"]
    #[allow(clippy::all)]
    pub fn send(&self, data: &::wasm_bindgen::JsValue) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MidiOutput",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_MIDIOutput(
                self_: <&MidiOutput as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_MIDIOutput(
            self_: <&MidiOutput as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiOutput as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_send_MIDIOutput(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MidiOutput",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_timestamp_MIDIOutput() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&MidiOutput as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MidiOutput {
    #[cfg(all(feature = "MidiOutput",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/send)\n\n*This API requires the following crate features to be activated: `MidiOutput`*"]
    #[allow(clippy::all)]
    pub fn send_with_timestamp(
        &self,
        data: &::wasm_bindgen::JsValue,
        timestamp: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MidiOutput",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_timestamp_MIDIOutput(
                self_: <&MidiOutput as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timestamp: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_timestamp_MIDIOutput(
            self_: <&MidiOutput as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timestamp: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            drop(timestamp);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiOutput as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                let timestamp = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timestamp);
                __widl_f_send_with_timestamp_MIDIOutput(self_, data, timestamp)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_af4c64f9fee1dabb: [u8; 363usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"})\x01\0\0\0\0\x04\0\0\x02\nMIDIOutput\x1C__widl_instanceof_MIDIOutput\0\0\0\0\x19__widl_f_clear_MIDIOutput\0\0\0\x01\nMIDIOutput\x01\0\0\x01\x01\x05self_\x05clear\0\0\0\x18__widl_f_send_MIDIOutput\x01\0\0\x01\nMIDIOutput\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0'__widl_f_send_with_timestamp_MIDIOutput\x01\0\0\x01\nMIDIOutput\x01\0\0\x01\x03\x05self_\x04data\ttimestamp\x04send\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
