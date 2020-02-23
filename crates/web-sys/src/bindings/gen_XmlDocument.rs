use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `XMLDocument` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument)\n\n*This API requires the following crate features to be activated: `XmlDocument`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct XmlDocument {
    obj: Document,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_XmlDocument: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for XmlDocument {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(88u32);
            inform(77u32);
            inform(76u32);
            inform(68u32);
            inform(111u32);
            inform(99u32);
            inform(117u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for XmlDocument {
        type Target = Document;
        #[inline]
        fn deref(&self) -> &Document {
            &self.obj
        }
    }
    impl IntoWasmAbi for XmlDocument {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for XmlDocument {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a XmlDocument {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for XmlDocument {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            XmlDocument {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for XmlDocument {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a XmlDocument {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for XmlDocument {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<XmlDocument>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(XmlDocument {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for XmlDocument {
        #[inline]
        fn from(obj: JsValue) -> XmlDocument {
            XmlDocument { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for XmlDocument {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<XmlDocument> for XmlDocument {
        #[inline]
        fn as_ref(&self) -> &XmlDocument {
            self
        }
    }
    impl From<XmlDocument> for JsValue {
        #[inline]
        fn from(obj: XmlDocument) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for XmlDocument {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_XMLDocument(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_XMLDocument(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_XMLDocument(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            XmlDocument { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const XmlDocument) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<XmlDocument> for Document {
    #[inline]
    fn from(obj: XmlDocument) -> Document {
        use wasm_bindgen::JsCast;
        Document::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Document> for XmlDocument {
    #[inline]
    fn as_ref(&self) -> &Document {
        use wasm_bindgen::JsCast;
        Document::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<XmlDocument> for Node {
    #[inline]
    fn from(obj: XmlDocument) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for XmlDocument {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<XmlDocument> for EventTarget {
    #[inline]
    fn from(obj: XmlDocument) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for XmlDocument {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<XmlDocument> for ::js_sys::Object {
    #[inline]
    fn from(obj: XmlDocument) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for XmlDocument {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "XmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_XMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlDocument as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl XmlDocument {
    #[cfg(all(feature = "XmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `load()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/load)\n\n*This API requires the following crate features to be activated: `XmlDocument`*"]
    #[allow(clippy::all)]
    pub fn load(&self, url: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_XMLDocument(
                self_: <&XmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_XMLDocument(
            self_: <&XmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_load_XMLDocument(self_, url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "XmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_async_XMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlDocument as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl XmlDocument {
    #[cfg(all(feature = "XmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `async` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/async)\n\n*This API requires the following crate features to be activated: `XmlDocument`*"]
    #[allow(clippy::all)]
    pub fn r#async(&self) -> bool {
        #[cfg(all(feature = "XmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_async_XMLDocument(
                self_: <&XmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_async_XMLDocument(
            self_: <&XmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_async_XMLDocument(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlDocument",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_async_XMLDocument() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlDocument as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlDocument {
    #[cfg(all(feature = "XmlDocument",))]
    #[allow(bad_style)]
    #[doc = "The `async` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/async)\n\n*This API requires the following crate features to be activated: `XmlDocument`*"]
    #[allow(clippy::all)]
    pub fn set_async(&self, r#async: bool) {
        #[cfg(all(feature = "XmlDocument",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_async_XMLDocument(
                self_: <&XmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                r#async: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_async_XMLDocument(
            self_: <&XmlDocument as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            r#async: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(r#async);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XmlDocument as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let r#async = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r#async);
                __widl_f_set_async_XMLDocument(self_, r#async)
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
pub static __WASM_BINDGEN_GENERATED_ab0595dde26d556d: [u8; 366usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"},\x01\0\0\0\0\x04\0\0\x02\x0BXMLDocument\x1D__widl_instanceof_XMLDocument\0\0\0\0\x19__widl_f_load_XMLDocument\x01\0\0\x01\x0BXMLDocument\x01\0\0\x01\x02\x05self_\x03url\x04load\0\0\0\x1A__widl_f_async_XMLDocument\0\0\0\x01\x0BXMLDocument\x01\0\x01\x05async\x01\x01\x05self_\x05async\0\0\0\x1E__widl_f_set_async_XMLDocument\0\0\0\x01\x0BXMLDocument\x01\0\x02\x05async\x01\x02\x05self_\x07r#async\x05async\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
