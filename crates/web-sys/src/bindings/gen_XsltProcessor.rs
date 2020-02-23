use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `XSLTProcessor` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor)\n\n*This API requires the following crate features to be activated: `XsltProcessor`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct XsltProcessor {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_XsltProcessor: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for XsltProcessor {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(88u32);
            inform(83u32);
            inform(76u32);
            inform(84u32);
            inform(80u32);
            inform(114u32);
            inform(111u32);
            inform(99u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(111u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for XsltProcessor {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for XsltProcessor {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for XsltProcessor {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a XsltProcessor {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for XsltProcessor {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            XsltProcessor {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for XsltProcessor {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a XsltProcessor {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for XsltProcessor {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<XsltProcessor>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(XsltProcessor {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for XsltProcessor {
        #[inline]
        fn from(obj: JsValue) -> XsltProcessor {
            XsltProcessor { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for XsltProcessor {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<XsltProcessor> for XsltProcessor {
        #[inline]
        fn as_ref(&self) -> &XsltProcessor {
            self
        }
    }
    impl From<XsltProcessor> for JsValue {
        #[inline]
        fn from(obj: XsltProcessor) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for XsltProcessor {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_XSLTProcessor(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_XSLTProcessor(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_XSLTProcessor(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            XsltProcessor { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const XsltProcessor) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<XsltProcessor> for ::js_sys::Object {
    #[inline]
    fn from(obj: XsltProcessor) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for XsltProcessor {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "XsltProcessor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_XSLTProcessor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <XsltProcessor as WasmDescribe>::describe();
}
impl XsltProcessor {
    #[cfg(all(feature = "XsltProcessor",))]
    #[allow(bad_style)]
    #[doc = "The `new XSLTProcessor(..)` constructor, creating a new instance of `XSLTProcessor`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/XSLTProcessor)\n\n*This API requires the following crate features to be activated: `XsltProcessor`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<XsltProcessor, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XsltProcessor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_XSLTProcessor(
            ) -> <XsltProcessor as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_XSLTProcessor(
        ) -> <XsltProcessor as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_XSLTProcessor() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XsltProcessor as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "XsltProcessor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_parameters_XSLTProcessor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XsltProcessor as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XsltProcessor {
    #[cfg(all(feature = "XsltProcessor",))]
    #[allow(bad_style)]
    #[doc = "The `clearParameters()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/clearParameters)\n\n*This API requires the following crate features to be activated: `XsltProcessor`*"]
    #[allow(clippy::all)]
    pub fn clear_parameters(&self) {
        #[cfg(all(feature = "XsltProcessor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_parameters_XSLTProcessor(
                self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_parameters_XSLTProcessor(
            self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_parameters_XSLTProcessor(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Node", feature = "XsltProcessor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_stylesheet_XSLTProcessor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XsltProcessor as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XsltProcessor {
    #[cfg(all(feature = "Node", feature = "XsltProcessor",))]
    #[allow(bad_style)]
    #[doc = "The `importStylesheet()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/importStylesheet)\n\n*This API requires the following crate features to be activated: `Node`, `XsltProcessor`*"]
    #[allow(clippy::all)]
    pub fn import_stylesheet(&self, style: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "XsltProcessor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_stylesheet_XSLTProcessor(
                self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                style: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_stylesheet_XSLTProcessor(
            self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            style: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(style);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let style = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(style);
                __widl_f_import_stylesheet_XSLTProcessor(self_, style)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XsltProcessor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_parameter_XSLTProcessor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&XsltProcessor as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XsltProcessor {
    #[cfg(all(feature = "XsltProcessor",))]
    #[allow(bad_style)]
    #[doc = "The `removeParameter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/removeParameter)\n\n*This API requires the following crate features to be activated: `XsltProcessor`*"]
    #[allow(clippy::all)]
    pub fn remove_parameter(
        &self,
        namespace_uri: &str,
        local_name: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XsltProcessor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_parameter_XSLTProcessor(
                self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace_uri: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_parameter_XSLTProcessor(
            self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace_uri: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(namespace_uri);
            drop(local_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace_uri =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace_uri);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_remove_parameter_XSLTProcessor(self_, namespace_uri, local_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "XsltProcessor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reset_XSLTProcessor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XsltProcessor as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XsltProcessor {
    #[cfg(all(feature = "XsltProcessor",))]
    #[allow(bad_style)]
    #[doc = "The `reset()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/reset)\n\n*This API requires the following crate features to be activated: `XsltProcessor`*"]
    #[allow(clippy::all)]
    pub fn reset(&self) {
        #[cfg(all(feature = "XsltProcessor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reset_XSLTProcessor(
                self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reset_XSLTProcessor(
            self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_reset_XSLTProcessor(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "XsltProcessor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_parameter_XSLTProcessor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&XsltProcessor as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XsltProcessor {
    #[cfg(all(feature = "XsltProcessor",))]
    #[allow(bad_style)]
    #[doc = "The `setParameter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/setParameter)\n\n*This API requires the following crate features to be activated: `XsltProcessor`*"]
    #[allow(clippy::all)]
    pub fn set_parameter(
        &self,
        namespace_uri: &str,
        local_name: &str,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XsltProcessor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_parameter_XSLTProcessor(
                self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace_uri: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_parameter_XSLTProcessor(
            self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace_uri: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(namespace_uri);
            drop(local_name);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace_uri =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace_uri);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                let value =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        value,
                    );
                __widl_f_set_parameter_XSLTProcessor(self_, namespace_uri, local_name, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node", feature = "XsltProcessor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transform_to_document_XSLTProcessor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XsltProcessor as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Document as WasmDescribe>::describe();
}
impl XsltProcessor {
    #[cfg(all(feature = "Document", feature = "Node", feature = "XsltProcessor",))]
    #[allow(bad_style)]
    #[doc = "The `transformToDocument()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/transformToDocument)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `XsltProcessor`*"]
    #[allow(clippy::all)]
    pub fn transform_to_document(
        &self,
        source: &Node,
    ) -> Result<Document, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node", feature = "XsltProcessor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transform_to_document_XSLTProcessor(
                self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transform_to_document_XSLTProcessor(
            self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(source);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let source = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                __widl_f_transform_to_document_XSLTProcessor(self_, source)
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
    feature = "DocumentFragment",
    feature = "Node",
    feature = "XsltProcessor",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transform_to_fragment_XSLTProcessor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&XsltProcessor as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <DocumentFragment as WasmDescribe>::describe();
}
impl XsltProcessor {
    #[cfg(all(
        feature = "Document",
        feature = "DocumentFragment",
        feature = "Node",
        feature = "XsltProcessor",
    ))]
    #[allow(bad_style)]
    #[doc = "The `transformToFragment()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/transformToFragment)\n\n*This API requires the following crate features to be activated: `Document`, `DocumentFragment`, `Node`, `XsltProcessor`*"]
    #[allow(clippy::all)]
    pub fn transform_to_fragment(
        &self,
        source: &Node,
        output: &Document,
    ) -> Result<DocumentFragment, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "DocumentFragment",
            feature = "Node",
            feature = "XsltProcessor",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transform_to_fragment_XSLTProcessor(
                self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                output: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transform_to_fragment_XSLTProcessor(
            self_: <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            output: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(source);
            drop(output);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XsltProcessor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let source = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                let output = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(output);
                __widl_f_transform_to_fragment_XSLTProcessor(self_, source, output)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ff7e88f62cd100ca: [u8; 902usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}D\x03\0\0\0\0\t\0\0\x02\rXSLTProcessor\x1F__widl_instanceof_XSLTProcessor\0\0\0\0\x1A__widl_f_new_XSLTProcessor\x01\0\0\x01\rXSLTProcessor\0\x01\0\x03new\0\0\0'__widl_f_clear_parameters_XSLTProcessor\0\0\0\x01\rXSLTProcessor\x01\0\0\x01\x01\x05self_\x0FclearParameters\0\0\0(__widl_f_import_stylesheet_XSLTProcessor\x01\0\0\x01\rXSLTProcessor\x01\0\0\x01\x02\x05self_\x05style\x10importStylesheet\0\0\0'__widl_f_remove_parameter_XSLTProcessor\x01\0\0\x01\rXSLTProcessor\x01\0\0\x01\x03\x05self_\rnamespace_uri\nlocal_name\x0FremoveParameter\0\0\0\x1C__widl_f_reset_XSLTProcessor\0\0\0\x01\rXSLTProcessor\x01\0\0\x01\x01\x05self_\x05reset\0\0\0$__widl_f_set_parameter_XSLTProcessor\x01\0\0\x01\rXSLTProcessor\x01\0\0\x01\x04\x05self_\rnamespace_uri\nlocal_name\x05value\x0CsetParameter\0\0\0,__widl_f_transform_to_document_XSLTProcessor\x01\0\0\x01\rXSLTProcessor\x01\0\0\x01\x02\x05self_\x06source\x13transformToDocument\0\0\0,__widl_f_transform_to_fragment_XSLTProcessor\x01\0\0\x01\rXSLTProcessor\x01\0\0\x01\x03\x05self_\x06source\x06output\x13transformToFragment\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
