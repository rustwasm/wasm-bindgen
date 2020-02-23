use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DocumentTimeline` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline)\n\n*This API requires the following crate features to be activated: `DocumentTimeline`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DocumentTimeline {
    obj: AnimationTimeline,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DocumentTimeline: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DocumentTimeline {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(68u32);
            inform(111u32);
            inform(99u32);
            inform(117u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(84u32);
            inform(105u32);
            inform(109u32);
            inform(101u32);
            inform(108u32);
            inform(105u32);
            inform(110u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for DocumentTimeline {
        type Target = AnimationTimeline;
        #[inline]
        fn deref(&self) -> &AnimationTimeline {
            &self.obj
        }
    }
    impl IntoWasmAbi for DocumentTimeline {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DocumentTimeline {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DocumentTimeline {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DocumentTimeline {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DocumentTimeline {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DocumentTimeline {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DocumentTimeline {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DocumentTimeline {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DocumentTimeline>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DocumentTimeline {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DocumentTimeline {
        #[inline]
        fn from(obj: JsValue) -> DocumentTimeline {
            DocumentTimeline { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DocumentTimeline {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DocumentTimeline> for DocumentTimeline {
        #[inline]
        fn as_ref(&self) -> &DocumentTimeline {
            self
        }
    }
    impl From<DocumentTimeline> for JsValue {
        #[inline]
        fn from(obj: DocumentTimeline) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DocumentTimeline {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DocumentTimeline(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DocumentTimeline(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DocumentTimeline(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DocumentTimeline { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DocumentTimeline) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DocumentTimeline> for AnimationTimeline {
    #[inline]
    fn from(obj: DocumentTimeline) -> AnimationTimeline {
        use wasm_bindgen::JsCast;
        AnimationTimeline::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AnimationTimeline> for DocumentTimeline {
    #[inline]
    fn as_ref(&self) -> &AnimationTimeline {
        use wasm_bindgen::JsCast;
        AnimationTimeline::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DocumentTimeline> for ::js_sys::Object {
    #[inline]
    fn from(obj: DocumentTimeline) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DocumentTimeline {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DocumentTimeline",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DocumentTimeline() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DocumentTimeline as WasmDescribe>::describe();
}
impl DocumentTimeline {
    #[cfg(all(feature = "DocumentTimeline",))]
    #[allow(bad_style)]
    #[doc = "The `new DocumentTimeline(..)` constructor, creating a new instance of `DocumentTimeline`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline/DocumentTimeline)\n\n*This API requires the following crate features to be activated: `DocumentTimeline`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DocumentTimeline, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentTimeline",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DocumentTimeline(
            ) -> <DocumentTimeline as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DocumentTimeline(
        ) -> <DocumentTimeline as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DocumentTimeline() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DocumentTimeline as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DocumentTimeline", feature = "DocumentTimelineOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_DocumentTimeline() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentTimelineOptions as WasmDescribe>::describe();
    <DocumentTimeline as WasmDescribe>::describe();
}
impl DocumentTimeline {
    #[cfg(all(feature = "DocumentTimeline", feature = "DocumentTimelineOptions",))]
    #[allow(bad_style)]
    #[doc = "The `new DocumentTimeline(..)` constructor, creating a new instance of `DocumentTimeline`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline/DocumentTimeline)\n\n*This API requires the following crate features to be activated: `DocumentTimeline`, `DocumentTimelineOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        options: &DocumentTimelineOptions,
    ) -> Result<DocumentTimeline, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentTimeline", feature = "DocumentTimelineOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_DocumentTimeline(
                options: <&DocumentTimelineOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DocumentTimeline as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_DocumentTimeline(
            options: <&DocumentTimelineOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DocumentTimeline as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let options =
                    <&DocumentTimelineOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_new_with_options_DocumentTimeline(options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DocumentTimeline as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_c270b014501d72c9: [u8; 304usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xEE\0\0\0\0\0\x03\0\0\x02\x10DocumentTimeline\"__widl_instanceof_DocumentTimeline\0\0\0\0\x1D__widl_f_new_DocumentTimeline\x01\0\0\x01\x10DocumentTimeline\0\x01\0\x03new\0\0\0*__widl_f_new_with_options_DocumentTimeline\x01\0\0\x01\x10DocumentTimeline\0\x01\x01\x07options\x03new\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
