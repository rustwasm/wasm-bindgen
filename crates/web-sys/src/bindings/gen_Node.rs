use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Node` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node)\n\n*This API requires the following crate features to be activated: `Node`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Node {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Node: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Node {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(4u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for Node {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for Node {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Node {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Node {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Node {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Node {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Node {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Node {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Node {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Node>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Node {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Node {
        #[inline]
        fn from(obj: JsValue) -> Node {
            Node { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Node {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Node> for Node {
        #[inline]
        fn as_ref(&self) -> &Node {
            self
        }
    }
    impl From<Node> for JsValue {
        #[inline]
        fn from(obj: Node) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Node {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Node(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Node(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Node(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Node { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Node) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Node> for EventTarget {
    #[inline]
    fn from(obj: Node) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for Node {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Node> for ::js_sys::Object {
    #[inline]
    fn from(obj: Node) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Node {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_child_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `appendChild()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/appendChild)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn append_child(&self, node: &Node) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_child_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_child_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_append_child_Node(self_, node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clone_node_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `cloneNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn clone_node(&self) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clone_node_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clone_node_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clone_node_Node(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clone_node_with_deep_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `cloneNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn clone_node_with_deep(&self, deep: bool) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clone_node_with_deep_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                deep: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clone_node_with_deep_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            deep: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(deep);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let deep = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(deep);
                __widl_f_clone_node_with_deep_Node(self_, deep)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_compare_document_position_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `compareDocumentPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/compareDocumentPosition)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn compare_document_position(&self, other: &Node) -> u16 {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_compare_document_position_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                other: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_compare_document_position_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            other: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(other);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let other = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(other);
                __widl_f_compare_document_position_Node(self_, other)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_contains_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `contains()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/contains)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn contains(&self, other: Option<&Node>) -> bool {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_contains_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                other: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_contains_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            other: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(other);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let other = <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(other);
                __widl_f_contains_Node(self_, other)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_root_node_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `getRootNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn get_root_node(&self) -> Node {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_root_node_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_root_node_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_root_node_Node(self_)
            };
            <Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GetRootNodeOptions", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_root_node_with_options_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <&GetRootNodeOptions as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "GetRootNodeOptions", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `getRootNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)\n\n*This API requires the following crate features to be activated: `GetRootNodeOptions`, `Node`*"]
    #[allow(clippy::all)]
    pub fn get_root_node_with_options(&self, options: &GetRootNodeOptions) -> Node {
        #[cfg(all(feature = "GetRootNodeOptions", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_root_node_with_options_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&GetRootNodeOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_root_node_with_options_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&GetRootNodeOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&GetRootNodeOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_get_root_node_with_options_Node(self_, options)
            };
            <Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_child_nodes_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `hasChildNodes()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/hasChildNodes)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn has_child_nodes(&self) -> bool {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_child_nodes_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_child_nodes_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_has_child_nodes_Node(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_before_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `insertBefore()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/insertBefore)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn insert_before(
        &self,
        node: &Node,
        child: Option<&Node>,
    ) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_before_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                child: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_before_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            child: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node);
            drop(child);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                let child = <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(child);
                __widl_f_insert_before_Node(self_, node, child)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_default_namespace_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `isDefaultNamespace()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isDefaultNamespace)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn is_default_namespace(&self, namespace: Option<&str>) -> bool {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_default_namespace_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_default_namespace_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(namespace);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                __widl_f_is_default_namespace_Node(self_, namespace)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_equal_node_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `isEqualNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isEqualNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn is_equal_node(&self, node: Option<&Node>) -> bool {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_equal_node_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_equal_node_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_is_equal_node_Node(self_, node)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_same_node_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <Option<&Node> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `isSameNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isSameNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn is_same_node(&self, node: Option<&Node>) -> bool {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_same_node_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_same_node_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <Option<&Node> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_is_same_node_Node(self_, node)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lookup_namespace_uri_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `lookupNamespaceURI()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupNamespaceURI)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn lookup_namespace_uri(&self, prefix: Option<&str>) -> Option<String> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lookup_namespace_uri_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                prefix: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lookup_namespace_uri_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            prefix: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(prefix);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let prefix = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(prefix);
                __widl_f_lookup_namespace_uri_Node(self_, prefix)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lookup_prefix_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `lookupPrefix()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupPrefix)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn lookup_prefix(&self, namespace: Option<&str>) -> Option<String> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lookup_prefix_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lookup_prefix_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(namespace);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                __widl_f_lookup_prefix_Node(self_, namespace)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_normalize_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `normalize()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/normalize)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn normalize(&self) {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_normalize_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_normalize_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_normalize_Node(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_child_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `removeChild()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/removeChild)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn remove_child(&self, child: &Node) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_child_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                child: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_child_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            child: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(child);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let child = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(child);
                __widl_f_remove_child_Node(self_, child)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_child_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceChild()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/replaceChild)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_child(
        &self,
        node: &Node,
        child: &Node,
    ) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_child_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                child: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_child_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            child: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node);
            drop(child);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                let child = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(child);
                __widl_f_replace_child_Node(self_, node, child)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_node_type_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `nodeType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn node_type(&self) -> u16 {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_node_type_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_node_type_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_node_type_Node(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_node_name_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `nodeName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeName)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn node_name(&self) -> String {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_node_name_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_node_name_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_node_name_Node(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_base_uri_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `baseURI` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn base_uri(&self) -> Result<Option<String>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_base_uri_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_base_uri_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_base_uri_Node(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_connected_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `isConnected` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isConnected)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn is_connected(&self) -> bool {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_connected_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_connected_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_connected_Node(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_owner_document_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Option<Document> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `ownerDocument` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/ownerDocument)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn owner_document(&self) -> Option<Document> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_owner_document_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_owner_document_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_owner_document_Node(self_)
            };
            <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_parent_node_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `parentNode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn parent_node(&self) -> Option<Node> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_parent_node_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_parent_node_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_parent_node_Node(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_parent_element_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `parentElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentElement)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn parent_element(&self) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_parent_element_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_parent_element_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_parent_element_Node(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_child_nodes_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `childNodes` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/childNodes)\n\n*This API requires the following crate features to be activated: `Node`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn child_nodes(&self) -> NodeList {
        #[cfg(all(feature = "Node", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_child_nodes_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_child_nodes_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_child_nodes_Node(self_)
            };
            <NodeList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_first_child_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `firstChild` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/firstChild)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn first_child(&self) -> Option<Node> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_first_child_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_first_child_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_first_child_Node(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_last_child_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `lastChild` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lastChild)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn last_child(&self) -> Option<Node> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_last_child_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_last_child_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_last_child_Node(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_previous_sibling_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `previousSibling` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/previousSibling)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn previous_sibling(&self) -> Option<Node> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_previous_sibling_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_previous_sibling_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_previous_sibling_Node(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_next_sibling_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `nextSibling` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nextSibling)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn next_sibling(&self) -> Option<Node> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_next_sibling_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_next_sibling_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_next_sibling_Node(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_node_value_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `nodeValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn node_value(&self) -> Option<String> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_node_value_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_node_value_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_node_value_Node(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_node_value_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `nodeValue` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn set_node_value(&self, node_value: Option<&str>) {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_node_value_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_node_value_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node_value: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(node_value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node_value =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node_value);
                __widl_f_set_node_value_Node(self_, node_value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_content_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Node as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `textContent` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn text_content(&self) -> Option<String> {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_content_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_content_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_content_Node(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_content_Node() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Node as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Node {
    #[cfg(all(feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `textContent` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)\n\n*This API requires the following crate features to be activated: `Node`*"]
    #[allow(clippy::all)]
    pub fn set_text_content(&self, text_content: Option<&str>) {
        #[cfg(all(feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_content_Node(
                self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_content: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_content_Node(
            self_: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_content: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_content);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text_content =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_content);
                __widl_f_set_text_content_Node(self_, text_content)
            };
            ()
        }
    }
}
impl Node {
    pub const ELEMENT_NODE: u16 = 1u64 as u16;
}
impl Node {
    pub const ATTRIBUTE_NODE: u16 = 2u64 as u16;
}
impl Node {
    pub const TEXT_NODE: u16 = 3u64 as u16;
}
impl Node {
    pub const CDATA_SECTION_NODE: u16 = 4u64 as u16;
}
impl Node {
    pub const ENTITY_REFERENCE_NODE: u16 = 5u64 as u16;
}
impl Node {
    pub const ENTITY_NODE: u16 = 6u64 as u16;
}
impl Node {
    pub const PROCESSING_INSTRUCTION_NODE: u16 = 7u64 as u16;
}
impl Node {
    pub const COMMENT_NODE: u16 = 8u64 as u16;
}
impl Node {
    pub const DOCUMENT_NODE: u16 = 9u64 as u16;
}
impl Node {
    pub const DOCUMENT_TYPE_NODE: u16 = 10u64 as u16;
}
impl Node {
    pub const DOCUMENT_FRAGMENT_NODE: u16 = 11u64 as u16;
}
impl Node {
    pub const NOTATION_NODE: u16 = 12u64 as u16;
}
impl Node {
    pub const DOCUMENT_POSITION_DISCONNECTED: u16 = 1u64 as u16;
}
impl Node {
    pub const DOCUMENT_POSITION_PRECEDING: u16 = 2u64 as u16;
}
impl Node {
    pub const DOCUMENT_POSITION_FOLLOWING: u16 = 4u64 as u16;
}
impl Node {
    pub const DOCUMENT_POSITION_CONTAINS: u16 = 8u64 as u16;
}
impl Node {
    pub const DOCUMENT_POSITION_CONTAINED_BY: u16 = 16u64 as u16;
}
impl Node {
    pub const DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: u16 = 32u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8a0dd04bbf142251: [u8; 2540usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAA\t\0\0\0\0\"\0\0\x02\x04Node\x16__widl_instanceof_Node\0\0\0\0\x1A__widl_f_append_child_Node\x01\0\0\x01\x04Node\x01\0\0\x01\x02\x05self_\x04node\x0BappendChild\0\0\0\x18__widl_f_clone_node_Node\x01\0\0\x01\x04Node\x01\0\0\x01\x01\x05self_\tcloneNode\0\0\0\"__widl_f_clone_node_with_deep_Node\x01\0\0\x01\x04Node\x01\0\0\x01\x02\x05self_\x04deep\tcloneNode\0\0\0'__widl_f_compare_document_position_Node\0\0\0\x01\x04Node\x01\0\0\x01\x02\x05self_\x05other\x17compareDocumentPosition\0\0\0\x16__widl_f_contains_Node\0\0\0\x01\x04Node\x01\0\0\x01\x02\x05self_\x05other\x08contains\0\0\0\x1B__widl_f_get_root_node_Node\0\0\0\x01\x04Node\x01\0\0\x01\x01\x05self_\x0BgetRootNode\0\0\0(__widl_f_get_root_node_with_options_Node\0\0\0\x01\x04Node\x01\0\0\x01\x02\x05self_\x07options\x0BgetRootNode\0\0\0\x1D__widl_f_has_child_nodes_Node\0\0\0\x01\x04Node\x01\0\0\x01\x01\x05self_\rhasChildNodes\0\0\0\x1B__widl_f_insert_before_Node\x01\0\0\x01\x04Node\x01\0\0\x01\x03\x05self_\x04node\x05child\x0CinsertBefore\0\0\0\"__widl_f_is_default_namespace_Node\0\0\0\x01\x04Node\x01\0\0\x01\x02\x05self_\tnamespace\x12isDefaultNamespace\0\0\0\x1B__widl_f_is_equal_node_Node\0\0\0\x01\x04Node\x01\0\0\x01\x02\x05self_\x04node\x0BisEqualNode\0\0\0\x1A__widl_f_is_same_node_Node\0\0\0\x01\x04Node\x01\0\0\x01\x02\x05self_\x04node\nisSameNode\0\0\0\"__widl_f_lookup_namespace_uri_Node\0\0\0\x01\x04Node\x01\0\0\x01\x02\x05self_\x06prefix\x12lookupNamespaceURI\0\0\0\x1B__widl_f_lookup_prefix_Node\0\0\0\x01\x04Node\x01\0\0\x01\x02\x05self_\tnamespace\x0ClookupPrefix\0\0\0\x17__widl_f_normalize_Node\0\0\0\x01\x04Node\x01\0\0\x01\x01\x05self_\tnormalize\0\0\0\x1A__widl_f_remove_child_Node\x01\0\0\x01\x04Node\x01\0\0\x01\x02\x05self_\x05child\x0BremoveChild\0\0\0\x1B__widl_f_replace_child_Node\x01\0\0\x01\x04Node\x01\0\0\x01\x03\x05self_\x04node\x05child\x0CreplaceChild\0\0\0\x17__widl_f_node_type_Node\0\0\0\x01\x04Node\x01\0\x01\x08nodeType\x01\x01\x05self_\x08nodeType\0\0\0\x17__widl_f_node_name_Node\0\0\0\x01\x04Node\x01\0\x01\x08nodeName\x01\x01\x05self_\x08nodeName\0\0\0\x16__widl_f_base_uri_Node\x01\0\0\x01\x04Node\x01\0\x01\x07baseURI\x01\x01\x05self_\x07baseURI\0\0\0\x1A__widl_f_is_connected_Node\0\0\0\x01\x04Node\x01\0\x01\x0BisConnected\x01\x01\x05self_\x0BisConnected\0\0\0\x1C__widl_f_owner_document_Node\0\0\0\x01\x04Node\x01\0\x01\rownerDocument\x01\x01\x05self_\rownerDocument\0\0\0\x19__widl_f_parent_node_Node\0\0\0\x01\x04Node\x01\0\x01\nparentNode\x01\x01\x05self_\nparentNode\0\0\0\x1C__widl_f_parent_element_Node\0\0\0\x01\x04Node\x01\0\x01\rparentElement\x01\x01\x05self_\rparentElement\0\0\0\x19__widl_f_child_nodes_Node\0\0\0\x01\x04Node\x01\0\x01\nchildNodes\x01\x01\x05self_\nchildNodes\0\0\0\x19__widl_f_first_child_Node\0\0\0\x01\x04Node\x01\0\x01\nfirstChild\x01\x01\x05self_\nfirstChild\0\0\0\x18__widl_f_last_child_Node\0\0\0\x01\x04Node\x01\0\x01\tlastChild\x01\x01\x05self_\tlastChild\0\0\0\x1E__widl_f_previous_sibling_Node\0\0\0\x01\x04Node\x01\0\x01\x0FpreviousSibling\x01\x01\x05self_\x0FpreviousSibling\0\0\0\x1A__widl_f_next_sibling_Node\0\0\0\x01\x04Node\x01\0\x01\x0BnextSibling\x01\x01\x05self_\x0BnextSibling\0\0\0\x18__widl_f_node_value_Node\0\0\0\x01\x04Node\x01\0\x01\tnodeValue\x01\x01\x05self_\tnodeValue\0\0\0\x1C__widl_f_set_node_value_Node\0\0\0\x01\x04Node\x01\0\x02\tnodeValue\x01\x02\x05self_\nnode_value\tnodeValue\0\0\0\x1A__widl_f_text_content_Node\0\0\0\x01\x04Node\x01\0\x01\x0BtextContent\x01\x01\x05self_\x0BtextContent\0\0\0\x1E__widl_f_set_text_content_Node\0\0\0\x01\x04Node\x01\0\x02\x0BtextContent\x01\x02\x05self_\x0Ctext_content\x0BtextContent\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
