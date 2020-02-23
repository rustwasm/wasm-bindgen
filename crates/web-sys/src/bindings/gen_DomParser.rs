use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMParser` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser)\n\n*This API requires the following crate features to be activated: `DomParser`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomParser {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomParser: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomParser {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(80u32);
            inform(97u32);
            inform(114u32);
            inform(115u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for DomParser {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomParser {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomParser {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomParser {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomParser {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomParser {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomParser {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomParser {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomParser {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomParser>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomParser {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomParser {
        #[inline]
        fn from(obj: JsValue) -> DomParser {
            DomParser { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomParser {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomParser> for DomParser {
        #[inline]
        fn as_ref(&self) -> &DomParser {
            self
        }
    }
    impl From<DomParser> for JsValue {
        #[inline]
        fn from(obj: DomParser) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomParser {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMParser(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMParser(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMParser(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomParser { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomParser) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomParser> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomParser) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomParser {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomParser",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DOMParser() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DomParser as WasmDescribe>::describe();
}
impl DomParser {
    #[cfg(all(feature = "DomParser",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMParser(..)` constructor, creating a new instance of `DOMParser`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser/DOMParser)\n\n*This API requires the following crate features to be activated: `DomParser`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DomParser, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomParser",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DOMParser() -> <DomParser as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DOMParser() -> <DomParser as wasm_bindgen::convert::FromWasmAbi>::Abi
        {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DOMParser() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomParser as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "DomParser", feature = "SupportedType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_parse_from_string_DOMParser() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomParser as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <SupportedType as WasmDescribe>::describe();
    <Document as WasmDescribe>::describe();
}
impl DomParser {
    #[cfg(all(feature = "Document", feature = "DomParser", feature = "SupportedType",))]
    #[allow(bad_style)]
    #[doc = "The `parseFromString()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser/parseFromString)\n\n*This API requires the following crate features to be activated: `Document`, `DomParser`, `SupportedType`*"]
    #[allow(clippy::all)]
    pub fn parse_from_string(
        &self,
        str: &str,
        type_: SupportedType,
    ) -> Result<Document, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomParser", feature = "SupportedType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_parse_from_string_DOMParser(
                self_: <&DomParser as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                str: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <SupportedType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_parse_from_string_DOMParser(
            self_: <&DomParser as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            str: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <SupportedType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(str);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomParser as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let str = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(str);
                let type_ = <SupportedType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_parse_from_string_DOMParser(self_, str, type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Document as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_fd720b08f8bffb79: [u8; 285usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDB\0\0\0\0\0\x03\0\0\x02\tDOMParser\x1B__widl_instanceof_DOMParser\0\0\0\0\x16__widl_f_new_DOMParser\x01\0\0\x01\tDOMParser\0\x01\0\x03new\0\0\0$__widl_f_parse_from_string_DOMParser\x01\0\0\x01\tDOMParser\x01\0\0\x01\x03\x05self_\x03str\x05type_\x0FparseFromString\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
