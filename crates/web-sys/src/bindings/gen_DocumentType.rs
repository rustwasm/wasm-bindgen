use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DocumentType` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DocumentType {
    obj: Node,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DocumentType: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DocumentType {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(68u32);
            inform(111u32);
            inform(99u32);
            inform(117u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(84u32);
            inform(121u32);
            inform(112u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for DocumentType {
        type Target = Node;
        #[inline]
        fn deref(&self) -> &Node {
            &self.obj
        }
    }
    impl IntoWasmAbi for DocumentType {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DocumentType {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DocumentType {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DocumentType {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DocumentType {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DocumentType {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DocumentType {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DocumentType {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DocumentType>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DocumentType {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DocumentType {
        #[inline]
        fn from(obj: JsValue) -> DocumentType {
            DocumentType { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DocumentType {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DocumentType> for DocumentType {
        #[inline]
        fn as_ref(&self) -> &DocumentType {
            self
        }
    }
    impl From<DocumentType> for JsValue {
        #[inline]
        fn from(obj: DocumentType) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DocumentType {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DocumentType(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DocumentType(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DocumentType(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DocumentType { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DocumentType) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DocumentType> for Node {
    #[inline]
    fn from(obj: DocumentType) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for DocumentType {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DocumentType> for EventTarget {
    #[inline]
    fn from(obj: DocumentType) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for DocumentType {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DocumentType> for ::js_sys::Object {
    #[inline]
    fn from(obj: DocumentType) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DocumentType {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentType as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/name)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_DocumentType(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_public_id_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentType as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `publicId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/publicId)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn public_id(&self) -> String {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_public_id_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_public_id_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_public_id_DocumentType(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_system_id_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentType as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `systemId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/systemId)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn system_id(&self) -> String {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_system_id_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_system_id_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_system_id_DocumentType(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn after_with_node(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_after_with_node_DocumentType(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_0_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_0_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_0_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_after_with_node_0_DocumentType(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_1_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_1_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_1_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_after_with_node_1_DocumentType(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_2_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_2_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_2_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_after_with_node_2_DocumentType(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_3_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_3_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_3_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_after_with_node_3_DocumentType(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_4_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_4_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_4_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_after_with_node_4_DocumentType(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_5_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_5_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_5_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_after_with_node_5_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_6_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_6(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_6_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_6_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_after_with_node_6_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_7_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_7(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_7_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_after_with_node_7_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_after_with_node_7_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn after_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_after_with_str_DocumentType(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_0_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_0_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_0_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_after_with_str_0_DocumentType(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_1_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_1_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_1_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_after_with_str_1_DocumentType(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_2_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_2_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_2_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_after_with_str_2_DocumentType(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_3_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_3_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_3_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_after_with_str_3_DocumentType(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_4_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_4_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_4_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_after_with_str_4_DocumentType(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_5_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_5_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_5_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_after_with_str_5_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_6_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_6(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_6_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_6_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_after_with_str_6_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_7_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_7(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_7_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_after_with_str_7_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_after_with_str_7_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn before_with_node(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_before_with_node_DocumentType(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_0_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_0_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_0_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_before_with_node_0_DocumentType(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_1_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_1_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_1_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_before_with_node_1_DocumentType(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_2_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_2_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_2_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_before_with_node_2_DocumentType(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_3_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_3_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_3_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_before_with_node_3_DocumentType(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_4_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_4_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_4_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_before_with_node_4_DocumentType(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_5_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_5_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_5_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_before_with_node_5_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_6_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_6(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_6_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_6_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_before_with_node_6_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_7_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_7(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_7_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_before_with_node_7_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_before_with_node_7_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn before_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_before_with_str_DocumentType(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_0_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_0_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_0_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_before_with_str_0_DocumentType(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_1_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_1_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_1_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_before_with_str_1_DocumentType(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_2_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_2_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_2_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_before_with_str_2_DocumentType(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_3_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_3_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_3_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_before_with_str_3_DocumentType(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_4_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_4_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_4_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_before_with_str_4_DocumentType(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_5_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_5_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_5_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_before_with_str_5_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_6_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_6(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_6_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_6_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_before_with_str_6_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_7_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_7(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_7_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_before_with_str_7_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_before_with_str_7_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/remove)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn remove(&self) {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_remove_DocumentType(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node(
        &self,
        nodes: &::js_sys::Array,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_replace_with_with_node_DocumentType(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_0_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_0_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_0_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_replace_with_with_node_0_DocumentType(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_1_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_1_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_1_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_replace_with_with_node_1_DocumentType(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_2_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_2_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_2_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_replace_with_with_node_2_DocumentType(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_3_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_3_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_3_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_replace_with_with_node_3_DocumentType(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_4_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_4_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_4_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_replace_with_with_node_4_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_5_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_5_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_5_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_replace_with_with_node_5_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_6_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_6(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_6_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_6_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_replace_with_with_node_6_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_7_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DocumentType as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_7(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_7_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_replace_with_with_node_7_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_replace_with_with_node_7_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str(
        &self,
        nodes: &::js_sys::Array,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_replace_with_with_str_DocumentType(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_0_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DocumentType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_0_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_0_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_replace_with_with_str_0_DocumentType(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_1_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_1_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_1_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_replace_with_with_str_1_DocumentType(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_2_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_2_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_2_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_replace_with_with_str_2_DocumentType(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_3_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_3_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_3_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_replace_with_with_str_3_DocumentType(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_4_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_4_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_4_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_replace_with_with_str_4_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_5_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_5_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_5_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_replace_with_with_str_5_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_6_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_6(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_6_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_6_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_replace_with_with_str_6_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_7_DocumentType() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DocumentType as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DocumentType {
    #[cfg(all(feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_7(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_7_DocumentType(
                self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_replace_with_with_str_7_DocumentType(
            self_: <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DocumentType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_replace_with_with_str_7_DocumentType(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8d4c1114f91b5ad5: [u8; 6219usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\t\x18\0\0\0\0;\0\0\x02\x0CDocumentType\x1E__widl_instanceof_DocumentType\0\0\0\0\x1A__widl_f_name_DocumentType\0\0\0\x01\x0CDocumentType\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\x1F__widl_f_public_id_DocumentType\0\0\0\x01\x0CDocumentType\x01\0\x01\x08publicId\x01\x01\x05self_\x08publicId\0\0\0\x1F__widl_f_system_id_DocumentType\0\0\0\x01\x0CDocumentType\x01\0\x01\x08systemId\x01\x01\x05self_\x08systemId\0\0\0%__widl_f_after_with_node_DocumentType\x01\x01\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x05nodes\x05after\0\0\0'__widl_f_after_with_node_0_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x01\x05self_\x05after\0\0\0'__widl_f_after_with_node_1_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x07nodes_1\x05after\0\0\0'__widl_f_after_with_node_2_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x05after\0\0\0'__widl_f_after_with_node_3_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x05after\0\0\0'__widl_f_after_with_node_4_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x05after\0\0\0'__widl_f_after_with_node_5_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x05after\0\0\0'__widl_f_after_with_node_6_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x05after\0\0\0'__widl_f_after_with_node_7_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x05after\0\0\0$__widl_f_after_with_str_DocumentType\x01\x01\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x05nodes\x05after\0\0\0&__widl_f_after_with_str_0_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x01\x05self_\x05after\0\0\0&__widl_f_after_with_str_1_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x07nodes_1\x05after\0\0\0&__widl_f_after_with_str_2_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x05after\0\0\0&__widl_f_after_with_str_3_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x05after\0\0\0&__widl_f_after_with_str_4_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x05after\0\0\0&__widl_f_after_with_str_5_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x05after\0\0\0&__widl_f_after_with_str_6_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x05after\0\0\0&__widl_f_after_with_str_7_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x05after\0\0\0&__widl_f_before_with_node_DocumentType\x01\x01\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x05nodes\x06before\0\0\0(__widl_f_before_with_node_0_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x01\x05self_\x06before\0\0\0(__widl_f_before_with_node_1_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x07nodes_1\x06before\0\0\0(__widl_f_before_with_node_2_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06before\0\0\0(__widl_f_before_with_node_3_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06before\0\0\0(__widl_f_before_with_node_4_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06before\0\0\0(__widl_f_before_with_node_5_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06before\0\0\0(__widl_f_before_with_node_6_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06before\0\0\0(__widl_f_before_with_node_7_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06before\0\0\0%__widl_f_before_with_str_DocumentType\x01\x01\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x05nodes\x06before\0\0\0'__widl_f_before_with_str_0_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x01\x05self_\x06before\0\0\0'__widl_f_before_with_str_1_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x07nodes_1\x06before\0\0\0'__widl_f_before_with_str_2_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06before\0\0\0'__widl_f_before_with_str_3_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06before\0\0\0'__widl_f_before_with_str_4_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06before\0\0\0'__widl_f_before_with_str_5_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06before\0\0\0'__widl_f_before_with_str_6_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06before\0\0\0'__widl_f_before_with_str_7_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06before\0\0\0\x1C__widl_f_remove_DocumentType\0\0\0\x01\x0CDocumentType\x01\0\0\x01\x01\x05self_\x06remove\0\0\0,__widl_f_replace_with_with_node_DocumentType\x01\x01\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x05nodes\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_node_0_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x01\x05self_\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_node_1_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x07nodes_1\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_node_2_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_node_3_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_node_4_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_node_5_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_node_6_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x0BreplaceWith\0\0\0.__widl_f_replace_with_with_node_7_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x0BreplaceWith\0\0\0+__widl_f_replace_with_with_str_DocumentType\x01\x01\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x05nodes\x0BreplaceWith\0\0\0-__widl_f_replace_with_with_str_0_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x01\x05self_\x0BreplaceWith\0\0\0-__widl_f_replace_with_with_str_1_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x02\x05self_\x07nodes_1\x0BreplaceWith\0\0\0-__widl_f_replace_with_with_str_2_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x0BreplaceWith\0\0\0-__widl_f_replace_with_with_str_3_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x0BreplaceWith\0\0\0-__widl_f_replace_with_with_str_4_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x0BreplaceWith\0\0\0-__widl_f_replace_with_with_str_5_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x0BreplaceWith\0\0\0-__widl_f_replace_with_with_str_6_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x0BreplaceWith\0\0\0-__widl_f_replace_with_with_str_7_DocumentType\x01\0\0\x01\x0CDocumentType\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x0BreplaceWith\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
