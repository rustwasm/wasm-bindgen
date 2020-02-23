use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `XPathResult` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct XPathResult {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_XPathResult: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for XPathResult {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(88u32);
            inform(80u32);
            inform(97u32);
            inform(116u32);
            inform(104u32);
            inform(82u32);
            inform(101u32);
            inform(115u32);
            inform(117u32);
            inform(108u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for XPathResult {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for XPathResult {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for XPathResult {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a XPathResult {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for XPathResult {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            XPathResult {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for XPathResult {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a XPathResult {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for XPathResult {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<XPathResult>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(XPathResult {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for XPathResult {
        #[inline]
        fn from(obj: JsValue) -> XPathResult {
            XPathResult { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for XPathResult {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<XPathResult> for XPathResult {
        #[inline]
        fn as_ref(&self) -> &XPathResult {
            self
        }
    }
    impl From<XPathResult> for JsValue {
        #[inline]
        fn from(obj: XPathResult) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for XPathResult {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_XPathResult(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_XPathResult(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_XPathResult(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            XPathResult { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const XPathResult) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<XPathResult> for ::js_sys::Object {
    #[inline]
    fn from(obj: XPathResult) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for XPathResult {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Node", feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_iterate_next_XPathResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XPathResult as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl XPathResult {
    #[cfg(all(feature = "Node", feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `iterateNext()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/iterateNext)\n\n*This API requires the following crate features to be activated: `Node`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn iterate_next(&self) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_iterate_next_XPathResult(
                self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_iterate_next_XPathResult(
            self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_iterate_next_XPathResult(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_snapshot_item_XPathResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XPathResult as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl XPathResult {
    #[cfg(all(feature = "Node", feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `snapshotItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/snapshotItem)\n\n*This API requires the following crate features to be activated: `Node`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn snapshot_item(&self, index: u32) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_snapshot_item_XPathResult(
                self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_snapshot_item_XPathResult(
            self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_snapshot_item_XPathResult(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_type_XPathResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XPathResult as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl XPathResult {
    #[cfg(all(feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `resultType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/resultType)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn result_type(&self) -> u16 {
        #[cfg(all(feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_type_XPathResult(
                self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_type_XPathResult(
            self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_result_type_XPathResult(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_number_value_XPathResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XPathResult as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl XPathResult {
    #[cfg(all(feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `numberValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/numberValue)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn number_value(&self) -> Result<f64, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_number_value_XPathResult(
                self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_number_value_XPathResult(
            self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_number_value_XPathResult(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_string_value_XPathResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XPathResult as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl XPathResult {
    #[cfg(all(feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `stringValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/stringValue)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn string_value(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_string_value_XPathResult(
                self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_string_value_XPathResult(
            self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_string_value_XPathResult(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_boolean_value_XPathResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XPathResult as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl XPathResult {
    #[cfg(all(feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `booleanValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/booleanValue)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn boolean_value(&self) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_boolean_value_XPathResult(
                self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_boolean_value_XPathResult(
            self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_boolean_value_XPathResult(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_single_node_value_XPathResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XPathResult as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl XPathResult {
    #[cfg(all(feature = "Node", feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `singleNodeValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/singleNodeValue)\n\n*This API requires the following crate features to be activated: `Node`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn single_node_value(&self) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_single_node_value_XPathResult(
                self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_single_node_value_XPathResult(
            self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_single_node_value_XPathResult(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_invalid_iterator_state_XPathResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XPathResult as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl XPathResult {
    #[cfg(all(feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `invalidIteratorState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/invalidIteratorState)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn invalid_iterator_state(&self) -> bool {
        #[cfg(all(feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_invalid_iterator_state_XPathResult(
                self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_invalid_iterator_state_XPathResult(
            self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_invalid_iterator_state_XPathResult(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_snapshot_length_XPathResult() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XPathResult as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl XPathResult {
    #[cfg(all(feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `snapshotLength` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/snapshotLength)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn snapshot_length(&self) -> Result<u32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_snapshot_length_XPathResult(
                self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_snapshot_length_XPathResult(
            self_: <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&XPathResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_snapshot_length_XPathResult(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
impl XPathResult {
    pub const ANY_TYPE: u16 = 0i64 as u16;
}
impl XPathResult {
    pub const NUMBER_TYPE: u16 = 1u64 as u16;
}
impl XPathResult {
    pub const STRING_TYPE: u16 = 2u64 as u16;
}
impl XPathResult {
    pub const BOOLEAN_TYPE: u16 = 3u64 as u16;
}
impl XPathResult {
    pub const UNORDERED_NODE_ITERATOR_TYPE: u16 = 4u64 as u16;
}
impl XPathResult {
    pub const ORDERED_NODE_ITERATOR_TYPE: u16 = 5u64 as u16;
}
impl XPathResult {
    pub const UNORDERED_NODE_SNAPSHOT_TYPE: u16 = 6u64 as u16;
}
impl XPathResult {
    pub const ORDERED_NODE_SNAPSHOT_TYPE: u16 = 7u64 as u16;
}
impl XPathResult {
    pub const ANY_UNORDERED_NODE_TYPE: u16 = 8u64 as u16;
}
impl XPathResult {
    pub const FIRST_ORDERED_NODE_TYPE: u16 = 9u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_65c4a45490c103d5: [u8; 977usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x8F\x03\0\0\0\0\n\0\0\x02\x0BXPathResult\x1D__widl_instanceof_XPathResult\0\0\0\0!__widl_f_iterate_next_XPathResult\x01\0\0\x01\x0BXPathResult\x01\0\0\x01\x01\x05self_\x0BiterateNext\0\0\0\"__widl_f_snapshot_item_XPathResult\x01\0\0\x01\x0BXPathResult\x01\0\0\x01\x02\x05self_\x05index\x0CsnapshotItem\0\0\0 __widl_f_result_type_XPathResult\0\0\0\x01\x0BXPathResult\x01\0\x01\nresultType\x01\x01\x05self_\nresultType\0\0\0!__widl_f_number_value_XPathResult\x01\0\0\x01\x0BXPathResult\x01\0\x01\x0BnumberValue\x01\x01\x05self_\x0BnumberValue\0\0\0!__widl_f_string_value_XPathResult\x01\0\0\x01\x0BXPathResult\x01\0\x01\x0BstringValue\x01\x01\x05self_\x0BstringValue\0\0\0\"__widl_f_boolean_value_XPathResult\x01\0\0\x01\x0BXPathResult\x01\0\x01\x0CbooleanValue\x01\x01\x05self_\x0CbooleanValue\0\0\0&__widl_f_single_node_value_XPathResult\x01\0\0\x01\x0BXPathResult\x01\0\x01\x0FsingleNodeValue\x01\x01\x05self_\x0FsingleNodeValue\0\0\0+__widl_f_invalid_iterator_state_XPathResult\0\0\0\x01\x0BXPathResult\x01\0\x01\x14invalidIteratorState\x01\x01\x05self_\x14invalidIteratorState\0\0\0$__widl_f_snapshot_length_XPathResult\x01\0\0\x01\x0BXPathResult\x01\0\x01\x0EsnapshotLength\x01\x01\x05self_\x0EsnapshotLength\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
