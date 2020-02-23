use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `XPathExpression` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression)\n\n*This API requires the following crate features to be activated: `XPathExpression`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct XPathExpression {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_XPathExpression: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for XPathExpression {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(88u32);
            inform(80u32);
            inform(97u32);
            inform(116u32);
            inform(104u32);
            inform(69u32);
            inform(120u32);
            inform(112u32);
            inform(114u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for XPathExpression {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for XPathExpression {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for XPathExpression {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a XPathExpression {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for XPathExpression {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            XPathExpression {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for XPathExpression {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a XPathExpression {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for XPathExpression {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<XPathExpression>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(XPathExpression {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for XPathExpression {
        #[inline]
        fn from(obj: JsValue) -> XPathExpression {
            XPathExpression { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for XPathExpression {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<XPathExpression> for XPathExpression {
        #[inline]
        fn as_ref(&self) -> &XPathExpression {
            self
        }
    }
    impl From<XPathExpression> for JsValue {
        #[inline]
        fn from(obj: XPathExpression) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for XPathExpression {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_XPathExpression(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_XPathExpression(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_XPathExpression(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            XPathExpression { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const XPathExpression) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<XPathExpression> for ::js_sys::Object {
    #[inline]
    fn from(obj: XPathExpression) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for XPathExpression {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Node", feature = "XPathExpression", feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_evaluate_XPathExpression() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XPathExpression as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <XPathResult as WasmDescribe>::describe();
}
impl XPathExpression {
    #[cfg(all(feature = "Node", feature = "XPathExpression", feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `evaluate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)\n\n*This API requires the following crate features to be activated: `Node`, `XPathExpression`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn evaluate(&self, context_node: &Node) -> Result<XPathResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "XPathExpression", feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_evaluate_XPathExpression(
                self_: <&XPathExpression as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_evaluate_XPathExpression(
            self_: <&XPathExpression as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(context_node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&XPathExpression as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let context_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_node);
                __widl_f_evaluate_XPathExpression(self_, context_node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "XPathExpression", feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_evaluate_with_type_XPathExpression() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&XPathExpression as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <XPathResult as WasmDescribe>::describe();
}
impl XPathExpression {
    #[cfg(all(feature = "Node", feature = "XPathExpression", feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `evaluate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)\n\n*This API requires the following crate features to be activated: `Node`, `XPathExpression`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn evaluate_with_type(
        &self,
        context_node: &Node,
        type_: u16,
    ) -> Result<XPathResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "XPathExpression", feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_evaluate_with_type_XPathExpression(
                self_: <&XPathExpression as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_evaluate_with_type_XPathExpression(
            self_: <&XPathExpression as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(context_node);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&XPathExpression as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let context_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_node);
                let type_ = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_evaluate_with_type_XPathExpression(self_, context_node, type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "XPathExpression", feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_evaluate_with_type_and_result_XPathExpression() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&XPathExpression as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <XPathResult as WasmDescribe>::describe();
}
impl XPathExpression {
    #[cfg(all(feature = "Node", feature = "XPathExpression", feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `evaluate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)\n\n*This API requires the following crate features to be activated: `Node`, `XPathExpression`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn evaluate_with_type_and_result(
        &self,
        context_node: &Node,
        type_: u16,
        result: Option<&::js_sys::Object>,
    ) -> Result<XPathResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "XPathExpression", feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_evaluate_with_type_and_result_XPathExpression(
                self_: <&XPathExpression as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                result: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_evaluate_with_type_and_result_XPathExpression(
            self_: <&XPathExpression as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            result: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(context_node);
            drop(type_);
            drop(result);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&XPathExpression as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let context_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_node);
                let type_ = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let result =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        result,
                    );
                __widl_f_evaluate_with_type_and_result_XPathExpression(
                    self_,
                    context_node,
                    type_,
                    result,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_be667d33124ebb6a: [u8; 479usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x9D\x01\0\0\0\0\x04\0\0\x02\x0FXPathExpression!__widl_instanceof_XPathExpression\0\0\0\0!__widl_f_evaluate_XPathExpression\x01\0\0\x01\x0FXPathExpression\x01\0\0\x01\x02\x05self_\x0Ccontext_node\x08evaluate\0\0\0+__widl_f_evaluate_with_type_XPathExpression\x01\0\0\x01\x0FXPathExpression\x01\0\0\x01\x03\x05self_\x0Ccontext_node\x05type_\x08evaluate\0\0\06__widl_f_evaluate_with_type_and_result_XPathExpression\x01\0\0\x01\x0FXPathExpression\x01\0\0\x01\x04\x05self_\x0Ccontext_node\x05type_\x06result\x08evaluate\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
