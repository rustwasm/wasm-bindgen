use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SpeechRecognitionResultList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResultList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SpeechRecognitionResultList {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SpeechRecognitionResultList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SpeechRecognitionResultList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(27u32);
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
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SpeechRecognitionResultList {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SpeechRecognitionResultList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SpeechRecognitionResultList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SpeechRecognitionResultList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SpeechRecognitionResultList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SpeechRecognitionResultList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SpeechRecognitionResultList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SpeechRecognitionResultList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SpeechRecognitionResultList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SpeechRecognitionResultList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SpeechRecognitionResultList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SpeechRecognitionResultList {
        #[inline]
        fn from(obj: JsValue) -> SpeechRecognitionResultList {
            SpeechRecognitionResultList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SpeechRecognitionResultList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SpeechRecognitionResultList> for SpeechRecognitionResultList {
        #[inline]
        fn as_ref(&self) -> &SpeechRecognitionResultList {
            self
        }
    }
    impl From<SpeechRecognitionResultList> for JsValue {
        #[inline]
        fn from(obj: SpeechRecognitionResultList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SpeechRecognitionResultList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SpeechRecognitionResultList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SpeechRecognitionResultList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SpeechRecognitionResultList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SpeechRecognitionResultList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SpeechRecognitionResultList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SpeechRecognitionResultList> for ::js_sys::Object {
    #[inline]
    fn from(obj: SpeechRecognitionResultList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SpeechRecognitionResultList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "SpeechRecognitionResult",
    feature = "SpeechRecognitionResultList",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_SpeechRecognitionResultList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognitionResultList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SpeechRecognitionResult as WasmDescribe>::describe();
}
impl SpeechRecognitionResultList {
    #[cfg(all(
        feature = "SpeechRecognitionResult",
        feature = "SpeechRecognitionResultList",
    ))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/item)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResult`, `SpeechRecognitionResultList`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> SpeechRecognitionResult {
        #[cfg(all(
            feature = "SpeechRecognitionResult",
            feature = "SpeechRecognitionResultList",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_SpeechRecognitionResultList(
                self_: <&SpeechRecognitionResultList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SpeechRecognitionResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_SpeechRecognitionResultList(
            self_: <&SpeechRecognitionResultList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SpeechRecognitionResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&SpeechRecognitionResultList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_SpeechRecognitionResultList(self_, index)
            };
            <SpeechRecognitionResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SpeechRecognitionResult",
    feature = "SpeechRecognitionResultList",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_SpeechRecognitionResultList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SpeechRecognitionResultList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<SpeechRecognitionResult> as WasmDescribe>::describe();
}
impl SpeechRecognitionResultList {
    #[cfg(all(
        feature = "SpeechRecognitionResult",
        feature = "SpeechRecognitionResultList",
    ))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResult`, `SpeechRecognitionResultList`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Option<SpeechRecognitionResult> {
        #[cfg(all(
            feature = "SpeechRecognitionResult",
            feature = "SpeechRecognitionResultList",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_SpeechRecognitionResultList(
                self_: <&SpeechRecognitionResultList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SpeechRecognitionResult> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_SpeechRecognitionResultList(
            self_: <&SpeechRecognitionResultList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SpeechRecognitionResult> as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&SpeechRecognitionResultList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_SpeechRecognitionResultList(self_, index)
            };
            <Option<SpeechRecognitionResult> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechRecognitionResultList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_SpeechRecognitionResultList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechRecognitionResultList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl SpeechRecognitionResultList {
    #[cfg(all(feature = "SpeechRecognitionResultList",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/length)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResultList`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "SpeechRecognitionResultList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_SpeechRecognitionResultList(
                self_: <&SpeechRecognitionResultList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_SpeechRecognitionResultList(
            self_: <&SpeechRecognitionResultList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechRecognitionResultList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_length_SpeechRecognitionResultList(self_)
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
pub static __WASM_BINDGEN_GENERATED_eac2d907b3e1dbf5: [u8; 483usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA1\x01\0\0\0\0\x04\0\0\x02\x1BSpeechRecognitionResultList-__widl_instanceof_SpeechRecognitionResultList\0\0\0\0)__widl_f_item_SpeechRecognitionResultList\0\0\0\x01\x1BSpeechRecognitionResultList\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0(__widl_f_get_SpeechRecognitionResultList\0\0\0\x01\x1BSpeechRecognitionResultList\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0+__widl_f_length_SpeechRecognitionResultList\0\0\0\x01\x1BSpeechRecognitionResultList\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
