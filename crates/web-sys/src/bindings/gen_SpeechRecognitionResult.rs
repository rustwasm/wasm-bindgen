use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SpeechRecognitionResult` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResult`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SpeechRecognitionResult {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SpeechRecognitionResult: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SpeechRecognitionResult {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(23u32);
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
            inform(82u32);
            inform(101u32);
            inform(115u32);
            inform(117u32);
            inform(108u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SpeechRecognitionResult {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SpeechRecognitionResult {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SpeechRecognitionResult {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SpeechRecognitionResult {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SpeechRecognitionResult {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SpeechRecognitionResult {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SpeechRecognitionResult {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SpeechRecognitionResult {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SpeechRecognitionResult {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SpeechRecognitionResult>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SpeechRecognitionResult {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SpeechRecognitionResult {
        #[inline]
        fn from(obj: JsValue) -> SpeechRecognitionResult {
            SpeechRecognitionResult { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SpeechRecognitionResult {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SpeechRecognitionResult> for SpeechRecognitionResult {
        #[inline]
        fn as_ref(&self) -> &SpeechRecognitionResult {
            self
        }
    }
    impl From<SpeechRecognitionResult> for JsValue {
        #[inline]
        fn from(obj: SpeechRecognitionResult) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SpeechRecognitionResult {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SpeechRecognitionResult(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SpeechRecognitionResult(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SpeechRecognitionResult(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SpeechRecognitionResult { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SpeechRecognitionResult) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SpeechRecognitionResult> for ::js_sys::Object {
    #[inline]
    fn from(obj: SpeechRecognitionResult) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SpeechRecognitionResult {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "SpeechRecognitionAlternative",
    feature = "SpeechRecognitionResult",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_SpeechRecognitionResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognitionResult as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SpeechRecognitionAlternative as WasmDescribe>::describe();
}
impl SpeechRecognitionResult {
    #[cfg(all(
        feature = "SpeechRecognitionAlternative",
        feature = "SpeechRecognitionResult",
    ))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/item)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`, `SpeechRecognitionResult`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> SpeechRecognitionAlternative {
        #[cfg(all(
            feature = "SpeechRecognitionAlternative",
            feature = "SpeechRecognitionResult",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_SpeechRecognitionResult(
                self_: <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SpeechRecognitionAlternative as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_SpeechRecognitionResult(
            self_: <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SpeechRecognitionAlternative as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_SpeechRecognitionResult(self_, index)
            };
            <SpeechRecognitionAlternative as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SpeechRecognitionAlternative",
    feature = "SpeechRecognitionResult",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_SpeechRecognitionResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognitionResult as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<SpeechRecognitionAlternative> as WasmDescribe>::describe();
}
impl SpeechRecognitionResult {
    #[cfg(all(
        feature = "SpeechRecognitionAlternative",
        feature = "SpeechRecognitionResult",
    ))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`, `SpeechRecognitionResult`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Option<SpeechRecognitionAlternative> {
        #[cfg(all(
            feature = "SpeechRecognitionAlternative",
            feature = "SpeechRecognitionResult",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_SpeechRecognitionResult(
                self_: <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SpeechRecognitionAlternative> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_SpeechRecognitionResult(
            self_: <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SpeechRecognitionAlternative> as wasm_bindgen::convert::FromWasmAbi>::Abi
        {
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
                    <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_SpeechRecognitionResult(self_, index)
            };
            <Option<SpeechRecognitionAlternative> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            )
        }
    }
}
#[cfg(all(feature = "SpeechRecognitionResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_SpeechRecognitionResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognitionResult as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl SpeechRecognitionResult {
    #[cfg(all(feature = "SpeechRecognitionResult",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/length)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResult`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "SpeechRecognitionResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_SpeechRecognitionResult(
                self_: <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_SpeechRecognitionResult(
            self_: <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_length_SpeechRecognitionResult(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognitionResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_final_SpeechRecognitionResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognitionResult as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SpeechRecognitionResult {
    #[cfg(all(feature = "SpeechRecognitionResult",))]
    #[allow(bad_style)]
    #[doc = "The `isFinal` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/isFinal)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResult`*"]
    #[allow(clippy::all)]
    pub fn is_final(&self) -> bool {
        #[cfg(all(feature = "SpeechRecognitionResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_final_SpeechRecognitionResult(
                self_: <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_final_SpeechRecognitionResult(
            self_: <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognitionResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_is_final_SpeechRecognitionResult(self_)
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
pub static __WASM_BINDGEN_GENERATED_7b350c4d22161903: [u8; 551usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE5\x01\0\0\0\0\x05\0\0\x02\x17SpeechRecognitionResult)__widl_instanceof_SpeechRecognitionResult\0\0\0\0%__widl_f_item_SpeechRecognitionResult\0\0\0\x01\x17SpeechRecognitionResult\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0$__widl_f_get_SpeechRecognitionResult\0\0\0\x01\x17SpeechRecognitionResult\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0'__widl_f_length_SpeechRecognitionResult\0\0\0\x01\x17SpeechRecognitionResult\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0)__widl_f_is_final_SpeechRecognitionResult\0\0\0\x01\x17SpeechRecognitionResult\x01\0\x01\x07isFinal\x01\x01\x05self_\x07isFinal\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
