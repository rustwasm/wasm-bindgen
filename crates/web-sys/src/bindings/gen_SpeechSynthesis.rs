use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SpeechSynthesis` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SpeechSynthesis {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SpeechSynthesis: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SpeechSynthesis {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(83u32);
            inform(112u32);
            inform(101u32);
            inform(101u32);
            inform(99u32);
            inform(104u32);
            inform(83u32);
            inform(121u32);
            inform(110u32);
            inform(116u32);
            inform(104u32);
            inform(101u32);
            inform(115u32);
            inform(105u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for SpeechSynthesis {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for SpeechSynthesis {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SpeechSynthesis {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SpeechSynthesis {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SpeechSynthesis {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SpeechSynthesis {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SpeechSynthesis {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SpeechSynthesis {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SpeechSynthesis {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SpeechSynthesis>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SpeechSynthesis {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SpeechSynthesis {
        #[inline]
        fn from(obj: JsValue) -> SpeechSynthesis {
            SpeechSynthesis { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SpeechSynthesis {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SpeechSynthesis> for SpeechSynthesis {
        #[inline]
        fn as_ref(&self) -> &SpeechSynthesis {
            self
        }
    }
    impl From<SpeechSynthesis> for JsValue {
        #[inline]
        fn from(obj: SpeechSynthesis) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SpeechSynthesis {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SpeechSynthesis(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SpeechSynthesis(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SpeechSynthesis(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SpeechSynthesis { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SpeechSynthesis) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SpeechSynthesis> for EventTarget {
    #[inline]
    fn from(obj: SpeechSynthesis) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SpeechSynthesis {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SpeechSynthesis> for ::js_sys::Object {
    #[inline]
    fn from(obj: SpeechSynthesis) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SpeechSynthesis {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SpeechSynthesis",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cancel_SpeechSynthesis() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesis as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesis {
    #[cfg(all(feature = "SpeechSynthesis",))]
    #[allow(bad_style)]
    #[doc = "The `cancel()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/cancel)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    #[allow(clippy::all)]
    pub fn cancel(&self) {
        #[cfg(all(feature = "SpeechSynthesis",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cancel_SpeechSynthesis(
                self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cancel_SpeechSynthesis(
            self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cancel_SpeechSynthesis(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesis",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_voices_SpeechSynthesis() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesis as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl SpeechSynthesis {
    #[cfg(all(feature = "SpeechSynthesis",))]
    #[allow(bad_style)]
    #[doc = "The `getVoices()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/getVoices)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    #[allow(clippy::all)]
    pub fn get_voices(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "SpeechSynthesis",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_voices_SpeechSynthesis(
                self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_voices_SpeechSynthesis(
            self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_voices_SpeechSynthesis(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesis",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pause_SpeechSynthesis() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesis as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesis {
    #[cfg(all(feature = "SpeechSynthesis",))]
    #[allow(bad_style)]
    #[doc = "The `pause()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pause)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    #[allow(clippy::all)]
    pub fn pause(&self) {
        #[cfg(all(feature = "SpeechSynthesis",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pause_SpeechSynthesis(
                self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pause_SpeechSynthesis(
            self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pause_SpeechSynthesis(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesis",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_resume_SpeechSynthesis() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesis as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesis {
    #[cfg(all(feature = "SpeechSynthesis",))]
    #[allow(bad_style)]
    #[doc = "The `resume()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/resume)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    #[allow(clippy::all)]
    pub fn resume(&self) {
        #[cfg(all(feature = "SpeechSynthesis",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_resume_SpeechSynthesis(
                self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_resume_SpeechSynthesis(
            self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_resume_SpeechSynthesis(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesis", feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_speak_SpeechSynthesis() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesis as WasmDescribe>::describe();
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesis {
    #[cfg(all(feature = "SpeechSynthesis", feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `speak()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speak)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`, `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn speak(&self, utterance: &SpeechSynthesisUtterance) {
        #[cfg(all(feature = "SpeechSynthesis", feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_speak_SpeechSynthesis(
                self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                utterance: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_speak_SpeechSynthesis(
            self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            utterance: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(utterance);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let utterance =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        utterance,
                    );
                __widl_f_speak_SpeechSynthesis(self_, utterance)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesis",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pending_SpeechSynthesis() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesis as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SpeechSynthesis {
    #[cfg(all(feature = "SpeechSynthesis",))]
    #[allow(bad_style)]
    #[doc = "The `pending` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pending)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    #[allow(clippy::all)]
    pub fn pending(&self) -> bool {
        #[cfg(all(feature = "SpeechSynthesis",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pending_SpeechSynthesis(
                self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pending_SpeechSynthesis(
            self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pending_SpeechSynthesis(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesis",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_speaking_SpeechSynthesis() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesis as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SpeechSynthesis {
    #[cfg(all(feature = "SpeechSynthesis",))]
    #[allow(bad_style)]
    #[doc = "The `speaking` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speaking)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    #[allow(clippy::all)]
    pub fn speaking(&self) -> bool {
        #[cfg(all(feature = "SpeechSynthesis",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_speaking_SpeechSynthesis(
                self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_speaking_SpeechSynthesis(
            self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_speaking_SpeechSynthesis(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesis",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_paused_SpeechSynthesis() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesis as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SpeechSynthesis {
    #[cfg(all(feature = "SpeechSynthesis",))]
    #[allow(bad_style)]
    #[doc = "The `paused` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/paused)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    #[allow(clippy::all)]
    pub fn paused(&self) -> bool {
        #[cfg(all(feature = "SpeechSynthesis",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_paused_SpeechSynthesis(
                self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_paused_SpeechSynthesis(
            self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_paused_SpeechSynthesis(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesis",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onvoiceschanged_SpeechSynthesis() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesis as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechSynthesis {
    #[cfg(all(feature = "SpeechSynthesis",))]
    #[allow(bad_style)]
    #[doc = "The `onvoiceschanged` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    #[allow(clippy::all)]
    pub fn onvoiceschanged(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechSynthesis",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onvoiceschanged_SpeechSynthesis(
                self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onvoiceschanged_SpeechSynthesis(
            self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onvoiceschanged_SpeechSynthesis(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesis",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onvoiceschanged_SpeechSynthesis() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesis as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesis {
    #[cfg(all(feature = "SpeechSynthesis",))]
    #[allow(bad_style)]
    #[doc = "The `onvoiceschanged` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    #[allow(clippy::all)]
    pub fn set_onvoiceschanged(&self, onvoiceschanged: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechSynthesis",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onvoiceschanged_SpeechSynthesis(
                self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onvoiceschanged : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onvoiceschanged_SpeechSynthesis(
            self_: <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onvoiceschanged : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onvoiceschanged);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesis as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onvoiceschanged =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onvoiceschanged,
                    );
                __widl_f_set_onvoiceschanged_SpeechSynthesis(self_, onvoiceschanged)
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
pub static __WASM_BINDGEN_GENERATED_bef898e5b0fcbcb1: [u8; 1020usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xBA\x03\0\0\0\0\x0B\0\0\x02\x0FSpeechSynthesis!__widl_instanceof_SpeechSynthesis\0\0\0\0\x1F__widl_f_cancel_SpeechSynthesis\0\0\0\x01\x0FSpeechSynthesis\x01\0\0\x01\x01\x05self_\x06cancel\0\0\0#__widl_f_get_voices_SpeechSynthesis\0\0\0\x01\x0FSpeechSynthesis\x01\0\0\x01\x01\x05self_\tgetVoices\0\0\0\x1E__widl_f_pause_SpeechSynthesis\0\0\0\x01\x0FSpeechSynthesis\x01\0\0\x01\x01\x05self_\x05pause\0\0\0\x1F__widl_f_resume_SpeechSynthesis\0\0\0\x01\x0FSpeechSynthesis\x01\0\0\x01\x01\x05self_\x06resume\0\0\0\x1E__widl_f_speak_SpeechSynthesis\0\0\0\x01\x0FSpeechSynthesis\x01\0\0\x01\x02\x05self_\tutterance\x05speak\0\0\0 __widl_f_pending_SpeechSynthesis\0\0\0\x01\x0FSpeechSynthesis\x01\0\x01\x07pending\x01\x01\x05self_\x07pending\0\0\0!__widl_f_speaking_SpeechSynthesis\0\0\0\x01\x0FSpeechSynthesis\x01\0\x01\x08speaking\x01\x01\x05self_\x08speaking\0\0\0\x1F__widl_f_paused_SpeechSynthesis\0\0\0\x01\x0FSpeechSynthesis\x01\0\x01\x06paused\x01\x01\x05self_\x06paused\0\0\0(__widl_f_onvoiceschanged_SpeechSynthesis\0\0\0\x01\x0FSpeechSynthesis\x01\0\x01\x0Fonvoiceschanged\x01\x01\x05self_\x0Fonvoiceschanged\0\0\0,__widl_f_set_onvoiceschanged_SpeechSynthesis\0\0\0\x01\x0FSpeechSynthesis\x01\0\x02\x0Fonvoiceschanged\x01\x02\x05self_\x0Fonvoiceschanged\x0Fonvoiceschanged\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
