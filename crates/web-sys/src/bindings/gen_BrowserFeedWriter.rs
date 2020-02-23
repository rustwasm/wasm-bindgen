use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `BrowserFeedWriter` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter)\n\n*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct BrowserFeedWriter {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_BrowserFeedWriter: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for BrowserFeedWriter {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(66u32);
            inform(114u32);
            inform(111u32);
            inform(119u32);
            inform(115u32);
            inform(101u32);
            inform(114u32);
            inform(70u32);
            inform(101u32);
            inform(101u32);
            inform(100u32);
            inform(87u32);
            inform(114u32);
            inform(105u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for BrowserFeedWriter {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for BrowserFeedWriter {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for BrowserFeedWriter {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a BrowserFeedWriter {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for BrowserFeedWriter {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            BrowserFeedWriter {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for BrowserFeedWriter {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a BrowserFeedWriter {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for BrowserFeedWriter {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<BrowserFeedWriter>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(BrowserFeedWriter {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for BrowserFeedWriter {
        #[inline]
        fn from(obj: JsValue) -> BrowserFeedWriter {
            BrowserFeedWriter { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for BrowserFeedWriter {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<BrowserFeedWriter> for BrowserFeedWriter {
        #[inline]
        fn as_ref(&self) -> &BrowserFeedWriter {
            self
        }
    }
    impl From<BrowserFeedWriter> for JsValue {
        #[inline]
        fn from(obj: BrowserFeedWriter) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for BrowserFeedWriter {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_BrowserFeedWriter(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_BrowserFeedWriter(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_BrowserFeedWriter(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            BrowserFeedWriter { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const BrowserFeedWriter) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<BrowserFeedWriter> for ::js_sys::Object {
    #[inline]
    fn from(obj: BrowserFeedWriter) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for BrowserFeedWriter {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BrowserFeedWriter",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_BrowserFeedWriter() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <BrowserFeedWriter as WasmDescribe>::describe();
}
impl BrowserFeedWriter {
    #[cfg(all(feature = "BrowserFeedWriter",))]
    #[allow(bad_style)]
    #[doc = "The `new BrowserFeedWriter(..)` constructor, creating a new instance of `BrowserFeedWriter`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/BrowserFeedWriter)\n\n*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<BrowserFeedWriter, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BrowserFeedWriter",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_BrowserFeedWriter(
            ) -> <BrowserFeedWriter as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_BrowserFeedWriter(
        ) -> <BrowserFeedWriter as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_BrowserFeedWriter() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<BrowserFeedWriter as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "BrowserFeedWriter",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_BrowserFeedWriter() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BrowserFeedWriter as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BrowserFeedWriter {
    #[cfg(all(feature = "BrowserFeedWriter",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/close)\n\n*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "BrowserFeedWriter",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_BrowserFeedWriter(
                self_: <&BrowserFeedWriter as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_BrowserFeedWriter(
            self_: <&BrowserFeedWriter as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&BrowserFeedWriter as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_BrowserFeedWriter(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "BrowserFeedWriter",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_content_BrowserFeedWriter() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BrowserFeedWriter as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BrowserFeedWriter {
    #[cfg(all(feature = "BrowserFeedWriter",))]
    #[allow(bad_style)]
    #[doc = "The `writeContent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/writeContent)\n\n*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
    #[allow(clippy::all)]
    pub fn write_content(&self) {
        #[cfg(all(feature = "BrowserFeedWriter",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_content_BrowserFeedWriter(
                self_: <&BrowserFeedWriter as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_content_BrowserFeedWriter(
            self_: <&BrowserFeedWriter as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&BrowserFeedWriter as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_write_content_BrowserFeedWriter(self_)
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
pub static __WASM_BINDGEN_GENERATED_3e284cc2b42c8a71: [u8; 391usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}E\x01\0\0\0\0\x04\0\0\x02\x11BrowserFeedWriter#__widl_instanceof_BrowserFeedWriter\0\0\0\0\x1E__widl_f_new_BrowserFeedWriter\x01\0\0\x01\x11BrowserFeedWriter\0\x01\0\x03new\0\0\0 __widl_f_close_BrowserFeedWriter\0\0\0\x01\x11BrowserFeedWriter\x01\0\0\x01\x01\x05self_\x05close\0\0\0(__widl_f_write_content_BrowserFeedWriter\0\0\0\x01\x11BrowserFeedWriter\x01\0\0\x01\x01\x05self_\x0CwriteContent\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
