use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SpeechGrammarList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SpeechGrammarList {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SpeechGrammarList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SpeechGrammarList {
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
            inform(71u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
            inform(109u32);
            inform(97u32);
            inform(114u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SpeechGrammarList {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SpeechGrammarList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SpeechGrammarList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SpeechGrammarList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SpeechGrammarList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SpeechGrammarList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SpeechGrammarList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SpeechGrammarList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SpeechGrammarList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SpeechGrammarList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SpeechGrammarList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SpeechGrammarList {
        #[inline]
        fn from(obj: JsValue) -> SpeechGrammarList {
            SpeechGrammarList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SpeechGrammarList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SpeechGrammarList> for SpeechGrammarList {
        #[inline]
        fn as_ref(&self) -> &SpeechGrammarList {
            self
        }
    }
    impl From<SpeechGrammarList> for JsValue {
        #[inline]
        fn from(obj: SpeechGrammarList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SpeechGrammarList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SpeechGrammarList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SpeechGrammarList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SpeechGrammarList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SpeechGrammarList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SpeechGrammarList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SpeechGrammarList> for ::js_sys::Object {
    #[inline]
    fn from(obj: SpeechGrammarList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SpeechGrammarList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SpeechGrammarList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_SpeechGrammarList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <SpeechGrammarList as WasmDescribe>::describe();
}
impl SpeechGrammarList {
    #[cfg(all(feature = "SpeechGrammarList",))]
    #[allow(bad_style)]
    #[doc = "The `new SpeechGrammarList(..)` constructor, creating a new instance of `SpeechGrammarList`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/SpeechGrammarList)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<SpeechGrammarList, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammarList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_SpeechGrammarList(
            ) -> <SpeechGrammarList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_SpeechGrammarList(
        ) -> <SpeechGrammarList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_SpeechGrammarList() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SpeechGrammarList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SpeechGrammarList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_from_string_SpeechGrammarList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechGrammarList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechGrammarList {
    #[cfg(all(feature = "SpeechGrammarList",))]
    #[allow(bad_style)]
    #[doc = "The `addFromString()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    #[allow(clippy::all)]
    pub fn add_from_string(&self, string: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammarList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_from_string_SpeechGrammarList(
                self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                string: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_from_string_SpeechGrammarList(
            self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            string: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(string);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let string = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(string);
                __widl_f_add_from_string_SpeechGrammarList(self_, string)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SpeechGrammarList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_from_string_with_weight_SpeechGrammarList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SpeechGrammarList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechGrammarList {
    #[cfg(all(feature = "SpeechGrammarList",))]
    #[allow(bad_style)]
    #[doc = "The `addFromString()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    #[allow(clippy::all)]
    pub fn add_from_string_with_weight(
        &self,
        string: &str,
        weight: f32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammarList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_from_string_with_weight_SpeechGrammarList(
                self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                string: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                weight: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_from_string_with_weight_SpeechGrammarList(
            self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            string: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            weight: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(string);
            drop(weight);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let string = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(string);
                let weight = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(weight);
                __widl_f_add_from_string_with_weight_SpeechGrammarList(self_, string, weight)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SpeechGrammarList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_from_uri_SpeechGrammarList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechGrammarList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechGrammarList {
    #[cfg(all(feature = "SpeechGrammarList",))]
    #[allow(bad_style)]
    #[doc = "The `addFromURI()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    #[allow(clippy::all)]
    pub fn add_from_uri(&self, src: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammarList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_from_uri_SpeechGrammarList(
                self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_from_uri_SpeechGrammarList(
            self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(src);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_add_from_uri_SpeechGrammarList(self_, src)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SpeechGrammarList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_from_uri_with_weight_SpeechGrammarList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SpeechGrammarList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SpeechGrammarList {
    #[cfg(all(feature = "SpeechGrammarList",))]
    #[allow(bad_style)]
    #[doc = "The `addFromURI()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    #[allow(clippy::all)]
    pub fn add_from_uri_with_weight(
        &self,
        src: &str,
        weight: f32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammarList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_from_uri_with_weight_SpeechGrammarList(
                self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                weight: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_from_uri_with_weight_SpeechGrammarList(
            self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            weight: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(src);
            drop(weight);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                let weight = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(weight);
                __widl_f_add_from_uri_with_weight_SpeechGrammarList(self_, src, weight)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SpeechGrammar", feature = "SpeechGrammarList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_SpeechGrammarList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechGrammarList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SpeechGrammar as WasmDescribe>::describe();
}
impl SpeechGrammarList {
    #[cfg(all(feature = "SpeechGrammar", feature = "SpeechGrammarList",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/item)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`, `SpeechGrammarList`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> Result<SpeechGrammar, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammar", feature = "SpeechGrammarList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_SpeechGrammarList(
                self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SpeechGrammar as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_SpeechGrammarList(
            self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SpeechGrammar as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_SpeechGrammarList(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SpeechGrammar as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SpeechGrammar", feature = "SpeechGrammarList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_SpeechGrammarList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechGrammarList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SpeechGrammar as WasmDescribe>::describe();
}
impl SpeechGrammarList {
    #[cfg(all(feature = "SpeechGrammar", feature = "SpeechGrammarList",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `SpeechGrammar`, `SpeechGrammarList`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Result<SpeechGrammar, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechGrammar", feature = "SpeechGrammarList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_SpeechGrammarList(
                self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SpeechGrammar as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_SpeechGrammarList(
            self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SpeechGrammar as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_SpeechGrammarList(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SpeechGrammar as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SpeechGrammarList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_SpeechGrammarList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechGrammarList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl SpeechGrammarList {
    #[cfg(all(feature = "SpeechGrammarList",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/length)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "SpeechGrammarList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_SpeechGrammarList(
                self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_SpeechGrammarList(
            self_: <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechGrammarList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_SpeechGrammarList(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8405358780a48576: [u8; 886usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}4\x03\0\0\0\0\t\0\0\x02\x11SpeechGrammarList#__widl_instanceof_SpeechGrammarList\0\0\0\0\x1E__widl_f_new_SpeechGrammarList\x01\0\0\x01\x11SpeechGrammarList\0\x01\0\x03new\0\0\0*__widl_f_add_from_string_SpeechGrammarList\x01\0\0\x01\x11SpeechGrammarList\x01\0\0\x01\x02\x05self_\x06string\raddFromString\0\0\06__widl_f_add_from_string_with_weight_SpeechGrammarList\x01\0\0\x01\x11SpeechGrammarList\x01\0\0\x01\x03\x05self_\x06string\x06weight\raddFromString\0\0\0'__widl_f_add_from_uri_SpeechGrammarList\x01\0\0\x01\x11SpeechGrammarList\x01\0\0\x01\x02\x05self_\x03src\naddFromURI\0\0\03__widl_f_add_from_uri_with_weight_SpeechGrammarList\x01\0\0\x01\x11SpeechGrammarList\x01\0\0\x01\x03\x05self_\x03src\x06weight\naddFromURI\0\0\0\x1F__widl_f_item_SpeechGrammarList\x01\0\0\x01\x11SpeechGrammarList\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0\x1E__widl_f_get_SpeechGrammarList\x01\0\0\x01\x11SpeechGrammarList\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0!__widl_f_length_SpeechGrammarList\0\0\0\x01\x11SpeechGrammarList\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
