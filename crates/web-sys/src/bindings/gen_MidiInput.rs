use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MIDIInput` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput)\n\n*This API requires the following crate features to be activated: `MidiInput`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MidiInput {
    obj: MidiPort,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MidiInput: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MidiInput {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(77u32);
            inform(73u32);
            inform(68u32);
            inform(73u32);
            inform(73u32);
            inform(110u32);
            inform(112u32);
            inform(117u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MidiInput {
        type Target = MidiPort;
        #[inline]
        fn deref(&self) -> &MidiPort {
            &self.obj
        }
    }
    impl IntoWasmAbi for MidiInput {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MidiInput {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MidiInput {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MidiInput {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MidiInput {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MidiInput {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MidiInput {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MidiInput {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MidiInput>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MidiInput {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MidiInput {
        #[inline]
        fn from(obj: JsValue) -> MidiInput {
            MidiInput { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MidiInput {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MidiInput> for MidiInput {
        #[inline]
        fn as_ref(&self) -> &MidiInput {
            self
        }
    }
    impl From<MidiInput> for JsValue {
        #[inline]
        fn from(obj: MidiInput) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MidiInput {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MIDIInput(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MIDIInput(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MIDIInput(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MidiInput { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MidiInput) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MidiInput> for MidiPort {
    #[inline]
    fn from(obj: MidiInput) -> MidiPort {
        use wasm_bindgen::JsCast;
        MidiPort::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<MidiPort> for MidiInput {
    #[inline]
    fn as_ref(&self) -> &MidiPort {
        use wasm_bindgen::JsCast;
        MidiPort::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MidiInput> for EventTarget {
    #[inline]
    fn from(obj: MidiInput) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MidiInput {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MidiInput> for ::js_sys::Object {
    #[inline]
    fn from(obj: MidiInput) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MidiInput {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MidiInput",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmidimessage_MIDIInput() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiInput as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MidiInput {
    #[cfg(all(feature = "MidiInput",))]
    #[allow(bad_style)]
    #[doc = "The `onmidimessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput/onmidimessage)\n\n*This API requires the following crate features to be activated: `MidiInput`*"]
    #[allow(clippy::all)]
    pub fn onmidimessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MidiInput",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmidimessage_MIDIInput(
                self_: <&MidiInput as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmidimessage_MIDIInput(
            self_: <&MidiInput as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiInput as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmidimessage_MIDIInput(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiInput",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmidimessage_MIDIInput() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MidiInput as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MidiInput {
    #[cfg(all(feature = "MidiInput",))]
    #[allow(bad_style)]
    #[doc = "The `onmidimessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput/onmidimessage)\n\n*This API requires the following crate features to be activated: `MidiInput`*"]
    #[allow(clippy::all)]
    pub fn set_onmidimessage(&self, onmidimessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MidiInput",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmidimessage_MIDIInput(
                self_: <&MidiInput as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmidimessage : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmidimessage_MIDIInput(
            self_: <&MidiInput as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmidimessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmidimessage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiInput as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmidimessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmidimessage,
                    );
                __widl_f_set_onmidimessage_MIDIInput(self_, onmidimessage)
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
pub static __WASM_BINDGEN_GENERATED_53948da48b7e8334: [u8; 343usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x15\x01\0\0\0\0\x03\0\0\x02\tMIDIInput\x1B__widl_instanceof_MIDIInput\0\0\0\0 __widl_f_onmidimessage_MIDIInput\0\0\0\x01\tMIDIInput\x01\0\x01\ronmidimessage\x01\x01\x05self_\ronmidimessage\0\0\0$__widl_f_set_onmidimessage_MIDIInput\0\0\0\x01\tMIDIInput\x01\0\x02\ronmidimessage\x01\x02\x05self_\ronmidimessage\ronmidimessage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
