use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `NodeIterator` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator)\n\n*This API requires the following crate features to be activated: `NodeIterator`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct NodeIterator {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_NodeIterator: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for NodeIterator {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
            inform(73u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(97u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for NodeIterator {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for NodeIterator {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for NodeIterator {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a NodeIterator {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for NodeIterator {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            NodeIterator {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for NodeIterator {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a NodeIterator {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for NodeIterator {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<NodeIterator>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(NodeIterator {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for NodeIterator {
        #[inline]
        fn from(obj: JsValue) -> NodeIterator {
            NodeIterator { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for NodeIterator {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<NodeIterator> for NodeIterator {
        #[inline]
        fn as_ref(&self) -> &NodeIterator {
            self
        }
    }
    impl From<NodeIterator> for JsValue {
        #[inline]
        fn from(obj: NodeIterator) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for NodeIterator {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_NodeIterator(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_NodeIterator(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_NodeIterator(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            NodeIterator { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const NodeIterator) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<NodeIterator> for ::js_sys::Object {
    #[inline]
    fn from(obj: NodeIterator) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for NodeIterator {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "NodeIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_detach_NodeIterator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NodeIterator as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl NodeIterator {
    #[cfg(all(feature = "NodeIterator",))]
    #[allow(bad_style)]
    #[doc = "The `detach()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/detach)\n\n*This API requires the following crate features to be activated: `NodeIterator`*"]
    #[allow(clippy::all)]
    pub fn detach(&self) {
        #[cfg(all(feature = "NodeIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_detach_NodeIterator(
                self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_detach_NodeIterator(
            self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_detach_NodeIterator(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Node", feature = "NodeIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_next_node_NodeIterator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NodeIterator as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl NodeIterator {
    #[cfg(all(feature = "Node", feature = "NodeIterator",))]
    #[allow(bad_style)]
    #[doc = "The `nextNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/nextNode)\n\n*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    #[allow(clippy::all)]
    pub fn next_node(&self) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "NodeIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_next_node_NodeIterator(
                self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_next_node_NodeIterator(
            self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_next_node_NodeIterator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "NodeIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_previous_node_NodeIterator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NodeIterator as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl NodeIterator {
    #[cfg(all(feature = "Node", feature = "NodeIterator",))]
    #[allow(bad_style)]
    #[doc = "The `previousNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/previousNode)\n\n*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    #[allow(clippy::all)]
    pub fn previous_node(&self) -> Result<Option<Node>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node", feature = "NodeIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_previous_node_NodeIterator(
                self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_previous_node_NodeIterator(
            self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_previous_node_NodeIterator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "NodeIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_root_NodeIterator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NodeIterator as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl NodeIterator {
    #[cfg(all(feature = "Node", feature = "NodeIterator",))]
    #[allow(bad_style)]
    #[doc = "The `root` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/root)\n\n*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    #[allow(clippy::all)]
    pub fn root(&self) -> Node {
        #[cfg(all(feature = "Node", feature = "NodeIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_root_NodeIterator(
                self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_root_NodeIterator(
            self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_root_NodeIterator(self_)
            };
            <Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node", feature = "NodeIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reference_node_NodeIterator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NodeIterator as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl NodeIterator {
    #[cfg(all(feature = "Node", feature = "NodeIterator",))]
    #[allow(bad_style)]
    #[doc = "The `referenceNode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/referenceNode)\n\n*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    #[allow(clippy::all)]
    pub fn reference_node(&self) -> Option<Node> {
        #[cfg(all(feature = "Node", feature = "NodeIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reference_node_NodeIterator(
                self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reference_node_NodeIterator(
            self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_reference_node_NodeIterator(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "NodeIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pointer_before_reference_node_NodeIterator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NodeIterator as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl NodeIterator {
    #[cfg(all(feature = "NodeIterator",))]
    #[allow(bad_style)]
    #[doc = "The `pointerBeforeReferenceNode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/pointerBeforeReferenceNode)\n\n*This API requires the following crate features to be activated: `NodeIterator`*"]
    #[allow(clippy::all)]
    pub fn pointer_before_reference_node(&self) -> bool {
        #[cfg(all(feature = "NodeIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pointer_before_reference_node_NodeIterator(
                self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pointer_before_reference_node_NodeIterator(
            self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pointer_before_reference_node_NodeIterator(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "NodeIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_what_to_show_NodeIterator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NodeIterator as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl NodeIterator {
    #[cfg(all(feature = "NodeIterator",))]
    #[allow(bad_style)]
    #[doc = "The `whatToShow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/whatToShow)\n\n*This API requires the following crate features to be activated: `NodeIterator`*"]
    #[allow(clippy::all)]
    pub fn what_to_show(&self) -> u32 {
        #[cfg(all(feature = "NodeIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_what_to_show_NodeIterator(
                self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_what_to_show_NodeIterator(
            self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_what_to_show_NodeIterator(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "NodeFilter", feature = "NodeIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_filter_NodeIterator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NodeIterator as WasmDescribe>::describe();
    <Option<NodeFilter> as WasmDescribe>::describe();
}
impl NodeIterator {
    #[cfg(all(feature = "NodeFilter", feature = "NodeIterator",))]
    #[allow(bad_style)]
    #[doc = "The `filter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/filter)\n\n*This API requires the following crate features to be activated: `NodeFilter`, `NodeIterator`*"]
    #[allow(clippy::all)]
    pub fn filter(&self) -> Option<NodeFilter> {
        #[cfg(all(feature = "NodeFilter", feature = "NodeIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_filter_NodeIterator(
                self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<NodeFilter> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_filter_NodeIterator(
            self_: <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<NodeFilter> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NodeIterator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_filter_NodeIterator(self_)
            };
            <Option<NodeFilter> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a2fefcd6b7160c3f: [u8; 835usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x01\x03\0\0\0\0\t\0\0\x02\x0CNodeIterator\x1E__widl_instanceof_NodeIterator\0\0\0\0\x1C__widl_f_detach_NodeIterator\0\0\0\x01\x0CNodeIterator\x01\0\0\x01\x01\x05self_\x06detach\0\0\0\x1F__widl_f_next_node_NodeIterator\x01\0\0\x01\x0CNodeIterator\x01\0\0\x01\x01\x05self_\x08nextNode\0\0\0#__widl_f_previous_node_NodeIterator\x01\0\0\x01\x0CNodeIterator\x01\0\0\x01\x01\x05self_\x0CpreviousNode\0\0\0\x1A__widl_f_root_NodeIterator\0\0\0\x01\x0CNodeIterator\x01\0\x01\x04root\x01\x01\x05self_\x04root\0\0\0$__widl_f_reference_node_NodeIterator\0\0\0\x01\x0CNodeIterator\x01\0\x01\rreferenceNode\x01\x01\x05self_\rreferenceNode\0\0\03__widl_f_pointer_before_reference_node_NodeIterator\0\0\0\x01\x0CNodeIterator\x01\0\x01\x1ApointerBeforeReferenceNode\x01\x01\x05self_\x1ApointerBeforeReferenceNode\0\0\0\"__widl_f_what_to_show_NodeIterator\0\0\0\x01\x0CNodeIterator\x01\0\x01\nwhatToShow\x01\x01\x05self_\nwhatToShow\0\0\0\x1C__widl_f_filter_NodeIterator\0\0\0\x01\x0CNodeIterator\x01\0\x01\x06filter\x01\x01\x05self_\x06filter\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
