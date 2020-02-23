use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DocumentFragment` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DocumentFragment {
    obj: Node,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DocumentFragment: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DocumentFragment {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(68u32);
            inform(111u32);
            inform(99u32);
            inform(117u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(70u32);
            inform(114u32);
            inform(97u32);
            inform(103u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for DocumentFragment {
        type Target = Node;
        #[inline]
        fn deref(&self) -> &Node {
            &self.obj
        }
    }
    impl IntoWasmAbi for DocumentFragment {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DocumentFragment {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DocumentFragment {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DocumentFragment {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DocumentFragment {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DocumentFragment {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DocumentFragment {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DocumentFragment {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DocumentFragment>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DocumentFragment {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DocumentFragment {
        #[inline]
        fn from(obj: JsValue) -> DocumentFragment {
            DocumentFragment { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DocumentFragment {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DocumentFragment> for DocumentFragment {
        #[inline]
        fn as_ref(&self) -> &DocumentFragment {
            self
        }
    }
    impl From<DocumentFragment> for JsValue {
        #[inline]
        fn from(obj: DocumentFragment) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DocumentFragment {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DocumentFragment(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DocumentFragment(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DocumentFragment(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DocumentFragment { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DocumentFragment) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DocumentFragment> for Node {
    #[inline]
    fn from(obj: DocumentFragment) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for DocumentFragment {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DocumentFragment> for EventTarget {
    #[inline]
    fn from(obj: DocumentFragment) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for DocumentFragment {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DocumentFragment> for ::js_sys::Object {
    #[inline]
    fn from(obj: DocumentFragment) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DocumentFragment {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DocumentFragment as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `new DocumentFragment(..)` constructor, creating a new instance of `DocumentFragment`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/DocumentFragment)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DocumentFragment, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DocumentFragment(
            ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DocumentFragment(
        ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DocumentFragment() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_element_by_id_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `getElementById()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/getElementById)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
    #[allow(clippy::all)]
    pub fn get_element_by_id(&self, element_id: &str) -> Option<Element> {
        #[cfg(all(feature = "DocumentFragment", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_element_by_id_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_element_by_id_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(element_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element_id);
                __widl_f_get_element_by_id_DocumentFragment(self_, element_id)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_selector_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `querySelector()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelector)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
    #[allow(clippy::all)]
    pub fn query_selector(
        &self,
        selectors: &str,
    ) -> Result<Option<Element>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_selector_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selectors: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_selector_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selectors: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(selectors);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selectors = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selectors);
                __widl_f_query_selector_DocumentFragment(self_, selectors)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_selector_all_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `querySelectorAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelectorAll)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn query_selector_all(&self, selectors: &str) -> Result<NodeList, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_selector_all_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selectors: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_selector_all_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selectors: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(selectors);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selectors = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selectors);
                __widl_f_query_selector_all_DocumentFragment(self_, selectors)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<NodeList as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn append_with_node(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_append_with_node_DocumentFragment(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_0_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_0_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_0_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_append_with_node_0_DocumentFragment(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_1_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_1_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_1_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_append_with_node_1_DocumentFragment(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_2_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_2_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_2_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_append_with_node_2_DocumentFragment(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_3_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_3_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_3_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_append_with_node_3_DocumentFragment(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_4_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_4_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_4_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_append_with_node_4_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_5_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_5_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_5_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_append_with_node_5_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_6_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_6(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_6_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_6_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_append_with_node_6_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_7_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_7(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_7_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_7: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_7_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_7: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            drop(nodes_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_append_with_node_7_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn append_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_append_with_str_DocumentFragment(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_0_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_0_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_0_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_append_with_str_0_DocumentFragment(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_1_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_1_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_1_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_append_with_str_1_DocumentFragment(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_2_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_2_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_2_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_append_with_str_2_DocumentFragment(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_3_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_3_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_3_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_append_with_str_3_DocumentFragment(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_4_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_4_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_4_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_append_with_str_4_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_5_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_5_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_5_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_append_with_str_5_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_6_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_6(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_6_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_6_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_append_with_str_6_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_7_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_7(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_7_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_7_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            drop(nodes_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_append_with_str_7_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node(
        &self,
        nodes: &::js_sys::Array,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_prepend_with_node_DocumentFragment(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_0_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_0_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_0_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prepend_with_node_0_DocumentFragment(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_1_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_1_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_1_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_prepend_with_node_1_DocumentFragment(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_2_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_2_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_2_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_prepend_with_node_2_DocumentFragment(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_3_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_3_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_3_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_prepend_with_node_3_DocumentFragment(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_4_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_4_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_4_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_prepend_with_node_4_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_5_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_5_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_5_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_prepend_with_node_5_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_6_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_6(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_6_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_6_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_prepend_with_node_6_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_7_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_7(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_7_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_7: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_7_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_7: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            drop(nodes_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_prepend_with_node_7_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_prepend_with_str_DocumentFragment(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_0_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_0_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_0_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prepend_with_str_0_DocumentFragment(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_1_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_1_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_1_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_prepend_with_str_1_DocumentFragment(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_2_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_2_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_2_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_prepend_with_str_2_DocumentFragment(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_3_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_3_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_3_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_prepend_with_str_3_DocumentFragment(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_4_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_4_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_4_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_prepend_with_str_4_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_5_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_5_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_5_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_prepend_with_str_5_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_6_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_6(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_6_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_6_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_prepend_with_str_6_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_7_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_7(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_7_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_7_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nodes_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(nodes_1);
            drop(nodes_2);
            drop(nodes_3);
            drop(nodes_4);
            drop(nodes_5);
            drop(nodes_6);
            drop(nodes_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_prepend_with_str_7_DocumentFragment(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_children_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `children` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/children)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn children(&self) -> HtmlCollection {
        #[cfg(all(feature = "DocumentFragment", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_children_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_children_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_children_DocumentFragment(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_first_element_child_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `firstElementChild` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/firstElementChild)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
    #[allow(clippy::all)]
    pub fn first_element_child(&self) -> Option<Element> {
        #[cfg(all(feature = "DocumentFragment", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_first_element_child_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_first_element_child_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_first_element_child_DocumentFragment(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_last_element_child_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `lastElementChild` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/lastElementChild)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
    #[allow(clippy::all)]
    pub fn last_element_child(&self) -> Option<Element> {
        #[cfg(all(feature = "DocumentFragment", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_last_element_child_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_last_element_child_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_last_element_child_DocumentFragment(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_child_element_count_DocumentFragment() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentFragment as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl DocumentFragment {
    #[cfg(all(feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `childElementCount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/childElementCount)\n\n*This API requires the following crate features to be activated: `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn child_element_count(&self) -> u32 {
        #[cfg(all(feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_child_element_count_DocumentFragment(
                self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_child_element_count_DocumentFragment(
            self_: <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DocumentFragment as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_child_element_count_DocumentFragment(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_32ff4aff765cbad1: [u8; 5032usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}f\x13\0\0\0\0-\0\0\x02\x10DocumentFragment\"__widl_instanceof_DocumentFragment\0\0\0\0\x1D__widl_f_new_DocumentFragment\x01\0\0\x01\x10DocumentFragment\0\x01\0\x03new\0\0\0+__widl_f_get_element_by_id_DocumentFragment\0\0\0\x01\x10DocumentFragment\x01\0\0\x01\x02\x05self_\nelement_id\x0EgetElementById\0\0\0(__widl_f_query_selector_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x02\x05self_\tselectors\rquerySelector\0\0\0,__widl_f_query_selector_all_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x02\x05self_\tselectors\x10querySelectorAll\0\0\0*__widl_f_append_with_node_DocumentFragment\x01\x01\0\x01\x10DocumentFragment\x01\0\0\x01\x02\x05self_\x05nodes\x06append\0\0\0,__widl_f_append_with_node_0_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x01\x05self_\x06append\0\0\0,__widl_f_append_with_node_1_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x02\x05self_\x07nodes_1\x06append\0\0\0,__widl_f_append_with_node_2_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06append\0\0\0,__widl_f_append_with_node_3_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06append\0\0\0,__widl_f_append_with_node_4_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06append\0\0\0,__widl_f_append_with_node_5_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06append\0\0\0,__widl_f_append_with_node_6_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06append\0\0\0,__widl_f_append_with_node_7_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06append\0\0\0)__widl_f_append_with_str_DocumentFragment\x01\x01\0\x01\x10DocumentFragment\x01\0\0\x01\x02\x05self_\x05nodes\x06append\0\0\0+__widl_f_append_with_str_0_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x01\x05self_\x06append\0\0\0+__widl_f_append_with_str_1_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x02\x05self_\x07nodes_1\x06append\0\0\0+__widl_f_append_with_str_2_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06append\0\0\0+__widl_f_append_with_str_3_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06append\0\0\0+__widl_f_append_with_str_4_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06append\0\0\0+__widl_f_append_with_str_5_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06append\0\0\0+__widl_f_append_with_str_6_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06append\0\0\0+__widl_f_append_with_str_7_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06append\0\0\0+__widl_f_prepend_with_node_DocumentFragment\x01\x01\0\x01\x10DocumentFragment\x01\0\0\x01\x02\x05self_\x05nodes\x07prepend\0\0\0-__widl_f_prepend_with_node_0_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x01\x05self_\x07prepend\0\0\0-__widl_f_prepend_with_node_1_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x02\x05self_\x07nodes_1\x07prepend\0\0\0-__widl_f_prepend_with_node_2_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x07prepend\0\0\0-__widl_f_prepend_with_node_3_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07prepend\0\0\0-__widl_f_prepend_with_node_4_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07prepend\0\0\0-__widl_f_prepend_with_node_5_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07prepend\0\0\0-__widl_f_prepend_with_node_6_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07prepend\0\0\0-__widl_f_prepend_with_node_7_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x07prepend\0\0\0*__widl_f_prepend_with_str_DocumentFragment\x01\x01\0\x01\x10DocumentFragment\x01\0\0\x01\x02\x05self_\x05nodes\x07prepend\0\0\0,__widl_f_prepend_with_str_0_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x01\x05self_\x07prepend\0\0\0,__widl_f_prepend_with_str_1_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x02\x05self_\x07nodes_1\x07prepend\0\0\0,__widl_f_prepend_with_str_2_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x07prepend\0\0\0,__widl_f_prepend_with_str_3_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07prepend\0\0\0,__widl_f_prepend_with_str_4_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07prepend\0\0\0,__widl_f_prepend_with_str_5_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07prepend\0\0\0,__widl_f_prepend_with_str_6_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07prepend\0\0\0,__widl_f_prepend_with_str_7_DocumentFragment\x01\0\0\x01\x10DocumentFragment\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x07prepend\0\0\0\"__widl_f_children_DocumentFragment\0\0\0\x01\x10DocumentFragment\x01\0\x01\x08children\x01\x01\x05self_\x08children\0\0\0-__widl_f_first_element_child_DocumentFragment\0\0\0\x01\x10DocumentFragment\x01\0\x01\x11firstElementChild\x01\x01\x05self_\x11firstElementChild\0\0\0,__widl_f_last_element_child_DocumentFragment\0\0\0\x01\x10DocumentFragment\x01\0\x01\x10lastElementChild\x01\x01\x05self_\x10lastElementChild\0\0\0-__widl_f_child_element_count_DocumentFragment\0\0\0\x01\x10DocumentFragment\x01\0\x01\x11childElementCount\x01\x01\x05self_\x11childElementCount\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
