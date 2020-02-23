use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `XMLSerializer` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer)\n\n*This API requires the following crate features to be activated: `XmlSerializer`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct XmlSerializer {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_XmlSerializer: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for XmlSerializer {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(88u32);
            inform(77u32);
            inform(76u32);
            inform(83u32);
            inform(101u32);
            inform(114u32);
            inform(105u32);
            inform(97u32);
            inform(108u32);
            inform(105u32);
            inform(122u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for XmlSerializer {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for XmlSerializer {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for XmlSerializer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a XmlSerializer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for XmlSerializer {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            XmlSerializer {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for XmlSerializer {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a XmlSerializer {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for XmlSerializer {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<XmlSerializer>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(XmlSerializer {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for XmlSerializer {
        #[inline]
        fn from(obj: JsValue) -> XmlSerializer {
            XmlSerializer { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for XmlSerializer {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<XmlSerializer> for XmlSerializer {
        #[inline]
        fn as_ref(&self) -> &XmlSerializer {
            self
        }
    }
    impl From<XmlSerializer> for JsValue {
        #[inline]
        fn from(obj: XmlSerializer) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for XmlSerializer {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_XMLSerializer(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_XMLSerializer(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_XMLSerializer(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            XmlSerializer { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const XmlSerializer) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<XmlSerializer> for ::js_sys::Object {
    #[inline]
    fn from(obj: XmlSerializer) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for XmlSerializer {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "XmlSerializer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_XMLSerializer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <XmlSerializer as WasmDescribe>::describe();
}
impl XmlSerializer {
    #[cfg(all(feature = "XmlSerializer",))]
    #[allow(bad_style)]
    #[doc = "The `new XMLSerializer(..)` constructor, creating a new instance of `XMLSerializer`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/XMLSerializer)\n\n*This API requires the following crate features to be activated: `XmlSerializer`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<XmlSerializer, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlSerializer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_XMLSerializer(
            ) -> <XmlSerializer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_XMLSerializer(
        ) -> <XmlSerializer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_XMLSerializer() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XmlSerializer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "XmlSerializer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_serialize_to_string_XMLSerializer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlSerializer as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl XmlSerializer {
    #[cfg(all(feature = "Node", feature = "XmlSerializer",))]
    #[allow(bad_style)]
    #[doc = "The `serializeToString()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/serializeToString)\n\n*This API requires the following crate features to be activated: `Node`, `XmlSerializer`*"]
    #[allow(clippy::all)]
    pub fn serialize_to_string(&self, root: &Node) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "XmlSerializer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_serialize_to_string_XMLSerializer(
                self_: <&XmlSerializer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_serialize_to_string_XMLSerializer(
            self_: <&XmlSerializer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(root);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlSerializer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let root = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(root);
                __widl_f_serialize_to_string_XMLSerializer(self_, root)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
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
pub static __WASM_BINDGEN_GENERATED_2df4b9aedccd4096: [u8; 308usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xF2\0\0\0\0\0\x03\0\0\x02\rXMLSerializer\x1F__widl_instanceof_XMLSerializer\0\0\0\0\x1A__widl_f_new_XMLSerializer\x01\0\0\x01\rXMLSerializer\0\x01\0\x03new\0\0\0*__widl_f_serialize_to_string_XMLSerializer\x01\0\0\x01\rXMLSerializer\x01\0\0\x01\x02\x05self_\x04root\x11serializeToString\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
