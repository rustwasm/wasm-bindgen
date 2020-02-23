use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SpeechSynthesisUtterance` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SpeechSynthesisUtterance {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SpeechSynthesisUtterance: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SpeechSynthesisUtterance {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
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
            inform(85u32);
            inform(116u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(97u32);
            inform(110u32);
            inform(99u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for SpeechSynthesisUtterance {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for SpeechSynthesisUtterance {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SpeechSynthesisUtterance {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SpeechSynthesisUtterance {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SpeechSynthesisUtterance {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SpeechSynthesisUtterance {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SpeechSynthesisUtterance {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SpeechSynthesisUtterance {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SpeechSynthesisUtterance {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SpeechSynthesisUtterance>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SpeechSynthesisUtterance {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SpeechSynthesisUtterance {
        #[inline]
        fn from(obj: JsValue) -> SpeechSynthesisUtterance {
            SpeechSynthesisUtterance { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SpeechSynthesisUtterance {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SpeechSynthesisUtterance> for SpeechSynthesisUtterance {
        #[inline]
        fn as_ref(&self) -> &SpeechSynthesisUtterance {
            self
        }
    }
    impl From<SpeechSynthesisUtterance> for JsValue {
        #[inline]
        fn from(obj: SpeechSynthesisUtterance) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SpeechSynthesisUtterance {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SpeechSynthesisUtterance(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SpeechSynthesisUtterance(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SpeechSynthesisUtterance(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SpeechSynthesisUtterance { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SpeechSynthesisUtterance) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SpeechSynthesisUtterance> for EventTarget {
    #[inline]
    fn from(obj: SpeechSynthesisUtterance) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SpeechSynthesisUtterance {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SpeechSynthesisUtterance> for ::js_sys::Object {
    #[inline]
    fn from(obj: SpeechSynthesisUtterance) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SpeechSynthesisUtterance {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <SpeechSynthesisUtterance as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `new SpeechSynthesisUtterance(..)` constructor, creating a new instance of `SpeechSynthesisUtterance`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/SpeechSynthesisUtterance)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<SpeechSynthesisUtterance, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_SpeechSynthesisUtterance(
            ) -> <SpeechSynthesisUtterance as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_SpeechSynthesisUtterance(
        ) -> <SpeechSynthesisUtterance as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_SpeechSynthesisUtterance() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SpeechSynthesisUtterance as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_text_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <SpeechSynthesisUtterance as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `new SpeechSynthesisUtterance(..)` constructor, creating a new instance of `SpeechSynthesisUtterance`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/SpeechSynthesisUtterance)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn new_with_text(text: &str) -> Result<SpeechSynthesisUtterance, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_text_SpeechSynthesisUtterance(
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SpeechSynthesisUtterance as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_text_SpeechSynthesisUtterance(
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SpeechSynthesisUtterance as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_new_with_text_SpeechSynthesisUtterance(text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SpeechSynthesisUtterance as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `text` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/text)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn text(&self) -> String {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_text_SpeechSynthesisUtterance(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `text` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/text)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_text(&self, text: &str) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_set_text_SpeechSynthesisUtterance(self_, text)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lang_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `lang` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/lang)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn lang(&self) -> String {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lang_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lang_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_lang_SpeechSynthesisUtterance(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_lang_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `lang` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/lang)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_lang(&self, lang: &str) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_lang_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                lang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_lang_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let lang = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(lang);
                __widl_f_set_lang_SpeechSynthesisUtterance(self_, lang)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance", feature = "SpeechSynthesisVoice",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_voice_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<SpeechSynthesisVoice> as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance", feature = "SpeechSynthesisVoice",))]
    #[allow(bad_style)]
    #[doc = "The `voice` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/voice)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`, `SpeechSynthesisVoice`*"]
    #[allow(clippy::all)]
    pub fn voice(&self) -> Option<SpeechSynthesisVoice> {
        #[cfg(all(feature = "SpeechSynthesisUtterance", feature = "SpeechSynthesisVoice",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_voice_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SpeechSynthesisVoice> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_voice_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SpeechSynthesisVoice> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_voice_SpeechSynthesisUtterance(self_)
            };
            <Option<SpeechSynthesisVoice> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance", feature = "SpeechSynthesisVoice",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_voice_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<&SpeechSynthesisVoice> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance", feature = "SpeechSynthesisVoice",))]
    #[allow(bad_style)]
    #[doc = "The `voice` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/voice)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`, `SpeechSynthesisVoice`*"]
    #[allow(clippy::all)]
    pub fn set_voice(&self, voice: Option<&SpeechSynthesisVoice>) {
        #[cfg(all(feature = "SpeechSynthesisUtterance", feature = "SpeechSynthesisVoice",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_voice_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                voice: <Option<&SpeechSynthesisVoice> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_voice_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            voice: <Option<&SpeechSynthesisVoice> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(voice);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let voice =
                    <Option<&SpeechSynthesisVoice> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        voice,
                    );
                __widl_f_set_voice_SpeechSynthesisUtterance(self_, voice)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_volume_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `volume` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/volume)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn volume(&self) -> f32 {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_volume_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_volume_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_volume_SpeechSynthesisUtterance(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_volume_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `volume` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/volume)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_volume(&self, volume: f32) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_volume_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                volume: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_volume_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            volume: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(volume);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let volume = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(volume);
                __widl_f_set_volume_SpeechSynthesisUtterance(self_, volume)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rate_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `rate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/rate)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn rate(&self) -> f32 {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rate_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rate_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_rate_SpeechSynthesisUtterance(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rate_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `rate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/rate)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_rate(&self, rate: f32) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rate_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rate: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rate_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rate: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(rate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let rate = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rate);
                __widl_f_set_rate_SpeechSynthesisUtterance(self_, rate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pitch_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `pitch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/pitch)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn pitch(&self) -> f32 {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pitch_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pitch_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_pitch_SpeechSynthesisUtterance(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_pitch_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `pitch` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/pitch)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_pitch(&self, pitch: f32) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_pitch_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pitch: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_pitch_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pitch: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(pitch);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let pitch = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pitch);
                __widl_f_set_pitch_SpeechSynthesisUtterance(self_, pitch)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstart_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onstart)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn onstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstart_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstart_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onstart_SpeechSynthesisUtterance(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstart_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onstart)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_onstart(&self, onstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstart_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstart_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstart,
                    );
                __widl_f_set_onstart_SpeechSynthesisUtterance(self_, onstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onend_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onend)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn onend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onend_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onend_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onend_SpeechSynthesisUtterance(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onend_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onend)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_onend(&self, onend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onend_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onend_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onend,
                    );
                __widl_f_set_onend_SpeechSynthesisUtterance(self_, onend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onerror)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onerror_SpeechSynthesisUtterance(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onerror)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_SpeechSynthesisUtterance(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpause_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onpause` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onpause)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn onpause(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpause_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpause_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onpause_SpeechSynthesisUtterance(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpause_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onpause` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onpause)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_onpause(&self, onpause: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpause_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpause: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpause_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpause: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpause);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onpause =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpause,
                    );
                __widl_f_set_onpause_SpeechSynthesisUtterance(self_, onpause)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onresume_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onresume` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onresume)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn onresume(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onresume_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onresume_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onresume_SpeechSynthesisUtterance(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onresume_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onresume` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onresume)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_onresume(&self, onresume: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onresume_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onresume: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onresume_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onresume: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onresume);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onresume =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onresume,
                    );
                __widl_f_set_onresume_SpeechSynthesisUtterance(self_, onresume)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmark_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onmark` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onmark)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn onmark(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmark_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmark_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onmark_SpeechSynthesisUtterance(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmark_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onmark` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onmark)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_onmark(&self, onmark: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmark_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmark: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmark_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmark: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmark);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onmark =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmark,
                    );
                __widl_f_set_onmark_SpeechSynthesisUtterance(self_, onmark)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onboundary_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onboundary` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onboundary)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn onboundary(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onboundary_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onboundary_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onboundary_SpeechSynthesisUtterance(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onboundary_SpeechSynthesisUtterance() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechSynthesisUtterance as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechSynthesisUtterance {
    #[cfg(all(feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `onboundary` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onboundary)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn set_onboundary(&self, onboundary: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onboundary_SpeechSynthesisUtterance(
                self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onboundary : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onboundary_SpeechSynthesisUtterance(
            self_: <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onboundary: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onboundary);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisUtterance as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onboundary =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onboundary,
                    );
                __widl_f_set_onboundary_SpeechSynthesisUtterance(self_, onboundary)
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
pub static __WASM_BINDGEN_GENERATED_92ef03eb5bd66fe2: [u8; 3037usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x9B\x0B\0\0\0\0\x1D\0\0\x02\x18SpeechSynthesisUtterance*__widl_instanceof_SpeechSynthesisUtterance\0\0\0\0%__widl_f_new_SpeechSynthesisUtterance\x01\0\0\x01\x18SpeechSynthesisUtterance\0\x01\0\x03new\0\0\0/__widl_f_new_with_text_SpeechSynthesisUtterance\x01\0\0\x01\x18SpeechSynthesisUtterance\0\x01\x01\x04text\x03new\0\0\0&__widl_f_text_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x04text\x01\x01\x05self_\x04text\0\0\0*__widl_f_set_text_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x04text\x01\x02\x05self_\x04text\x04text\0\0\0&__widl_f_lang_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x04lang\x01\x01\x05self_\x04lang\0\0\0*__widl_f_set_lang_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x04lang\x01\x02\x05self_\x04lang\x04lang\0\0\0'__widl_f_voice_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x05voice\x01\x01\x05self_\x05voice\0\0\0+__widl_f_set_voice_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x05voice\x01\x02\x05self_\x05voice\x05voice\0\0\0(__widl_f_volume_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x06volume\x01\x01\x05self_\x06volume\0\0\0,__widl_f_set_volume_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x06volume\x01\x02\x05self_\x06volume\x06volume\0\0\0&__widl_f_rate_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x04rate\x01\x01\x05self_\x04rate\0\0\0*__widl_f_set_rate_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x04rate\x01\x02\x05self_\x04rate\x04rate\0\0\0'__widl_f_pitch_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x05pitch\x01\x01\x05self_\x05pitch\0\0\0+__widl_f_set_pitch_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x05pitch\x01\x02\x05self_\x05pitch\x05pitch\0\0\0)__widl_f_onstart_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x07onstart\x01\x01\x05self_\x07onstart\0\0\0-__widl_f_set_onstart_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x07onstart\x01\x02\x05self_\x07onstart\x07onstart\0\0\0'__widl_f_onend_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x05onend\x01\x01\x05self_\x05onend\0\0\0+__widl_f_set_onend_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x05onend\x01\x02\x05self_\x05onend\x05onend\0\0\0)__widl_f_onerror_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0-__widl_f_set_onerror_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0)__widl_f_onpause_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x07onpause\x01\x01\x05self_\x07onpause\0\0\0-__widl_f_set_onpause_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x07onpause\x01\x02\x05self_\x07onpause\x07onpause\0\0\0*__widl_f_onresume_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x08onresume\x01\x01\x05self_\x08onresume\0\0\0.__widl_f_set_onresume_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x08onresume\x01\x02\x05self_\x08onresume\x08onresume\0\0\0(__widl_f_onmark_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\x06onmark\x01\x01\x05self_\x06onmark\0\0\0,__widl_f_set_onmark_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\x06onmark\x01\x02\x05self_\x06onmark\x06onmark\0\0\0,__widl_f_onboundary_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x01\nonboundary\x01\x01\x05self_\nonboundary\0\0\00__widl_f_set_onboundary_SpeechSynthesisUtterance\0\0\0\x01\x18SpeechSynthesisUtterance\x01\0\x02\nonboundary\x01\x02\x05self_\nonboundary\nonboundary\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
