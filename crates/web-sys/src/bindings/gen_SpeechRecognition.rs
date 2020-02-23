use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SpeechRecognition` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SpeechRecognition {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SpeechRecognition: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SpeechRecognition {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(83u32);
            inform(112u32);
            inform(101u32);
            inform(101u32);
            inform(99u32);
            inform(104u32);
            inform(82u32);
            inform(101u32);
            inform(99u32);
            inform(111u32);
            inform(103u32);
            inform(110u32);
            inform(105u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for SpeechRecognition {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for SpeechRecognition {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SpeechRecognition {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SpeechRecognition {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SpeechRecognition {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SpeechRecognition {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SpeechRecognition {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SpeechRecognition {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SpeechRecognition {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SpeechRecognition>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SpeechRecognition {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SpeechRecognition {
        #[inline]
        fn from(obj: JsValue) -> SpeechRecognition {
            SpeechRecognition { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SpeechRecognition {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SpeechRecognition> for SpeechRecognition {
        #[inline]
        fn as_ref(&self) -> &SpeechRecognition {
            self
        }
    }
    impl From<SpeechRecognition> for JsValue {
        #[inline]
        fn from(obj: SpeechRecognition) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SpeechRecognition {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SpeechRecognition(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SpeechRecognition(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SpeechRecognition(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SpeechRecognition { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SpeechRecognition) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SpeechRecognition> for EventTarget {
    #[inline]
    fn from(obj: SpeechRecognition) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SpeechRecognition {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SpeechRecognition> for ::js_sys::Object {
    #[inline]
    fn from(obj: SpeechRecognition) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SpeechRecognition {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <SpeechRecognition as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `new SpeechRecognition(..)` constructor, creating a new instance of `SpeechRecognition`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/SpeechRecognition)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<SpeechRecognition, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_SpeechRecognition(
            ) -> <SpeechRecognition as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_SpeechRecognition(
        ) -> <SpeechRecognition as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_SpeechRecognition() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SpeechRecognition as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_abort_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `abort()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/abort)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn abort(&self) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_abort_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_abort_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_abort_SpeechRecognition(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn start(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_SpeechRecognition(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaStream", feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_with_stream_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "MediaStream", feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)\n\n*This API requires the following crate features to be activated: `MediaStream`, `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn start_with_stream(&self, stream: &MediaStream) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaStream", feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_with_stream_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_with_stream_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(stream);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                __widl_f_start_with_stream_SpeechRecognition(self_, stream)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/stop)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn stop(&self) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stop_SpeechRecognition(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechGrammarList", feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_grammars_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <SpeechGrammarList as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechGrammarList", feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `grammars` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`, `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn grammars(&self) -> SpeechGrammarList {
        #[cfg(all(feature = "SpeechGrammarList", feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_grammars_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SpeechGrammarList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_grammars_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SpeechGrammarList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_grammars_SpeechRecognition(self_)
            };
            <SpeechGrammarList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechGrammarList", feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_grammars_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <&SpeechGrammarList as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechGrammarList", feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `grammars` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`, `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_grammars(&self, grammars: &SpeechGrammarList) {
        #[cfg(all(feature = "SpeechGrammarList", feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_grammars_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                grammars: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_grammars_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            grammars: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(grammars);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let grammars =
                    <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(grammars);
                __widl_f_set_grammars_SpeechRecognition(self_, grammars)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lang_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `lang` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn lang(&self) -> String {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lang_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lang_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_lang_SpeechRecognition(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_lang_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `lang` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_lang(&self, lang: &str) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_lang_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                lang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_lang_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            lang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(lang);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let lang = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(lang);
                __widl_f_set_lang_SpeechRecognition(self_, lang)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_continuous_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `continuous` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn continuous(&self) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_continuous_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_continuous_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_continuous_SpeechRecognition(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_continuous_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `continuous` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_continuous(&self, continuous: bool) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_continuous_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                continuous: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_continuous_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            continuous: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(continuous);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let continuous = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(continuous);
                __widl_f_set_continuous_SpeechRecognition(self_, continuous)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_interim_results_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `interimResults` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn interim_results(&self) -> bool {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_interim_results_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_interim_results_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_interim_results_SpeechRecognition(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_interim_results_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `interimResults` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_interim_results(&self, interim_results: bool) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_interim_results_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                interim_results: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_interim_results_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            interim_results: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(interim_results);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let interim_results =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(interim_results);
                __widl_f_set_interim_results_SpeechRecognition(self_, interim_results)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_alternatives_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `maxAlternatives` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn max_alternatives(&self) -> u32 {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_alternatives_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_alternatives_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_alternatives_SpeechRecognition(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_max_alternatives_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `maxAlternatives` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_max_alternatives(&self, max_alternatives: u32) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_max_alternatives_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_alternatives: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_max_alternatives_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_alternatives: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(max_alternatives);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max_alternatives =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_alternatives);
                __widl_f_set_max_alternatives_SpeechRecognition(self_, max_alternatives)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_service_uri_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `serviceURI` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/serviceURI)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn service_uri(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_service_uri_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_service_uri_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_service_uri_SpeechRecognition(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_service_uri_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `serviceURI` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/serviceURI)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_service_uri(&self, service_uri: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_service_uri_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                service_uri: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_service_uri_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            service_uri: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(service_uri);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let service_uri =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(service_uri);
                __widl_f_set_service_uri_SpeechRecognition(self_, service_uri)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onaudiostart_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onaudiostart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn onaudiostart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onaudiostart_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onaudiostart_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onaudiostart_SpeechRecognition(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onaudiostart_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onaudiostart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_onaudiostart(&self, onaudiostart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onaudiostart_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onaudiostart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onaudiostart_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onaudiostart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onaudiostart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onaudiostart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onaudiostart,
                    );
                __widl_f_set_onaudiostart_SpeechRecognition(self_, onaudiostart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsoundstart_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onsoundstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn onsoundstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsoundstart_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsoundstart_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsoundstart_SpeechRecognition(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsoundstart_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onsoundstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_onsoundstart(&self, onsoundstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsoundstart_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsoundstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsoundstart_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsoundstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onsoundstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsoundstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsoundstart,
                    );
                __widl_f_set_onsoundstart_SpeechRecognition(self_, onsoundstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onspeechstart_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onspeechstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn onspeechstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onspeechstart_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onspeechstart_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onspeechstart_SpeechRecognition(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onspeechstart_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onspeechstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_onspeechstart(&self, onspeechstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onspeechstart_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onspeechstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onspeechstart_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onspeechstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onspeechstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onspeechstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onspeechstart,
                    );
                __widl_f_set_onspeechstart_SpeechRecognition(self_, onspeechstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onspeechend_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onspeechend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn onspeechend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onspeechend_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onspeechend_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onspeechend_SpeechRecognition(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onspeechend_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onspeechend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_onspeechend(&self, onspeechend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onspeechend_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onspeechend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onspeechend_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onspeechend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onspeechend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onspeechend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onspeechend,
                    );
                __widl_f_set_onspeechend_SpeechRecognition(self_, onspeechend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsoundend_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onsoundend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn onsoundend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsoundend_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsoundend_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsoundend_SpeechRecognition(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsoundend_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onsoundend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_onsoundend(&self, onsoundend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsoundend_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsoundend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsoundend_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsoundend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onsoundend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsoundend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsoundend,
                    );
                __widl_f_set_onsoundend_SpeechRecognition(self_, onsoundend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onaudioend_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onaudioend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn onaudioend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onaudioend_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onaudioend_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onaudioend_SpeechRecognition(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onaudioend_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onaudioend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_onaudioend(&self, onaudioend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onaudioend_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onaudioend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onaudioend_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onaudioend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onaudioend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onaudioend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onaudioend,
                    );
                __widl_f_set_onaudioend_SpeechRecognition(self_, onaudioend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onresult_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onresult` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn onresult(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onresult_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onresult_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onresult_SpeechRecognition(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onresult_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onresult` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_onresult(&self, onresult: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onresult_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onresult: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onresult_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onresult: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onresult);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onresult =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onresult,
                    );
                __widl_f_set_onresult_SpeechRecognition(self_, onresult)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onnomatch_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onnomatch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn onnomatch(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onnomatch_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onnomatch_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onnomatch_SpeechRecognition(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onnomatch_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onnomatch` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_onnomatch(&self, onnomatch: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onnomatch_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onnomatch: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onnomatch_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onnomatch: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onnomatch);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onnomatch =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onnomatch,
                    );
                __widl_f_set_onnomatch_SpeechRecognition(self_, onnomatch)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_SpeechRecognition(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_SpeechRecognition(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstart_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn onstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstart_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstart_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstart_SpeechRecognition(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstart_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_onstart(&self, onstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstart_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstart_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstart,
                    );
                __widl_f_set_onstart_SpeechRecognition(self_, onstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onend_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn onend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onend_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onend_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onend_SpeechRecognition(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onend_SpeechRecognition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognition as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechRecognition {
    #[cfg(all(feature = "SpeechRecognition",))]
    #[allow(bad_style)]
    #[doc = "The `onend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    #[allow(clippy::all)]
    pub fn set_onend(&self, onend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechRecognition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onend_SpeechRecognition(
                self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onend_SpeechRecognition(
            self_: <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechRecognition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onend,
                    );
                __widl_f_set_onend_SpeechRecognition(self_, onend)
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
pub static __WASM_BINDGEN_GENERATED_a9b29b5475430d6f: [u8; 4036usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x82\x0F\0\0\0\0(\0\0\x02\x11SpeechRecognition#__widl_instanceof_SpeechRecognition\0\0\0\0\x1E__widl_f_new_SpeechRecognition\x01\0\0\x01\x11SpeechRecognition\0\x01\0\x03new\0\0\0 __widl_f_abort_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\0\x01\x01\x05self_\x05abort\0\0\0 __widl_f_start_SpeechRecognition\x01\0\0\x01\x11SpeechRecognition\x01\0\0\x01\x01\x05self_\x05start\0\0\0,__widl_f_start_with_stream_SpeechRecognition\x01\0\0\x01\x11SpeechRecognition\x01\0\0\x01\x02\x05self_\x06stream\x05start\0\0\0\x1F__widl_f_stop_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\0\x01\x01\x05self_\x04stop\0\0\0#__widl_f_grammars_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\x08grammars\x01\x01\x05self_\x08grammars\0\0\0'__widl_f_set_grammars_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\x08grammars\x01\x02\x05self_\x08grammars\x08grammars\0\0\0\x1F__widl_f_lang_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\x04lang\x01\x01\x05self_\x04lang\0\0\0#__widl_f_set_lang_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\x04lang\x01\x02\x05self_\x04lang\x04lang\0\0\0%__widl_f_continuous_SpeechRecognition\x01\0\0\x01\x11SpeechRecognition\x01\0\x01\ncontinuous\x01\x01\x05self_\ncontinuous\0\0\0)__widl_f_set_continuous_SpeechRecognition\x01\0\0\x01\x11SpeechRecognition\x01\0\x02\ncontinuous\x01\x02\x05self_\ncontinuous\ncontinuous\0\0\0*__widl_f_interim_results_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\x0EinterimResults\x01\x01\x05self_\x0EinterimResults\0\0\0.__widl_f_set_interim_results_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\x0EinterimResults\x01\x02\x05self_\x0Finterim_results\x0EinterimResults\0\0\0+__widl_f_max_alternatives_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\x0FmaxAlternatives\x01\x01\x05self_\x0FmaxAlternatives\0\0\0/__widl_f_set_max_alternatives_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\x0FmaxAlternatives\x01\x02\x05self_\x10max_alternatives\x0FmaxAlternatives\0\0\0&__widl_f_service_uri_SpeechRecognition\x01\0\0\x01\x11SpeechRecognition\x01\0\x01\nserviceURI\x01\x01\x05self_\nserviceURI\0\0\0*__widl_f_set_service_uri_SpeechRecognition\x01\0\0\x01\x11SpeechRecognition\x01\0\x02\nserviceURI\x01\x02\x05self_\x0Bservice_uri\nserviceURI\0\0\0'__widl_f_onaudiostart_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\x0Conaudiostart\x01\x01\x05self_\x0Conaudiostart\0\0\0+__widl_f_set_onaudiostart_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\x0Conaudiostart\x01\x02\x05self_\x0Conaudiostart\x0Conaudiostart\0\0\0'__widl_f_onsoundstart_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\x0Consoundstart\x01\x01\x05self_\x0Consoundstart\0\0\0+__widl_f_set_onsoundstart_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\x0Consoundstart\x01\x02\x05self_\x0Consoundstart\x0Consoundstart\0\0\0(__widl_f_onspeechstart_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\ronspeechstart\x01\x01\x05self_\ronspeechstart\0\0\0,__widl_f_set_onspeechstart_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\ronspeechstart\x01\x02\x05self_\ronspeechstart\ronspeechstart\0\0\0&__widl_f_onspeechend_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\x0Bonspeechend\x01\x01\x05self_\x0Bonspeechend\0\0\0*__widl_f_set_onspeechend_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\x0Bonspeechend\x01\x02\x05self_\x0Bonspeechend\x0Bonspeechend\0\0\0%__widl_f_onsoundend_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\nonsoundend\x01\x01\x05self_\nonsoundend\0\0\0)__widl_f_set_onsoundend_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\nonsoundend\x01\x02\x05self_\nonsoundend\nonsoundend\0\0\0%__widl_f_onaudioend_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\nonaudioend\x01\x01\x05self_\nonaudioend\0\0\0)__widl_f_set_onaudioend_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\nonaudioend\x01\x02\x05self_\nonaudioend\nonaudioend\0\0\0#__widl_f_onresult_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\x08onresult\x01\x01\x05self_\x08onresult\0\0\0'__widl_f_set_onresult_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\x08onresult\x01\x02\x05self_\x08onresult\x08onresult\0\0\0$__widl_f_onnomatch_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\tonnomatch\x01\x01\x05self_\tonnomatch\0\0\0(__widl_f_set_onnomatch_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\tonnomatch\x01\x02\x05self_\tonnomatch\tonnomatch\0\0\0\"__widl_f_onerror_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0&__widl_f_set_onerror_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\"__widl_f_onstart_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\x07onstart\x01\x01\x05self_\x07onstart\0\0\0&__widl_f_set_onstart_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\x07onstart\x01\x02\x05self_\x07onstart\x07onstart\0\0\0 __widl_f_onend_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x01\x05onend\x01\x01\x05self_\x05onend\0\0\0$__widl_f_set_onend_SpeechRecognition\0\0\0\x01\x11SpeechRecognition\x01\0\x02\x05onend\x01\x02\x05self_\x05onend\x05onend\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
