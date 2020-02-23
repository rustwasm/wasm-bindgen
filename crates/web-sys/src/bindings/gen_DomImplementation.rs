use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMImplementation` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation)\n\n*This API requires the following crate features to be activated: `DomImplementation`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomImplementation {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomImplementation: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomImplementation {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(73u32);
            inform(109u32);
            inform(112u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for DomImplementation {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomImplementation {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomImplementation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomImplementation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomImplementation {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomImplementation {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomImplementation {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomImplementation {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomImplementation {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomImplementation>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomImplementation {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomImplementation {
        #[inline]
        fn from(obj: JsValue) -> DomImplementation {
            DomImplementation { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomImplementation {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomImplementation> for DomImplementation {
        #[inline]
        fn as_ref(&self) -> &DomImplementation {
            self
        }
    }
    impl From<DomImplementation> for JsValue {
        #[inline]
        fn from(obj: DomImplementation) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomImplementation {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMImplementation(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMImplementation(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMImplementation(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomImplementation { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomImplementation) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomImplementation> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomImplementation) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomImplementation {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Document", feature = "DomImplementation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_document_DOMImplementation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomImplementation as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Document as WasmDescribe>::describe();
}
impl DomImplementation {
    #[cfg(all(feature = "Document", feature = "DomImplementation",))]
    #[allow(bad_style)]
    #[doc = "The `createDocument()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocument)\n\n*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
    #[allow(clippy::all)]
    pub fn create_document(
        &self,
        namespace: Option<&str>,
        qualified_name: &str,
    ) -> Result<Document, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomImplementation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_document_DOMImplementation(
                self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_document_DOMImplementation(
            self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(namespace);
            drop(qualified_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let qualified_name =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(qualified_name);
                __widl_f_create_document_DOMImplementation(self_, namespace, qualified_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Document as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "DocumentType",
    feature = "DomImplementation",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_document_with_doctype_DOMImplementation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomImplementation as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&DocumentType> as WasmDescribe>::describe();
    <Document as WasmDescribe>::describe();
}
impl DomImplementation {
    #[cfg(all(
        feature = "Document",
        feature = "DocumentType",
        feature = "DomImplementation",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createDocument()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocument)\n\n*This API requires the following crate features to be activated: `Document`, `DocumentType`, `DomImplementation`*"]
    #[allow(clippy::all)]
    pub fn create_document_with_doctype(
        &self,
        namespace: Option<&str>,
        qualified_name: &str,
        doctype: Option<&DocumentType>,
    ) -> Result<Document, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "DocumentType",
            feature = "DomImplementation",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_document_with_doctype_DOMImplementation(
                self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                doctype: <Option<&DocumentType> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_document_with_doctype_DOMImplementation(
            self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            doctype: <Option<&DocumentType> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(namespace);
            drop(qualified_name);
            drop(doctype);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let qualified_name =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(qualified_name);
                let doctype =
                    <Option<&DocumentType> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        doctype,
                    );
                __widl_f_create_document_with_doctype_DOMImplementation(
                    self_,
                    namespace,
                    qualified_name,
                    doctype,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Document as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "DomImplementation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_document_type_DOMImplementation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomImplementation as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <DocumentType as WasmDescribe>::describe();
}
impl DomImplementation {
    #[cfg(all(feature = "DocumentType", feature = "DomImplementation",))]
    #[allow(bad_style)]
    #[doc = "The `createDocumentType()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocumentType)\n\n*This API requires the following crate features to be activated: `DocumentType`, `DomImplementation`*"]
    #[allow(clippy::all)]
    pub fn create_document_type(
        &self,
        qualified_name: &str,
        public_id: &str,
        system_id: &str,
    ) -> Result<DocumentType, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "DomImplementation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_document_type_DOMImplementation(
                self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                public_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                system_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DocumentType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_document_type_DOMImplementation(
            self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            public_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            system_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DocumentType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(qualified_name);
            drop(public_id);
            drop(system_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let qualified_name =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(qualified_name);
                let public_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(public_id);
                let system_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(system_id);
                __widl_f_create_document_type_DOMImplementation(
                    self_,
                    qualified_name,
                    public_id,
                    system_id,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DocumentType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "DomImplementation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_html_document_DOMImplementation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomImplementation as WasmDescribe>::describe();
    <Document as WasmDescribe>::describe();
}
impl DomImplementation {
    #[cfg(all(feature = "Document", feature = "DomImplementation",))]
    #[allow(bad_style)]
    #[doc = "The `createHTMLDocument()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createHTMLDocument)\n\n*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
    #[allow(clippy::all)]
    pub fn create_html_document(&self) -> Result<Document, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomImplementation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_html_document_DOMImplementation(
                self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_html_document_DOMImplementation(
            self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_html_document_DOMImplementation(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Document as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "DomImplementation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_html_document_with_title_DOMImplementation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomImplementation as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Document as WasmDescribe>::describe();
}
impl DomImplementation {
    #[cfg(all(feature = "Document", feature = "DomImplementation",))]
    #[allow(bad_style)]
    #[doc = "The `createHTMLDocument()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createHTMLDocument)\n\n*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
    #[allow(clippy::all)]
    pub fn create_html_document_with_title(
        &self,
        title: &str,
    ) -> Result<Document, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomImplementation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_html_document_with_title_DOMImplementation(
                self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_html_document_with_title_DOMImplementation(
            self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(title);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                __widl_f_create_html_document_with_title_DOMImplementation(self_, title)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Document as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomImplementation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_feature_DOMImplementation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomImplementation as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl DomImplementation {
    #[cfg(all(feature = "DomImplementation",))]
    #[allow(bad_style)]
    #[doc = "The `hasFeature()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/hasFeature)\n\n*This API requires the following crate features to be activated: `DomImplementation`*"]
    #[allow(clippy::all)]
    pub fn has_feature(&self) -> bool {
        #[cfg(all(feature = "DomImplementation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_feature_DOMImplementation(
                self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_feature_DOMImplementation(
            self_: <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomImplementation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_has_feature_DOMImplementation(self_)
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
pub static __WASM_BINDGEN_GENERATED_a5cd666aaefb445e: [u8; 869usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}#\x03\0\0\0\0\x07\0\0\x02\x11DOMImplementation#__widl_instanceof_DOMImplementation\0\0\0\0*__widl_f_create_document_DOMImplementation\x01\0\0\x01\x11DOMImplementation\x01\0\0\x01\x03\x05self_\tnamespace\x0Equalified_name\x0EcreateDocument\0\0\07__widl_f_create_document_with_doctype_DOMImplementation\x01\0\0\x01\x11DOMImplementation\x01\0\0\x01\x04\x05self_\tnamespace\x0Equalified_name\x07doctype\x0EcreateDocument\0\0\0/__widl_f_create_document_type_DOMImplementation\x01\0\0\x01\x11DOMImplementation\x01\0\0\x01\x04\x05self_\x0Equalified_name\tpublic_id\tsystem_id\x12createDocumentType\0\0\0/__widl_f_create_html_document_DOMImplementation\x01\0\0\x01\x11DOMImplementation\x01\0\0\x01\x01\x05self_\x12createHTMLDocument\0\0\0:__widl_f_create_html_document_with_title_DOMImplementation\x01\0\0\x01\x11DOMImplementation\x01\0\0\x01\x02\x05self_\x05title\x12createHTMLDocument\0\0\0&__widl_f_has_feature_DOMImplementation\0\0\0\x01\x11DOMImplementation\x01\0\0\x01\x01\x05self_\nhasFeature\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
