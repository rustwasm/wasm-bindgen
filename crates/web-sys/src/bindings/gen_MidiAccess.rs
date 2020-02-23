use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MIDIAccess` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess)\n\n*This API requires the following crate features to be activated: `MidiAccess`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MidiAccess {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MidiAccess: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MidiAccess {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(77u32);
            inform(73u32);
            inform(68u32);
            inform(73u32);
            inform(65u32);
            inform(99u32);
            inform(99u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for MidiAccess {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for MidiAccess {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MidiAccess {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MidiAccess {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MidiAccess {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MidiAccess {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MidiAccess {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MidiAccess {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MidiAccess {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MidiAccess>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MidiAccess {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MidiAccess {
        #[inline]
        fn from(obj: JsValue) -> MidiAccess {
            MidiAccess { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MidiAccess {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MidiAccess> for MidiAccess {
        #[inline]
        fn as_ref(&self) -> &MidiAccess {
            self
        }
    }
    impl From<MidiAccess> for JsValue {
        #[inline]
        fn from(obj: MidiAccess) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MidiAccess {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MIDIAccess(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MIDIAccess(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MIDIAccess(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MidiAccess { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MidiAccess) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MidiAccess> for EventTarget {
    #[inline]
    fn from(obj: MidiAccess) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MidiAccess {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MidiAccess> for ::js_sys::Object {
    #[inline]
    fn from(obj: MidiAccess) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MidiAccess {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MidiAccess", feature = "MidiInputMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_inputs_MIDIAccess() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiAccess as WasmDescribe>::describe();
    <MidiInputMap as WasmDescribe>::describe();
}
impl MidiAccess {
    #[cfg(all(feature = "MidiAccess", feature = "MidiInputMap",))]
    #[allow(bad_style)]
    #[doc = "The `inputs` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/inputs)\n\n*This API requires the following crate features to be activated: `MidiAccess`, `MidiInputMap`*"]
    #[allow(clippy::all)]
    pub fn inputs(&self) -> MidiInputMap {
        #[cfg(all(feature = "MidiAccess", feature = "MidiInputMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_inputs_MIDIAccess(
                self_: <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MidiInputMap as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_inputs_MIDIAccess(
            self_: <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MidiInputMap as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_inputs_MIDIAccess(self_)
            };
            <MidiInputMap as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiAccess", feature = "MidiOutputMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_outputs_MIDIAccess() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiAccess as WasmDescribe>::describe();
    <MidiOutputMap as WasmDescribe>::describe();
}
impl MidiAccess {
    #[cfg(all(feature = "MidiAccess", feature = "MidiOutputMap",))]
    #[allow(bad_style)]
    #[doc = "The `outputs` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/outputs)\n\n*This API requires the following crate features to be activated: `MidiAccess`, `MidiOutputMap`*"]
    #[allow(clippy::all)]
    pub fn outputs(&self) -> MidiOutputMap {
        #[cfg(all(feature = "MidiAccess", feature = "MidiOutputMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_outputs_MIDIAccess(
                self_: <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MidiOutputMap as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_outputs_MIDIAccess(
            self_: <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MidiOutputMap as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_outputs_MIDIAccess(self_)
            };
            <MidiOutputMap as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiAccess",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstatechange_MIDIAccess() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiAccess as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MidiAccess {
    #[cfg(all(feature = "MidiAccess",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/onstatechange)\n\n*This API requires the following crate features to be activated: `MidiAccess`*"]
    #[allow(clippy::all)]
    pub fn onstatechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MidiAccess",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstatechange_MIDIAccess(
                self_: <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstatechange_MIDIAccess(
            self_: <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstatechange_MIDIAccess(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MidiAccess",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstatechange_MIDIAccess() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MidiAccess as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MidiAccess {
    #[cfg(all(feature = "MidiAccess",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/onstatechange)\n\n*This API requires the following crate features to be activated: `MidiAccess`*"]
    #[allow(clippy::all)]
    pub fn set_onstatechange(&self, onstatechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MidiAccess",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstatechange_MIDIAccess(
                self_: <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstatechange_MIDIAccess(
            self_: <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstatechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstatechange,
                    );
                __widl_f_set_onstatechange_MIDIAccess(self_, onstatechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MidiAccess",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sysex_enabled_MIDIAccess() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MidiAccess as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MidiAccess {
    #[cfg(all(feature = "MidiAccess",))]
    #[allow(bad_style)]
    #[doc = "The `sysexEnabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/sysexEnabled)\n\n*This API requires the following crate features to be activated: `MidiAccess`*"]
    #[allow(clippy::all)]
    pub fn sysex_enabled(&self) -> bool {
        #[cfg(all(feature = "MidiAccess",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sysex_enabled_MIDIAccess(
                self_: <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sysex_enabled_MIDIAccess(
            self_: <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MidiAccess as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sysex_enabled_MIDIAccess(self_)
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
pub static __WASM_BINDGEN_GENERATED_f610f6b738a8d99e: [u8; 581usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x03\x02\0\0\0\0\x06\0\0\x02\nMIDIAccess\x1C__widl_instanceof_MIDIAccess\0\0\0\0\x1A__widl_f_inputs_MIDIAccess\0\0\0\x01\nMIDIAccess\x01\0\x01\x06inputs\x01\x01\x05self_\x06inputs\0\0\0\x1B__widl_f_outputs_MIDIAccess\0\0\0\x01\nMIDIAccess\x01\0\x01\x07outputs\x01\x01\x05self_\x07outputs\0\0\0!__widl_f_onstatechange_MIDIAccess\0\0\0\x01\nMIDIAccess\x01\0\x01\ronstatechange\x01\x01\x05self_\ronstatechange\0\0\0%__widl_f_set_onstatechange_MIDIAccess\0\0\0\x01\nMIDIAccess\x01\0\x02\ronstatechange\x01\x02\x05self_\ronstatechange\ronstatechange\0\0\0!__widl_f_sysex_enabled_MIDIAccess\0\0\0\x01\nMIDIAccess\x01\0\x01\x0CsysexEnabled\x01\x01\x05self_\x0CsysexEnabled\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
