use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MutationRecord` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord)\n\n*This API requires the following crate features to be activated: `MutationRecord`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MutationRecord {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MutationRecord: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MutationRecord {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(77u32);
            inform(117u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(82u32);
            inform(101u32);
            inform(99u32);
            inform(111u32);
            inform(114u32);
            inform(100u32);
        }
    }
    impl core::ops::Deref for MutationRecord {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MutationRecord {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MutationRecord {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MutationRecord {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MutationRecord {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MutationRecord {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MutationRecord {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MutationRecord {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MutationRecord {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MutationRecord>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MutationRecord {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MutationRecord {
        #[inline]
        fn from(obj: JsValue) -> MutationRecord {
            MutationRecord { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MutationRecord {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MutationRecord> for MutationRecord {
        #[inline]
        fn as_ref(&self) -> &MutationRecord {
            self
        }
    }
    impl From<MutationRecord> for JsValue {
        #[inline]
        fn from(obj: MutationRecord) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MutationRecord {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MutationRecord(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MutationRecord(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MutationRecord(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MutationRecord { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MutationRecord) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MutationRecord> for ::js_sys::Object {
    #[inline]
    fn from(obj: MutationRecord) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MutationRecord {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MutationRecord",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_MutationRecord() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationRecord as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MutationRecord {
    #[cfg(all(feature = "MutationRecord",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/type)\n\n*This API requires the following crate features to be activated: `MutationRecord`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "MutationRecord",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_MutationRecord(
                self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_MutationRecord(
            self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_MutationRecord(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationRecord", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_MutationRecord() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationRecord as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl MutationRecord {
    #[cfg(all(feature = "MutationRecord", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/target)\n\n*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> Option<Node> {
        #[cfg(all(feature = "MutationRecord", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_MutationRecord(
                self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_MutationRecord(
            self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_MutationRecord(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationRecord", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_added_nodes_MutationRecord() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationRecord as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl MutationRecord {
    #[cfg(all(feature = "MutationRecord", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `addedNodes` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/addedNodes)\n\n*This API requires the following crate features to be activated: `MutationRecord`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn added_nodes(&self) -> NodeList {
        #[cfg(all(feature = "MutationRecord", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_added_nodes_MutationRecord(
                self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_added_nodes_MutationRecord(
            self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_added_nodes_MutationRecord(self_)
            };
            <NodeList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationRecord", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_removed_nodes_MutationRecord() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationRecord as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl MutationRecord {
    #[cfg(all(feature = "MutationRecord", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `removedNodes` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/removedNodes)\n\n*This API requires the following crate features to be activated: `MutationRecord`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn removed_nodes(&self) -> NodeList {
        #[cfg(all(feature = "MutationRecord", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_removed_nodes_MutationRecord(
                self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_removed_nodes_MutationRecord(
            self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_removed_nodes_MutationRecord(self_)
            };
            <NodeList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationRecord", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_previous_sibling_MutationRecord() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationRecord as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl MutationRecord {
    #[cfg(all(feature = "MutationRecord", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `previousSibling` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/previousSibling)\n\n*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
    #[allow(clippy::all)]
    pub fn previous_sibling(&self) -> Option<Node> {
        #[cfg(all(feature = "MutationRecord", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_previous_sibling_MutationRecord(
                self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_previous_sibling_MutationRecord(
            self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_previous_sibling_MutationRecord(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationRecord", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_next_sibling_MutationRecord() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationRecord as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl MutationRecord {
    #[cfg(all(feature = "MutationRecord", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `nextSibling` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/nextSibling)\n\n*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
    #[allow(clippy::all)]
    pub fn next_sibling(&self) -> Option<Node> {
        #[cfg(all(feature = "MutationRecord", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_next_sibling_MutationRecord(
                self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_next_sibling_MutationRecord(
            self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_next_sibling_MutationRecord(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationRecord",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_attribute_name_MutationRecord() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationRecord as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl MutationRecord {
    #[cfg(all(feature = "MutationRecord",))]
    #[allow(bad_style)]
    #[doc = "The `attributeName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/attributeName)\n\n*This API requires the following crate features to be activated: `MutationRecord`*"]
    #[allow(clippy::all)]
    pub fn attribute_name(&self) -> Option<String> {
        #[cfg(all(feature = "MutationRecord",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_attribute_name_MutationRecord(
                self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_attribute_name_MutationRecord(
            self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_attribute_name_MutationRecord(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationRecord",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_attribute_namespace_MutationRecord() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationRecord as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl MutationRecord {
    #[cfg(all(feature = "MutationRecord",))]
    #[allow(bad_style)]
    #[doc = "The `attributeNamespace` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/attributeNamespace)\n\n*This API requires the following crate features to be activated: `MutationRecord`*"]
    #[allow(clippy::all)]
    pub fn attribute_namespace(&self) -> Option<String> {
        #[cfg(all(feature = "MutationRecord",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_attribute_namespace_MutationRecord(
                self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_attribute_namespace_MutationRecord(
            self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_attribute_namespace_MutationRecord(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MutationRecord",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_old_value_MutationRecord() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MutationRecord as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl MutationRecord {
    #[cfg(all(feature = "MutationRecord",))]
    #[allow(bad_style)]
    #[doc = "The `oldValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/oldValue)\n\n*This API requires the following crate features to be activated: `MutationRecord`*"]
    #[allow(clippy::all)]
    pub fn old_value(&self) -> Option<String> {
        #[cfg(all(feature = "MutationRecord",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_old_value_MutationRecord(
                self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_old_value_MutationRecord(
            self_: <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MutationRecord as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_old_value_MutationRecord(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1fc886cf9a4381f6: [u8; 995usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA1\x03\0\0\0\0\n\0\0\x02\x0EMutationRecord __widl_instanceof_MutationRecord\0\0\0\0\x1C__widl_f_type_MutationRecord\0\0\0\x01\x0EMutationRecord\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\x1E__widl_f_target_MutationRecord\0\0\0\x01\x0EMutationRecord\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0#__widl_f_added_nodes_MutationRecord\0\0\0\x01\x0EMutationRecord\x01\0\x01\naddedNodes\x01\x01\x05self_\naddedNodes\0\0\0%__widl_f_removed_nodes_MutationRecord\0\0\0\x01\x0EMutationRecord\x01\0\x01\x0CremovedNodes\x01\x01\x05self_\x0CremovedNodes\0\0\0(__widl_f_previous_sibling_MutationRecord\0\0\0\x01\x0EMutationRecord\x01\0\x01\x0FpreviousSibling\x01\x01\x05self_\x0FpreviousSibling\0\0\0$__widl_f_next_sibling_MutationRecord\0\0\0\x01\x0EMutationRecord\x01\0\x01\x0BnextSibling\x01\x01\x05self_\x0BnextSibling\0\0\0&__widl_f_attribute_name_MutationRecord\0\0\0\x01\x0EMutationRecord\x01\0\x01\rattributeName\x01\x01\x05self_\rattributeName\0\0\0+__widl_f_attribute_namespace_MutationRecord\0\0\0\x01\x0EMutationRecord\x01\0\x01\x12attributeNamespace\x01\x01\x05self_\x12attributeNamespace\0\0\0!__widl_f_old_value_MutationRecord\0\0\0\x01\x0EMutationRecord\x01\0\x01\x08oldValue\x01\x01\x05self_\x08oldValue\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
