use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Document` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document)\n\n*This API requires the following crate features to be activated: `Document`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Document {
    obj: Node,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Document: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Document {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
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
    impl core::ops::Deref for Document {
        type Target = Node;
        #[inline]
        fn deref(&self) -> &Node {
            &self.obj
        }
    }
    impl IntoWasmAbi for Document {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Document {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Document {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Document {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Document {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Document {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Document {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Document {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Document>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Document {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Document {
        #[inline]
        fn from(obj: JsValue) -> Document {
            Document { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Document {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Document> for Document {
        #[inline]
        fn as_ref(&self) -> &Document {
            self
        }
    }
    impl From<Document> for JsValue {
        #[inline]
        fn from(obj: Document) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Document {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Document(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Document(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Document(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Document { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Document) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Document> for Node {
    #[inline]
    fn from(obj: Document) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for Document {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Document> for EventTarget {
    #[inline]
    fn from(obj: Document) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for Document {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Document> for ::js_sys::Object {
    #[inline]
    fn from(obj: Document) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Document {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <Document as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `new Document(..)` constructor, creating a new instance of `Document`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/Document)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<Document, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Document() -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Document() -> <Document as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_Document() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Document as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_adopt_node_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `adoptNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptNode)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn adopt_node(&self, node: &Node) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_adopt_node_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_adopt_node_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_adopt_node_Document(self_, node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CaretPosition", feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_caret_position_from_point_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <Option<CaretPosition> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "CaretPosition", feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `caretPositionFromPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/caretPositionFromPoint)\n\n*This API requires the following crate features to be activated: `CaretPosition`, `Document`*"]
    #[allow(clippy::all)]
    pub fn caret_position_from_point(&self, x: f32, y: f32) -> Option<CaretPosition> {
        #[cfg(all(feature = "CaretPosition", feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_caret_position_from_point_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CaretPosition> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_caret_position_from_point_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CaretPosition> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_caret_position_from_point_Document(self_, x, y)
            };
            <Option<CaretPosition> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Attr", feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_attribute_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Attr as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Attr", feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `createAttribute()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttribute)\n\n*This API requires the following crate features to be activated: `Attr`, `Document`*"]
    #[allow(clippy::all)]
    pub fn create_attribute(&self, name: &str) -> Result<Attr, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Attr", feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_attribute_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Attr as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_attribute_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Attr as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_create_attribute_Document(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Attr as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Attr", feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_attribute_ns_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Attr as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Attr", feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `createAttributeNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttributeNS)\n\n*This API requires the following crate features to be activated: `Attr`, `Document`*"]
    #[allow(clippy::all)]
    pub fn create_attribute_ns(
        &self,
        namespace: Option<&str>,
        name: &str,
    ) -> Result<Attr, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Attr", feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_attribute_ns_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Attr as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_attribute_ns_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Attr as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(namespace);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_create_attribute_ns_Document(self_, namespace, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Attr as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CdataSection", feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_cdata_section_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <CdataSection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "CdataSection", feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `createCDATASection()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createCDATASection)\n\n*This API requires the following crate features to be activated: `CdataSection`, `Document`*"]
    #[allow(clippy::all)]
    pub fn create_cdata_section(
        &self,
        data: &str,
    ) -> Result<CdataSection, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CdataSection", feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_cdata_section_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CdataSection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_cdata_section_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CdataSection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_create_cdata_section_Document(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<CdataSection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Comment", feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_comment_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Comment as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Comment", feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `createComment()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createComment)\n\n*This API requires the following crate features to be activated: `Comment`, `Document`*"]
    #[allow(clippy::all)]
    pub fn create_comment(&self, data: &str) -> Comment {
        #[cfg(all(feature = "Comment", feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_comment_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Comment as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_comment_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Comment as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_create_comment_Document(self_, data)
            };
            <Comment as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "DocumentFragment",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_document_fragment_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <DocumentFragment as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "DocumentFragment",))]
    #[allow(bad_style)]
    #[doc = "The `createDocumentFragment()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createDocumentFragment)\n\n*This API requires the following crate features to be activated: `Document`, `DocumentFragment`*"]
    #[allow(clippy::all)]
    pub fn create_document_fragment(&self) -> DocumentFragment {
        #[cfg(all(feature = "Document", feature = "DocumentFragment",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_document_fragment_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_document_fragment_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_document_fragment_Document(self_)
            };
            <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_element_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Element as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `createElement()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn create_element(&self, local_name: &str) -> Result<Element, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_element_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_element_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(local_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_create_element_Document(self_, local_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Element as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "Element",
    feature = "ElementCreationOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_element_with_element_creation_options_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&ElementCreationOptions as WasmDescribe>::describe();
    <Element as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "Element",
        feature = "ElementCreationOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createElement()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)\n\n*This API requires the following crate features to be activated: `Document`, `Element`, `ElementCreationOptions`*"]
    #[allow(clippy::all)]
    pub fn create_element_with_element_creation_options(
        &self,
        local_name: &str,
        options: &ElementCreationOptions,
    ) -> Result<Element, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "Element",
            feature = "ElementCreationOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_element_with_element_creation_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ElementCreationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_element_with_element_creation_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ElementCreationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(local_name);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                let options =
                    <&ElementCreationOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_create_element_with_element_creation_options_Document(
                    self_, local_name, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Element as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_element_with_str_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Element as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `createElement()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn create_element_with_str(
        &self,
        local_name: &str,
        options: &str,
    ) -> Result<Element, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_element_with_str_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_element_with_str_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(local_name);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                let options = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_create_element_with_str_Document(self_, local_name, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Element as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_element_ns_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Element as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `createElementNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn create_element_ns(
        &self,
        namespace: Option<&str>,
        qualified_name: &str,
    ) -> Result<Element, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_element_ns_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_element_ns_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let qualified_name =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(qualified_name);
                __widl_f_create_element_ns_Document(self_, namespace, qualified_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Element as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "Element",
    feature = "ElementCreationOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_element_ns_with_element_creation_options_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&ElementCreationOptions as WasmDescribe>::describe();
    <Element as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "Element",
        feature = "ElementCreationOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createElementNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)\n\n*This API requires the following crate features to be activated: `Document`, `Element`, `ElementCreationOptions`*"]
    #[allow(clippy::all)]
    pub fn create_element_ns_with_element_creation_options(
        &self,
        namespace: Option<&str>,
        qualified_name: &str,
        options: &ElementCreationOptions,
    ) -> Result<Element, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "Element",
            feature = "ElementCreationOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_element_ns_with_element_creation_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ElementCreationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_element_ns_with_element_creation_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ElementCreationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(namespace);
            drop(qualified_name);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let qualified_name =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(qualified_name);
                let options =
                    <&ElementCreationOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_create_element_ns_with_element_creation_options_Document(
                    self_,
                    namespace,
                    qualified_name,
                    options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Element as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_element_ns_with_str_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Element as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `createElementNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn create_element_ns_with_str(
        &self,
        namespace: Option<&str>,
        qualified_name: &str,
        options: &str,
    ) -> Result<Element, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_element_ns_with_str_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_element_ns_with_str_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            qualified_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(namespace);
            drop(qualified_name);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let qualified_name =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(qualified_name);
                let options = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_create_element_ns_with_str_Document(
                    self_,
                    namespace,
                    qualified_name,
                    options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Element as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_event_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Event as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `createEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createEvent)\n\n*This API requires the following crate features to be activated: `Document`, `Event`*"]
    #[allow(clippy::all)]
    pub fn create_event(&self, interface: &str) -> Result<Event, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_event_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                interface: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Event as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_event_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            interface: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Event as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(interface);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let interface = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(interface);
                __widl_f_create_event_Document(self_, interface)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Event as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node", feature = "NodeIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_node_iterator_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <NodeIterator as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node", feature = "NodeIterator",))]
    #[allow(bad_style)]
    #[doc = "The `createNodeIterator()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `NodeIterator`*"]
    #[allow(clippy::all)]
    pub fn create_node_iterator(
        &self,
        root: &Node,
    ) -> Result<NodeIterator, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node", feature = "NodeIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_node_iterator_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeIterator as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_node_iterator_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NodeIterator as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(root);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let root = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(root);
                __widl_f_create_node_iterator_Document(self_, root)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<NodeIterator as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node", feature = "NodeIterator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_node_iterator_with_what_to_show_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <NodeIterator as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node", feature = "NodeIterator",))]
    #[allow(bad_style)]
    #[doc = "The `createNodeIterator()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `NodeIterator`*"]
    #[allow(clippy::all)]
    pub fn create_node_iterator_with_what_to_show(
        &self,
        root: &Node,
        what_to_show: u32,
    ) -> Result<NodeIterator, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node", feature = "NodeIterator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_node_iterator_with_what_to_show_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                what_to_show: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeIterator as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_node_iterator_with_what_to_show_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            what_to_show: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NodeIterator as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(root);
            drop(what_to_show);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let root = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(root);
                let what_to_show =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(what_to_show);
                __widl_f_create_node_iterator_with_what_to_show_Document(self_, root, what_to_show)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<NodeIterator as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "Node",
    feature = "NodeFilter",
    feature = "NodeIterator",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_node_iterator_with_what_to_show_and_filter_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&NodeFilter> as WasmDescribe>::describe();
    <NodeIterator as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "Node",
        feature = "NodeFilter",
        feature = "NodeIterator",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createNodeIterator()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `NodeFilter`, `NodeIterator`*"]
    #[allow(clippy::all)]
    pub fn create_node_iterator_with_what_to_show_and_filter(
        &self,
        root: &Node,
        what_to_show: u32,
        filter: Option<&NodeFilter>,
    ) -> Result<NodeIterator, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "Node",
            feature = "NodeFilter",
            feature = "NodeIterator",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_node_iterator_with_what_to_show_and_filter_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                what_to_show: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                filter: <Option<&NodeFilter> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeIterator as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_node_iterator_with_what_to_show_and_filter_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            what_to_show: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            filter: <Option<&NodeFilter> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NodeIterator as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(root);
            drop(what_to_show);
            drop(filter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let root = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(root);
                let what_to_show =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(what_to_show);
                let filter =
                    <Option<&NodeFilter> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(filter);
                __widl_f_create_node_iterator_with_what_to_show_and_filter_Document(
                    self_,
                    root,
                    what_to_show,
                    filter,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<NodeIterator as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "ProcessingInstruction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_processing_instruction_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <ProcessingInstruction as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "ProcessingInstruction",))]
    #[allow(bad_style)]
    #[doc = "The `createProcessingInstruction()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createProcessingInstruction)\n\n*This API requires the following crate features to be activated: `Document`, `ProcessingInstruction`*"]
    #[allow(clippy::all)]
    pub fn create_processing_instruction(
        &self,
        target: &str,
        data: &str,
    ) -> Result<ProcessingInstruction, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "ProcessingInstruction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_processing_instruction_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ProcessingInstruction as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_processing_instruction_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ProcessingInstruction as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(target);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_create_processing_instruction_Document(self_, target, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ProcessingInstruction as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Range",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_range_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Range as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Range",))]
    #[allow(bad_style)]
    #[doc = "The `createRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createRange)\n\n*This API requires the following crate features to be activated: `Document`, `Range`*"]
    #[allow(clippy::all)]
    pub fn create_range(&self) -> Result<Range, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Range",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_range_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Range as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_range_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Range as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_range_Document(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Range as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_text_node_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Text as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `createTextNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)\n\n*This API requires the following crate features to be activated: `Document`, `Text`*"]
    #[allow(clippy::all)]
    pub fn create_text_node(&self, data: &str) -> Text {
        #[cfg(all(feature = "Document", feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_text_node_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Text as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_text_node_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Text as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_create_text_node_Document(self_, data)
            };
            <Text as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_tree_walker_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <TreeWalker as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `createTreeWalker()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn create_tree_walker(&self, root: &Node) -> Result<TreeWalker, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_tree_walker_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TreeWalker as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_tree_walker_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TreeWalker as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(root);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let root = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(root);
                __widl_f_create_tree_walker_Document(self_, root)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TreeWalker as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node", feature = "TreeWalker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_tree_walker_with_what_to_show_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <TreeWalker as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node", feature = "TreeWalker",))]
    #[allow(bad_style)]
    #[doc = "The `createTreeWalker()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn create_tree_walker_with_what_to_show(
        &self,
        root: &Node,
        what_to_show: u32,
    ) -> Result<TreeWalker, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node", feature = "TreeWalker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_tree_walker_with_what_to_show_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                what_to_show: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TreeWalker as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_tree_walker_with_what_to_show_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            what_to_show: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TreeWalker as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(root);
            drop(what_to_show);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let root = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(root);
                let what_to_show =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(what_to_show);
                __widl_f_create_tree_walker_with_what_to_show_Document(self_, root, what_to_show)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TreeWalker as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "Node",
    feature = "NodeFilter",
    feature = "TreeWalker",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_tree_walker_with_what_to_show_and_filter_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&NodeFilter> as WasmDescribe>::describe();
    <TreeWalker as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "Node",
        feature = "NodeFilter",
        feature = "TreeWalker",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createTreeWalker()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `NodeFilter`, `TreeWalker`*"]
    #[allow(clippy::all)]
    pub fn create_tree_walker_with_what_to_show_and_filter(
        &self,
        root: &Node,
        what_to_show: u32,
        filter: Option<&NodeFilter>,
    ) -> Result<TreeWalker, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "Node",
            feature = "NodeFilter",
            feature = "TreeWalker",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_tree_walker_with_what_to_show_and_filter_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                what_to_show: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                filter: <Option<&NodeFilter> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TreeWalker as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_tree_walker_with_what_to_show_and_filter_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            root: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            what_to_show: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            filter: <Option<&NodeFilter> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TreeWalker as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(root);
            drop(what_to_show);
            drop(filter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let root = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(root);
                let what_to_show =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(what_to_show);
                let filter =
                    <Option<&NodeFilter> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(filter);
                __widl_f_create_tree_walker_with_what_to_show_and_filter_Document(
                    self_,
                    root,
                    what_to_show,
                    filter,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TreeWalker as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_enable_style_sheets_for_set_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `enableStyleSheetsForSet()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/enableStyleSheetsForSet)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn enable_style_sheets_for_set(&self, name: Option<&str>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_enable_style_sheets_for_set_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_enable_style_sheets_for_set_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_enable_style_sheets_for_set_Document(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_exit_fullscreen_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `exitFullscreen()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitFullscreen)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn exit_fullscreen(&self) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exit_fullscreen_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exit_fullscreen_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_exit_fullscreen_Document(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_exit_pointer_lock_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `exitPointerLock()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitPointerLock)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn exit_pointer_lock(&self) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exit_pointer_lock_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exit_pointer_lock_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_exit_pointer_lock_Document(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_animations_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `getAnimations()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getAnimations)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn get_animations(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_animations_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_animations_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_animations_Document(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_element_by_id_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `getElementById()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementById)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn get_element_by_id(&self, element_id: &str) -> Option<Element> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_element_by_id_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_element_by_id_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element_id);
                __widl_f_get_element_by_id_Document(self_, element_id)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_elements_by_class_name_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `getElementsByClassName()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByClassName)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn get_elements_by_class_name(&self, class_names: &str) -> HtmlCollection {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_elements_by_class_name_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                class_names: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_elements_by_class_name_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            class_names: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(class_names);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let class_names =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(class_names);
                __widl_f_get_elements_by_class_name_Document(self_, class_names)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_elements_by_name_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `getElementsByName()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByName)\n\n*This API requires the following crate features to be activated: `Document`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn get_elements_by_name(&self, element_name: &str) -> NodeList {
        #[cfg(all(feature = "Document", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_elements_by_name_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_elements_by_name_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(element_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element_name =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element_name);
                __widl_f_get_elements_by_name_Document(self_, element_name)
            };
            <NodeList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_elements_by_tag_name_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `getElementsByTagName()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagName)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn get_elements_by_tag_name(&self, local_name: &str) -> HtmlCollection {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_elements_by_tag_name_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_elements_by_tag_name_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(local_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_get_elements_by_tag_name_Document(self_, local_name)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_elements_by_tag_name_ns_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `getElementsByTagNameNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagNameNS)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn get_elements_by_tag_name_ns(
        &self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<HtmlCollection, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_elements_by_tag_name_ns_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_elements_by_tag_name_ns_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(namespace);
            drop(local_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_get_elements_by_tag_name_ns_Document(self_, namespace, local_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Selection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_selection_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<Selection> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Selection",))]
    #[allow(bad_style)]
    #[doc = "The `getSelection()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getSelection)\n\n*This API requires the following crate features to be activated: `Document`, `Selection`*"]
    #[allow(clippy::all)]
    pub fn get_selection(&self) -> Result<Option<Selection>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Selection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_selection_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Selection> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_selection_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Selection> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_selection_Document(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Selection> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_focus_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `hasFocus()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/hasFocus)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn has_focus(&self) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_focus_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_focus_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_has_focus_Document(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_node_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `importNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/importNode)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn import_node(&self, node: &Node) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_node_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_node_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_import_node_Document(self_, node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_node_with_deep_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `importNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/importNode)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn import_node_with_deep(
        &self,
        node: &Node,
        deep: bool,
    ) -> Result<Node, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_node_with_deep_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                deep: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_node_with_deep_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            deep: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node);
            drop(deep);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                let deep = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(deep);
                __widl_f_import_node_with_deep_Document(self_, node, deep)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_selector_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `querySelector()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn query_selector(
        &self,
        selectors: &str,
    ) -> Result<Option<Element>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_selector_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selectors: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_selector_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selectors = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selectors);
                __widl_f_query_selector_Document(self_, selectors)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_selector_all_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `querySelectorAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelectorAll)\n\n*This API requires the following crate features to be activated: `Document`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn query_selector_all(&self, selectors: &str) -> Result<NodeList, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_selector_all_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selectors: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_selector_all_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selectors = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selectors);
                __widl_f_query_selector_all_Document(self_, selectors)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<NodeList as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_release_capture_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `releaseCapture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/releaseCapture)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn release_capture(&self) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_release_capture_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_release_capture_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_release_capture_Document(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document", feature = "DomImplementation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_implementation_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <DomImplementation as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "DomImplementation",))]
    #[allow(bad_style)]
    #[doc = "The `implementation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/implementation)\n\n*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
    #[allow(clippy::all)]
    pub fn implementation(&self) -> Result<DomImplementation, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomImplementation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_implementation_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomImplementation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_implementation_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomImplementation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_implementation_Document(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomImplementation as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_url_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `URL` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/URL)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn url(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_url_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_url_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_url_Document(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_document_uri_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `documentURI` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentURI)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn document_uri(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_document_uri_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_document_uri_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_document_uri_Document(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_compat_mode_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `compatMode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/compatMode)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn compat_mode(&self) -> String {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_compat_mode_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_compat_mode_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_compat_mode_Document(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_character_set_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `characterSet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/characterSet)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn character_set(&self) -> String {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_character_set_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_character_set_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_character_set_Document(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_charset_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `charset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/charset)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn charset(&self) -> String {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_charset_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_charset_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_charset_Document(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_input_encoding_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `inputEncoding` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/inputEncoding)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn input_encoding(&self) -> String {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_input_encoding_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_input_encoding_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_input_encoding_Document(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_content_type_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `contentType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/contentType)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn content_type(&self) -> String {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_content_type_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_content_type_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_content_type_Document(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "DocumentType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_doctype_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<DocumentType> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "DocumentType",))]
    #[allow(bad_style)]
    #[doc = "The `doctype` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/doctype)\n\n*This API requires the following crate features to be activated: `Document`, `DocumentType`*"]
    #[allow(clippy::all)]
    pub fn doctype(&self) -> Option<DocumentType> {
        #[cfg(all(feature = "Document", feature = "DocumentType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_doctype_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DocumentType> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_doctype_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DocumentType> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_doctype_Document(self_)
            };
            <Option<DocumentType> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_document_element_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `documentElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentElement)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn document_element(&self) -> Option<Element> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_document_element_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_document_element_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_document_element_Document(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Location",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_location_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<Location> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Location",))]
    #[allow(bad_style)]
    #[doc = "The `location` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/location)\n\n*This API requires the following crate features to be activated: `Document`, `Location`*"]
    #[allow(clippy::all)]
    pub fn location(&self) -> Option<Location> {
        #[cfg(all(feature = "Document", feature = "Location",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_location_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Location> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_location_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Location> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_location_Document(self_)
            };
            <Option<Location> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_referrer_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `referrer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/referrer)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn referrer(&self) -> String {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_referrer_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_referrer_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_referrer_Document(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_last_modified_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `lastModified` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastModified)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn last_modified(&self) -> String {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_last_modified_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_last_modified_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_last_modified_Document(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/readyState)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> String {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_Document(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_title_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `title` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn title(&self) -> String {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_title_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_title_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_title_Document(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_title_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `title` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_title(&self, title: &str) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_title_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_title_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(title);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                __widl_f_set_title_Document(self_, title)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dir_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `dir` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/dir)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn dir(&self) -> String {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dir_Document(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_dir_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `dir` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/dir)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_dir(&self, dir: &str) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_dir_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dir: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_dir_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dir: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(dir);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let dir = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dir);
                __widl_f_set_dir_Document(self_, dir)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_body_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<HtmlElement> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `body` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn body(&self) -> Option<HtmlElement> {
        #[cfg(all(feature = "Document", feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_body_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_body_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<HtmlElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_body_Document(self_)
            };
            <Option<HtmlElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_body_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&HtmlElement> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `body` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_body(&self, body: Option<&HtmlElement>) {
        #[cfg(all(feature = "Document", feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_body_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                body: <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_body_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            body: <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(body);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let body =
                    <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(body);
                __widl_f_set_body_Document(self_, body)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlHeadElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_head_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<HtmlHeadElement> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlHeadElement",))]
    #[allow(bad_style)]
    #[doc = "The `head` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/head)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlHeadElement`*"]
    #[allow(clippy::all)]
    pub fn head(&self) -> Option<HtmlHeadElement> {
        #[cfg(all(feature = "Document", feature = "HtmlHeadElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_head_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlHeadElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_head_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<HtmlHeadElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_head_Document(self_)
            };
            <Option<HtmlHeadElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_images_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `images` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/images)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn images(&self) -> HtmlCollection {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_images_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_images_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_images_Document(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_embeds_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `embeds` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/embeds)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn embeds(&self) -> HtmlCollection {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_embeds_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_embeds_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_embeds_Document(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_plugins_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `plugins` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/plugins)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn plugins(&self) -> HtmlCollection {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_plugins_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_plugins_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_plugins_Document(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_links_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `links` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/links)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn links(&self) -> HtmlCollection {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_links_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_links_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_links_Document(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_forms_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `forms` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/forms)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn forms(&self) -> HtmlCollection {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_forms_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_forms_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_forms_Document(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scripts_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `scripts` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/scripts)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn scripts(&self) -> HtmlCollection {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scripts_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scripts_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scripts_Document(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_view_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<Window> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `defaultView` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/defaultView)\n\n*This API requires the following crate features to be activated: `Document`, `Window`*"]
    #[allow(clippy::all)]
    pub fn default_view(&self) -> Option<Window> {
        #[cfg(all(feature = "Document", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_view_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_view_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_view_Document(self_)
            };
            <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onreadystatechange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onreadystatechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreadystatechange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onreadystatechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onreadystatechange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onreadystatechange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onreadystatechange_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onreadystatechange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onreadystatechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreadystatechange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onreadystatechange(&self, onreadystatechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onreadystatechange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onreadystatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onreadystatechange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onreadystatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onreadystatechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onreadystatechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onreadystatechange,
                    );
                __widl_f_set_onreadystatechange_Document(self_, onreadystatechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onbeforescriptexecute_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onbeforescriptexecute` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforescriptexecute)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onbeforescriptexecute(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onbeforescriptexecute_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onbeforescriptexecute_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onbeforescriptexecute_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onbeforescriptexecute_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onbeforescriptexecute` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforescriptexecute)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onbeforescriptexecute(&self, onbeforescriptexecute: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onbeforescriptexecute_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onbeforescriptexecute : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onbeforescriptexecute_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onbeforescriptexecute : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onbeforescriptexecute);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onbeforescriptexecute =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onbeforescriptexecute,
                    );
                __widl_f_set_onbeforescriptexecute_Document(self_, onbeforescriptexecute)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onafterscriptexecute_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onafterscriptexecute` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onafterscriptexecute)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onafterscriptexecute(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onafterscriptexecute_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onafterscriptexecute_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onafterscriptexecute_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onafterscriptexecute_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onafterscriptexecute` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onafterscriptexecute)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onafterscriptexecute(&self, onafterscriptexecute: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onafterscriptexecute_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onafterscriptexecute : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onafterscriptexecute_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onafterscriptexecute : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onafterscriptexecute);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onafterscriptexecute =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onafterscriptexecute,
                    );
                __widl_f_set_onafterscriptexecute_Document(self_, onafterscriptexecute)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onselectionchange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onselectionchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectionchange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onselectionchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onselectionchange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onselectionchange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onselectionchange_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onselectionchange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onselectionchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectionchange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onselectionchange(&self, onselectionchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onselectionchange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onselectionchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onselectionchange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onselectionchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onselectionchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onselectionchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onselectionchange,
                    );
                __widl_f_set_onselectionchange_Document(self_, onselectionchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_script_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `currentScript` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/currentScript)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn current_script(&self) -> Option<Element> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_script_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_script_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_script_Document(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_anchors_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `anchors` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/anchors)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn anchors(&self) -> HtmlCollection {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_anchors_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_anchors_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_anchors_Document(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_applets_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `applets` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/applets)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn applets(&self) -> HtmlCollection {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_applets_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_applets_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_applets_Document(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fullscreen_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `fullscreen` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreen)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn fullscreen(&self) -> bool {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fullscreen_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fullscreen_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_fullscreen_Document(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fullscreen_enabled_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `fullscreenEnabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreenEnabled)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn fullscreen_enabled(&self) -> bool {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fullscreen_enabled_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fullscreen_enabled_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_fullscreen_enabled_Document(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onfullscreenchange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onfullscreenchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenchange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onfullscreenchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onfullscreenchange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onfullscreenchange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onfullscreenchange_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onfullscreenchange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onfullscreenchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenchange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onfullscreenchange(&self, onfullscreenchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onfullscreenchange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onfullscreenchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onfullscreenchange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onfullscreenchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onfullscreenchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onfullscreenchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onfullscreenchange,
                    );
                __widl_f_set_onfullscreenchange_Document(self_, onfullscreenchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onfullscreenerror_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onfullscreenerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenerror)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onfullscreenerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onfullscreenerror_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onfullscreenerror_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onfullscreenerror_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onfullscreenerror_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onfullscreenerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenerror)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onfullscreenerror(&self, onfullscreenerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onfullscreenerror_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onfullscreenerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onfullscreenerror_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onfullscreenerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onfullscreenerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onfullscreenerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onfullscreenerror,
                    );
                __widl_f_set_onfullscreenerror_Document(self_, onfullscreenerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerlockchange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerlockchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockchange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpointerlockchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerlockchange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerlockchange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerlockchange_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerlockchange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerlockchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockchange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerlockchange(&self, onpointerlockchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerlockchange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerlockchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerlockchange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerlockchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpointerlockchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerlockchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerlockchange,
                    );
                __widl_f_set_onpointerlockchange_Document(self_, onpointerlockchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerlockerror_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerlockerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockerror)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpointerlockerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerlockerror_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerlockerror_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerlockerror_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerlockerror_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerlockerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockerror)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerlockerror(&self, onpointerlockerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerlockerror_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerlockerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerlockerror_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerlockerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpointerlockerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerlockerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerlockerror,
                    );
                __widl_f_set_onpointerlockerror_Document(self_, onpointerlockerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hidden_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `hidden` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/hidden)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn hidden(&self) -> bool {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hidden_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hidden_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hidden_Document(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "VisibilityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_visibility_state_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <VisibilityState as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "VisibilityState",))]
    #[allow(bad_style)]
    #[doc = "The `visibilityState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/visibilityState)\n\n*This API requires the following crate features to be activated: `Document`, `VisibilityState`*"]
    #[allow(clippy::all)]
    pub fn visibility_state(&self) -> VisibilityState {
        #[cfg(all(feature = "Document", feature = "VisibilityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_visibility_state_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VisibilityState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_visibility_state_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VisibilityState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_visibility_state_Document(self_)
            };
            <VisibilityState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onvisibilitychange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onvisibilitychange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvisibilitychange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onvisibilitychange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onvisibilitychange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onvisibilitychange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onvisibilitychange_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onvisibilitychange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onvisibilitychange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvisibilitychange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onvisibilitychange(&self, onvisibilitychange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onvisibilitychange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onvisibilitychange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onvisibilitychange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onvisibilitychange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onvisibilitychange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onvisibilitychange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onvisibilitychange,
                    );
                __widl_f_set_onvisibilitychange_Document(self_, onvisibilitychange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selected_style_sheet_set_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `selectedStyleSheetSet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/selectedStyleSheetSet)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn selected_style_sheet_set(&self) -> Option<String> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selected_style_sheet_set_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selected_style_sheet_set_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selected_style_sheet_set_Document(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selected_style_sheet_set_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `selectedStyleSheetSet` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/selectedStyleSheetSet)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_selected_style_sheet_set(&self, selected_style_sheet_set: Option<&str>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selected_style_sheet_set_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selected_style_sheet_set: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selected_style_sheet_set_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selected_style_sheet_set: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(selected_style_sheet_set);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selected_style_sheet_set =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        selected_style_sheet_set,
                    );
                __widl_f_set_selected_style_sheet_set_Document(self_, selected_style_sheet_set)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_last_style_sheet_set_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `lastStyleSheetSet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastStyleSheetSet)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn last_style_sheet_set(&self) -> Option<String> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_last_style_sheet_set_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_last_style_sheet_set_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_last_style_sheet_set_Document(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_preferred_style_sheet_set_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `preferredStyleSheetSet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/preferredStyleSheetSet)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn preferred_style_sheet_set(&self) -> Option<String> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_preferred_style_sheet_set_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_preferred_style_sheet_set_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_preferred_style_sheet_set_Document(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "DomStringList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_style_sheet_sets_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <DomStringList as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "DomStringList",))]
    #[allow(bad_style)]
    #[doc = "The `styleSheetSets` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/styleSheetSets)\n\n*This API requires the following crate features to be activated: `Document`, `DomStringList`*"]
    #[allow(clippy::all)]
    pub fn style_sheet_sets(&self) -> DomStringList {
        #[cfg(all(feature = "Document", feature = "DomStringList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_style_sheet_sets_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_style_sheet_sets_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomStringList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_style_sheet_sets_Document(self_)
            };
            <DomStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scrolling_element_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollingElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/scrollingElement)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn scrolling_element(&self) -> Option<Element> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scrolling_element_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scrolling_element_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scrolling_element_Document(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "DocumentTimeline",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_timeline_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <DocumentTimeline as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "DocumentTimeline",))]
    #[allow(bad_style)]
    #[doc = "The `timeline` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/timeline)\n\n*This API requires the following crate features to be activated: `Document`, `DocumentTimeline`*"]
    #[allow(clippy::all)]
    pub fn timeline(&self) -> DocumentTimeline {
        #[cfg(all(feature = "Document", feature = "DocumentTimeline",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_timeline_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DocumentTimeline as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_timeline_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DocumentTimeline as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_timeline_Document(self_)
            };
            <DocumentTimeline as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "SvgsvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_root_element_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<SvgsvgElement> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "SvgsvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `rootElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/rootElement)\n\n*This API requires the following crate features to be activated: `Document`, `SvgsvgElement`*"]
    #[allow(clippy::all)]
    pub fn root_element(&self) -> Option<SvgsvgElement> {
        #[cfg(all(feature = "Document", feature = "SvgsvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_root_element_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SvgsvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_root_element_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SvgsvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_root_element_Document(self_)
            };
            <Option<SvgsvgElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncopy_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oncopy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncopy)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn oncopy(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncopy_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncopy_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncopy_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncopy_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oncopy` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncopy)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_oncopy(&self, oncopy: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncopy_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncopy: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncopy_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncopy: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncopy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncopy =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncopy,
                    );
                __widl_f_set_oncopy_Document(self_, oncopy)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncut_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oncut` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncut)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn oncut(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncut_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncut_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncut_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncut_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oncut` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncut)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_oncut(&self, oncut: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncut_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncut: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncut_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncut: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncut);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncut =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncut,
                    );
                __widl_f_set_oncut_Document(self_, oncut)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpaste_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpaste` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpaste)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpaste(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpaste_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpaste_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpaste_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpaste_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpaste` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpaste)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpaste(&self, onpaste: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpaste_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpaste: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpaste_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpaste: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpaste);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpaste =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpaste,
                    );
                __widl_f_set_onpaste_Document(self_, onpaste)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_element_from_point_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `elementFromPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/elementFromPoint)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn element_from_point(&self, x: f32, y: f32) -> Option<Element> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_element_from_point_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_element_from_point_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_element_from_point_Document(self_, x, y)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_elements_from_point_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `elementsFromPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/elementsFromPoint)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn elements_from_point(&self, x: f32, y: f32) -> ::js_sys::Array {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_elements_from_point_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_elements_from_point_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_elements_from_point_Document(self_, x, y)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_active_element_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `activeElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/activeElement)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn active_element(&self) -> Option<Element> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_active_element_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_active_element_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_active_element_Document(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "StyleSheetList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_style_sheets_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <StyleSheetList as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "StyleSheetList",))]
    #[allow(bad_style)]
    #[doc = "The `styleSheets` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/styleSheets)\n\n*This API requires the following crate features to be activated: `Document`, `StyleSheetList`*"]
    #[allow(clippy::all)]
    pub fn style_sheets(&self) -> StyleSheetList {
        #[cfg(all(feature = "Document", feature = "StyleSheetList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_style_sheets_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <StyleSheetList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_style_sheets_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <StyleSheetList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_style_sheets_Document(self_)
            };
            <StyleSheetList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pointer_lock_element_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `pointerLockElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/pointerLockElement)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn pointer_lock_element(&self) -> Option<Element> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pointer_lock_element_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pointer_lock_element_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pointer_lock_element_Document(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fullscreen_element_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `fullscreenElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreenElement)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn fullscreen_element(&self) -> Option<Element> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fullscreen_element_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fullscreen_element_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_fullscreen_element_Document(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "FontFaceSet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fonts_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <FontFaceSet as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "FontFaceSet",))]
    #[allow(bad_style)]
    #[doc = "The `fonts` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fonts)\n\n*This API requires the following crate features to be activated: `Document`, `FontFaceSet`*"]
    #[allow(clippy::all)]
    pub fn fonts(&self) -> FontFaceSet {
        #[cfg(all(feature = "Document", feature = "FontFaceSet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fonts_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFaceSet as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fonts_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFaceSet as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_fonts_Document(self_)
            };
            <FontFaceSet as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_text_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_text(
        &self,
        point: &DomPointInit,
        from: &Text,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_text_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_text_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_point_from_node_with_text_Document(self_, point, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Element",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_element_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_element(
        &self,
        point: &DomPointInit,
        from: &Element,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Element",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_element_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_element_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_point_from_node_with_element_Document(self_, point, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "DomPoint", feature = "DomPointInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_document_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "DomPoint", feature = "DomPointInit",))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_document(
        &self,
        point: &DomPointInit,
        from: &Document,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomPoint", feature = "DomPointInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_document_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_document_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_point_from_node_with_document_Document(self_, point, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_text_and_options_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_text_and_options(
        &self,
        point: &DomPointInit,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_text_and_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_text_and_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_point_from_node_with_text_and_options_Document(
                    self_, point, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Element",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_element_and_options_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_element_and_options(
        &self,
        point: &DomPointInit,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Element",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_element_and_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_element_and_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_point_from_node_with_element_and_options_Document(
                    self_, point, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomPoint",
    feature = "DomPointInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_document_and_options_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_document_and_options(
        &self,
        point: &DomPointInit,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomPoint",
            feature = "DomPointInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_document_and_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_document_and_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_point_from_node_with_document_and_options_Document(
                    self_, point, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "DomQuad", feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_text_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "DomQuad", feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_text(
        &self,
        quad: &DomQuad,
        from: &Text,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomQuad", feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_text_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_text_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_quad_from_node_with_text_Document(self_, quad, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "DomQuad", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_element_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "DomQuad", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_element(
        &self,
        quad: &DomQuad,
        from: &Element,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomQuad", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_element_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_element_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_quad_from_node_with_element_Document(self_, quad, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "DomQuad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_document_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "DomQuad",))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_document(
        &self,
        quad: &DomQuad,
        from: &Document,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomQuad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_document_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_document_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_quad_from_node_with_document_Document(self_, quad, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomQuad",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_text_and_options_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_text_and_options(
        &self,
        quad: &DomQuad,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomQuad",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_text_and_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_text_and_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_quad_from_node_with_text_and_options_Document(
                    self_, quad, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomQuad",
    feature = "Element",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_element_and_options_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_element_and_options(
        &self,
        quad: &DomQuad,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomQuad",
            feature = "Element",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_element_and_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_element_and_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_quad_from_node_with_element_and_options_Document(
                    self_, quad, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomQuad",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_document_and_options_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_document_and_options(
        &self,
        quad: &DomQuad,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomQuad",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_document_and_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_document_and_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(quad);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_quad_from_node_with_document_and_options_Document(
                    self_, quad, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_text_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_text(
        &self,
        rect: &DomRectReadOnly,
        from: &Text,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_text_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_text_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_rect_from_node_with_text_Document(self_, rect, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Element",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_element_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_element(
        &self,
        rect: &DomRectReadOnly,
        from: &Element,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Element",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_element_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_element_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_rect_from_node_with_element_Document(self_, rect, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document", feature = "DomQuad", feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_document_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_document(
        &self,
        rect: &DomRectReadOnly,
        from: &Document,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomQuad", feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_document_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_document_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_rect_from_node_with_document_Document(self_, rect, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_text_and_options_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_text_and_options(
        &self,
        rect: &DomRectReadOnly,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_text_and_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_text_and_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_rect_from_node_with_text_and_options_Document(
                    self_, rect, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Element",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_element_and_options_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_element_and_options(
        &self,
        rect: &DomRectReadOnly,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Element",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_element_and_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_element_and_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_rect_from_node_with_element_and_options_Document(
                    self_, rect, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "ConvertCoordinateOptions",
    feature = "Document",
    feature = "DomQuad",
    feature = "DomRectReadOnly",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_document_and_options_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_document_and_options(
        &self,
        rect: &DomRectReadOnly,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "Document",
            feature = "DomQuad",
            feature = "DomRectReadOnly",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_document_and_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_document_and_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(rect);
            drop(from);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_rect_from_node_with_document_and_options_Document(
                    self_, rect, from, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_box_quads_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `getBoxQuads()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getBoxQuads)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn get_box_quads(&self) -> Result<::js_sys::Array, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_box_quads_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_box_quads_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_box_quads_Document(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "BoxQuadOptions", feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_box_quads_with_options_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&BoxQuadOptions as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "BoxQuadOptions", feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `getBoxQuads()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getBoxQuads)\n\n*This API requires the following crate features to be activated: `BoxQuadOptions`, `Document`*"]
    #[allow(clippy::all)]
    pub fn get_box_quads_with_options(
        &self,
        options: &BoxQuadOptions,
    ) -> Result<::js_sys::Array, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BoxQuadOptions", feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_box_quads_with_options_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&BoxQuadOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_box_quads_with_options_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&BoxQuadOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&BoxQuadOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_get_box_quads_with_options_Document(self_, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onabort_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onabort)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onabort(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onabort_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onabort_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onabort_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onabort_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onabort)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onabort(&self, onabort: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onabort_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onabort_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onabort);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onabort =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onabort,
                    );
                __widl_f_set_onabort_Document(self_, onabort)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onblur_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onblur` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onblur)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onblur(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onblur_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onblur_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onblur_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onblur_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onblur` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onblur)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onblur(&self, onblur: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onblur_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onblur: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onblur_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onblur: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onblur);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onblur =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onblur,
                    );
                __widl_f_set_onblur_Document(self_, onblur)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onfocus_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onfocus` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfocus)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onfocus(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onfocus_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onfocus_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onfocus_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onfocus_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onfocus` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfocus)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onfocus(&self, onfocus: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onfocus_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onfocus: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onfocus_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onfocus: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onfocus);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onfocus =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onfocus,
                    );
                __widl_f_set_onfocus_Document(self_, onfocus)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onauxclick_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onauxclick` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onauxclick)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onauxclick(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onauxclick_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onauxclick_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onauxclick_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onauxclick_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onauxclick` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onauxclick)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onauxclick(&self, onauxclick: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onauxclick_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onauxclick : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onauxclick_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onauxclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onauxclick);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onauxclick =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onauxclick,
                    );
                __widl_f_set_onauxclick_Document(self_, onauxclick)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncanplay_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplay` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplay)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn oncanplay(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncanplay_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncanplay_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncanplay_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncanplay_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplay` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplay)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_oncanplay(&self, oncanplay: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncanplay_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncanplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncanplay_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncanplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncanplay);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncanplay =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncanplay,
                    );
                __widl_f_set_oncanplay_Document(self_, oncanplay)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncanplaythrough_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplaythrough` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn oncanplaythrough(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncanplaythrough_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncanplaythrough_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncanplaythrough_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncanplaythrough_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplaythrough` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_oncanplaythrough(&self, oncanplaythrough: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncanplaythrough_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncanplaythrough : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncanplaythrough_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncanplaythrough : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(oncanplaythrough);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncanplaythrough =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncanplaythrough,
                    );
                __widl_f_set_oncanplaythrough_Document(self_, oncanplaythrough)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onchange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onchange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onchange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onchange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onchange_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onchange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onchange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onchange(&self, onchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onchange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onchange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onchange,
                    );
                __widl_f_set_onchange_Document(self_, onchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclick_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onclick` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclick)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onclick(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclick_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclick_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclick_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclick_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onclick` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclick)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onclick(&self, onclick: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclick_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclick_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onclick);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclick =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclick,
                    );
                __widl_f_set_onclick_Document(self_, onclick)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclose_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclose)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onclose(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclose_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclose_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclose_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclose_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclose)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onclose(&self, onclose: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclose_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclose_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onclose);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclose =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclose,
                    );
                __widl_f_set_onclose_Document(self_, onclose)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncontextmenu_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oncontextmenu` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncontextmenu)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn oncontextmenu(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncontextmenu_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncontextmenu_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncontextmenu_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncontextmenu_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oncontextmenu` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncontextmenu)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_oncontextmenu(&self, oncontextmenu: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncontextmenu_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncontextmenu : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncontextmenu_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncontextmenu: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncontextmenu);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncontextmenu =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncontextmenu,
                    );
                __widl_f_set_oncontextmenu_Document(self_, oncontextmenu)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondblclick_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondblclick` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondblclick)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ondblclick(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondblclick_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondblclick_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondblclick_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondblclick_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondblclick` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondblclick)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ondblclick(&self, ondblclick: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondblclick_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondblclick : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondblclick_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondblclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondblclick);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondblclick =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondblclick,
                    );
                __widl_f_set_ondblclick_Document(self_, ondblclick)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondrag_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondrag` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrag)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ondrag(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondrag_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondrag_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondrag_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondrag_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondrag` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrag)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ondrag(&self, ondrag: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondrag_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondrag: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondrag_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondrag: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondrag);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondrag =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondrag,
                    );
                __widl_f_set_ondrag_Document(self_, ondrag)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ondragend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragend_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ondragend(&self, ondragend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragend,
                    );
                __widl_f_set_ondragend_Document(self_, ondragend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragenter_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragenter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragenter)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ondragenter(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragenter_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragenter_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragenter_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragenter_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragenter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragenter)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ondragenter(&self, ondragenter: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragenter_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragenter_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragenter: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragenter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragenter =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragenter,
                    );
                __widl_f_set_ondragenter_Document(self_, ondragenter)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragexit_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragexit` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragexit)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ondragexit(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragexit_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragexit_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragexit_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragexit_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragexit` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragexit)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ondragexit(&self, ondragexit: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragexit_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragexit : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragexit_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragexit: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragexit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragexit =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragexit,
                    );
                __widl_f_set_ondragexit_Document(self_, ondragexit)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragleave_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragleave` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragleave)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ondragleave(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragleave_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragleave_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragleave_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragleave_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragleave` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragleave)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ondragleave(&self, ondragleave: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragleave_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragleave_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragleave: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragleave);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragleave =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragleave,
                    );
                __widl_f_set_ondragleave_Document(self_, ondragleave)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragover_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragover` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragover)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ondragover(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragover_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragover_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragover_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragover_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragover` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragover)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ondragover(&self, ondragover: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragover_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragover : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragover_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragover: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragover);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragover =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragover,
                    );
                __widl_f_set_ondragover_Document(self_, ondragover)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ondragstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragstart_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondragstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ondragstart(&self, ondragstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragstart,
                    );
                __widl_f_set_ondragstart_Document(self_, ondragstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondrop_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondrop` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrop)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ondrop(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondrop_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondrop_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondrop_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondrop_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondrop` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrop)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ondrop(&self, ondrop: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondrop_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondrop: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondrop_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondrop: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondrop);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondrop =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondrop,
                    );
                __widl_f_set_ondrop_Document(self_, ondrop)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondurationchange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondurationchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondurationchange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ondurationchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondurationchange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondurationchange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondurationchange_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondurationchange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ondurationchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondurationchange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ondurationchange(&self, ondurationchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondurationchange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondurationchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondurationchange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondurationchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ondurationchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondurationchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondurationchange,
                    );
                __widl_f_set_ondurationchange_Document(self_, ondurationchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onemptied_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onemptied` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onemptied)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onemptied(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onemptied_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onemptied_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onemptied_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onemptied_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onemptied` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onemptied)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onemptied(&self, onemptied: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onemptied_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onemptied: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onemptied_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onemptied: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onemptied);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onemptied =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onemptied,
                    );
                __widl_f_set_onemptied_Document(self_, onemptied)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onended_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onended` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onended)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onended(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onended_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onended_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onended_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onended_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onended` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onended)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onended(&self, onended: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onended_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onended_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onended);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onended =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onended,
                    );
                __widl_f_set_onended_Document(self_, onended)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oninput_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oninput` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninput)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn oninput(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oninput_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oninput_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oninput_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oninput_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oninput` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninput)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_oninput(&self, oninput: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oninput_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oninput: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oninput_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oninput: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oninput);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oninput =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oninput,
                    );
                __widl_f_set_oninput_Document(self_, oninput)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oninvalid_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oninvalid` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninvalid)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn oninvalid(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oninvalid_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oninvalid_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oninvalid_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oninvalid_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `oninvalid` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninvalid)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_oninvalid(&self, oninvalid: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oninvalid_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oninvalid: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oninvalid_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oninvalid: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oninvalid);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oninvalid =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oninvalid,
                    );
                __widl_f_set_oninvalid_Document(self_, oninvalid)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onkeydown_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onkeydown` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeydown)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onkeydown(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onkeydown_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onkeydown_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onkeydown_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onkeydown_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onkeydown` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeydown)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onkeydown(&self, onkeydown: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onkeydown_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onkeydown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onkeydown_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onkeydown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onkeydown);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onkeydown =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onkeydown,
                    );
                __widl_f_set_onkeydown_Document(self_, onkeydown)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onkeypress_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onkeypress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeypress)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onkeypress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onkeypress_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onkeypress_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onkeypress_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onkeypress_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onkeypress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeypress)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onkeypress(&self, onkeypress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onkeypress_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onkeypress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onkeypress_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onkeypress: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onkeypress);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onkeypress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onkeypress,
                    );
                __widl_f_set_onkeypress_Document(self_, onkeypress)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onkeyup_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onkeyup` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeyup)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onkeyup(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onkeyup_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onkeyup_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onkeyup_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onkeyup_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onkeyup` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeyup)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onkeyup(&self, onkeyup: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onkeyup_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onkeyup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onkeyup_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onkeyup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onkeyup);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onkeyup =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onkeyup,
                    );
                __widl_f_set_onkeyup_Document(self_, onkeyup)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onload_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onload)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onload(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onload_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onload_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onload_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onload_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onload` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onload)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onload(&self, onload: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onload_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onload_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onload);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onload =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onload,
                    );
                __widl_f_set_onload_Document(self_, onload)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadeddata_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onloadeddata` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadeddata)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onloadeddata(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadeddata_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadeddata_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadeddata_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadeddata_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onloadeddata` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadeddata)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onloadeddata(&self, onloadeddata: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadeddata_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadeddata : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadeddata_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadeddata: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadeddata);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadeddata =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadeddata,
                    );
                __widl_f_set_onloadeddata_Document(self_, onloadeddata)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadedmetadata_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onloadedmetadata` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onloadedmetadata(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadedmetadata_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadedmetadata_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadedmetadata_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadedmetadata_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onloadedmetadata` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onloadedmetadata(&self, onloadedmetadata: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadedmetadata_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadedmetadata : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadedmetadata_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadedmetadata : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onloadedmetadata);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadedmetadata =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadedmetadata,
                    );
                __widl_f_set_onloadedmetadata_Document(self_, onloadedmetadata)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onloadend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onloadend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadend_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onloadend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onloadend(&self, onloadend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadend,
                    );
                __widl_f_set_onloadend_Document(self_, onloadend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onloadstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onloadstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadstart_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onloadstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onloadstart(&self, onloadstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadstart,
                    );
                __widl_f_set_onloadstart_Document(self_, onloadstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmousedown_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmousedown` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousedown)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onmousedown(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmousedown_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmousedown_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmousedown_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmousedown_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmousedown` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousedown)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onmousedown(&self, onmousedown: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmousedown_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmousedown : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmousedown_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmousedown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmousedown);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmousedown =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmousedown,
                    );
                __widl_f_set_onmousedown_Document(self_, onmousedown)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseenter_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseenter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseenter)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onmouseenter(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseenter_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseenter_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseenter_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseenter_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseenter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseenter)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseenter(&self, onmouseenter: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseenter_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseenter_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseenter: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseenter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseenter =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseenter,
                    );
                __widl_f_set_onmouseenter_Document(self_, onmouseenter)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseleave_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseleave` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseleave)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onmouseleave(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseleave_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseleave_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseleave_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseleave_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseleave` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseleave)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseleave(&self, onmouseleave: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseleave_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseleave_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseleave: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseleave);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseleave =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseleave,
                    );
                __widl_f_set_onmouseleave_Document(self_, onmouseleave)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmousemove_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmousemove` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousemove)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onmousemove(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmousemove_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmousemove_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmousemove_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmousemove_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmousemove` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousemove)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onmousemove(&self, onmousemove: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmousemove_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmousemove : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmousemove_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmousemove: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmousemove);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmousemove =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmousemove,
                    );
                __widl_f_set_onmousemove_Document(self_, onmousemove)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseout_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseout` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseout)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onmouseout(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseout_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseout_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseout_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseout_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseout` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseout)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseout(&self, onmouseout: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseout_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseout : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseout_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseout: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseout =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseout,
                    );
                __widl_f_set_onmouseout_Document(self_, onmouseout)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseover_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseover` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseover)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onmouseover(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseover_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseover_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseover_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseover_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseover` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseover)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseover(&self, onmouseover: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseover_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseover : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseover_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseover: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseover);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseover =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseover,
                    );
                __widl_f_set_onmouseover_Document(self_, onmouseover)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseup_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseup` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseup)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onmouseup(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseup_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseup_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseup_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseup_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseup` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseup)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseup(&self, onmouseup: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseup_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseup_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseup);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseup =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseup,
                    );
                __widl_f_set_onmouseup_Document(self_, onmouseup)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwheel_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwheel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwheel)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onwheel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwheel_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwheel_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwheel_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwheel_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwheel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwheel)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onwheel(&self, onwheel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwheel_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwheel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwheel_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwheel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onwheel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwheel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwheel,
                    );
                __widl_f_set_onwheel_Document(self_, onwheel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpause_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpause` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpause)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpause(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpause_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpause_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpause_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpause_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpause` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpause)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpause(&self, onpause: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpause_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpause: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpause_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpause: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpause);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpause =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpause,
                    );
                __widl_f_set_onpause_Document(self_, onpause)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onplay_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onplay` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplay)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onplay(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onplay_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onplay_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onplay_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onplay_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onplay` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplay)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onplay(&self, onplay: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onplay_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onplay_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onplay);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onplay =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onplay,
                    );
                __widl_f_set_onplay_Document(self_, onplay)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onplaying_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onplaying` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplaying)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onplaying(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onplaying_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onplaying_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onplaying_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onplaying_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onplaying` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplaying)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onplaying(&self, onplaying: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onplaying_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onplaying: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onplaying_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onplaying: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onplaying);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onplaying =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onplaying,
                    );
                __widl_f_set_onplaying_Document(self_, onplaying)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onprogress_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onprogress)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onprogress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onprogress_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onprogress_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onprogress_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onprogress_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onprogress)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onprogress(&self, onprogress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onprogress_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onprogress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onprogress_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onprogress: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onprogress);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onprogress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onprogress,
                    );
                __widl_f_set_onprogress_Document(self_, onprogress)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onratechange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onratechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onratechange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onratechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onratechange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onratechange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onratechange_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onratechange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onratechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onratechange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onratechange(&self, onratechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onratechange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onratechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onratechange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onratechange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onratechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onratechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onratechange,
                    );
                __widl_f_set_onratechange_Document(self_, onratechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onreset_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onreset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreset)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onreset(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onreset_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onreset_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onreset_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onreset_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onreset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreset)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onreset(&self, onreset: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onreset_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onreset: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onreset_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onreset: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onreset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onreset =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onreset,
                    );
                __widl_f_set_onreset_Document(self_, onreset)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onresize_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onresize` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onresize)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onresize(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onresize_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onresize_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onresize_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onresize_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onresize` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onresize)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onresize(&self, onresize: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onresize_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onresize: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onresize_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onresize: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onresize);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onresize =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onresize,
                    );
                __widl_f_set_onresize_Document(self_, onresize)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onscroll_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onscroll` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onscroll)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onscroll(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onscroll_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onscroll_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onscroll_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onscroll_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onscroll` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onscroll)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onscroll(&self, onscroll: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onscroll_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onscroll: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onscroll_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onscroll: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onscroll);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onscroll =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onscroll,
                    );
                __widl_f_set_onscroll_Document(self_, onscroll)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onseeked_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onseeked` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeked)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onseeked(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onseeked_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onseeked_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onseeked_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onseeked_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onseeked` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeked)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onseeked(&self, onseeked: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onseeked_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onseeked: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onseeked_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onseeked: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onseeked);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onseeked =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onseeked,
                    );
                __widl_f_set_onseeked_Document(self_, onseeked)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onseeking_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onseeking` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeking)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onseeking(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onseeking_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onseeking_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onseeking_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onseeking_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onseeking` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeking)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onseeking(&self, onseeking: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onseeking_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onseeking: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onseeking_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onseeking: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onseeking);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onseeking =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onseeking,
                    );
                __widl_f_set_onseeking_Document(self_, onseeking)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onselect_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onselect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselect)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onselect(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onselect_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onselect_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onselect_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onselect_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onselect` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselect)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onselect(&self, onselect: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onselect_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onselect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onselect_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onselect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onselect);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onselect =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onselect,
                    );
                __widl_f_set_onselect_Document(self_, onselect)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onshow_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onshow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onshow)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onshow(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onshow_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onshow_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onshow_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onshow_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onshow` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onshow)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onshow(&self, onshow: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onshow_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onshow: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onshow_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onshow: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onshow);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onshow =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onshow,
                    );
                __widl_f_set_onshow_Document(self_, onshow)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstalled_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onstalled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onstalled)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onstalled(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstalled_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstalled_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstalled_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstalled_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onstalled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onstalled)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onstalled(&self, onstalled: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstalled_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstalled: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstalled_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstalled: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstalled);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstalled =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstalled,
                    );
                __widl_f_set_onstalled_Document(self_, onstalled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsubmit_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onsubmit` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsubmit)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onsubmit(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsubmit_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsubmit_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsubmit_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsubmit_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onsubmit` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsubmit)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onsubmit(&self, onsubmit: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsubmit_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsubmit: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsubmit_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsubmit: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onsubmit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsubmit =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsubmit,
                    );
                __widl_f_set_onsubmit_Document(self_, onsubmit)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsuspend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onsuspend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsuspend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onsuspend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsuspend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsuspend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsuspend_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsuspend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onsuspend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsuspend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onsuspend(&self, onsuspend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsuspend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsuspend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsuspend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsuspend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onsuspend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsuspend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsuspend,
                    );
                __widl_f_set_onsuspend_Document(self_, onsuspend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontimeupdate_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontimeupdate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontimeupdate)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ontimeupdate(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontimeupdate_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontimeupdate_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontimeupdate_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontimeupdate_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontimeupdate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontimeupdate)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ontimeupdate(&self, ontimeupdate: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontimeupdate_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontimeupdate : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontimeupdate_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontimeupdate: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontimeupdate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontimeupdate =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontimeupdate,
                    );
                __widl_f_set_ontimeupdate_Document(self_, ontimeupdate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onvolumechange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onvolumechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvolumechange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onvolumechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onvolumechange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onvolumechange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onvolumechange_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onvolumechange_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onvolumechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvolumechange)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onvolumechange(&self, onvolumechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onvolumechange_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onvolumechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onvolumechange_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onvolumechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onvolumechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onvolumechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onvolumechange,
                    );
                __widl_f_set_onvolumechange_Document(self_, onvolumechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwaiting_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwaiting` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwaiting)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onwaiting(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwaiting_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwaiting_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwaiting_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwaiting_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwaiting` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwaiting)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onwaiting(&self, onwaiting: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwaiting_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwaiting: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwaiting_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwaiting: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onwaiting);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwaiting =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwaiting,
                    );
                __widl_f_set_onwaiting_Document(self_, onwaiting)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onselectstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onselectstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onselectstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onselectstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onselectstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onselectstart_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onselectstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onselectstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onselectstart(&self, onselectstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onselectstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onselectstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onselectstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onselectstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onselectstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onselectstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onselectstart,
                    );
                __widl_f_set_onselectstart_Document(self_, onselectstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontoggle_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontoggle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontoggle)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ontoggle(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontoggle_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontoggle_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontoggle_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontoggle_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontoggle` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontoggle)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ontoggle(&self, ontoggle: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontoggle_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontoggle: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontoggle_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontoggle: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontoggle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontoggle =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontoggle,
                    );
                __widl_f_set_ontoggle_Document(self_, ontoggle)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointercancel_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointercancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointercancel)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpointercancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointercancel_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointercancel_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointercancel_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointercancel_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointercancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointercancel)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpointercancel(&self, onpointercancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointercancel_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointercancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointercancel_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointercancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpointercancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointercancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointercancel,
                    );
                __widl_f_set_onpointercancel_Document(self_, onpointercancel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerdown_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerdown` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerdown)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpointerdown(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerdown_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerdown_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerdown_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerdown_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerdown` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerdown)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerdown(&self, onpointerdown: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerdown_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerdown : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerdown_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerdown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerdown);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerdown =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerdown,
                    );
                __widl_f_set_onpointerdown_Document(self_, onpointerdown)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerup_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerup` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerup)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpointerup(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerup_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerup_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerup_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerup_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerup` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerup)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerup(&self, onpointerup: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerup_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerup : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerup_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerup);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerup =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerup,
                    );
                __widl_f_set_onpointerup_Document(self_, onpointerup)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointermove_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointermove` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointermove)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpointermove(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointermove_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointermove_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointermove_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointermove_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointermove` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointermove)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpointermove(&self, onpointermove: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointermove_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointermove : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointermove_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointermove: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointermove);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointermove =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointermove,
                    );
                __widl_f_set_onpointermove_Document(self_, onpointermove)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerout_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerout` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerout)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpointerout(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerout_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerout_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerout_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerout_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerout` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerout)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerout(&self, onpointerout: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerout_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerout : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerout_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerout: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerout =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerout,
                    );
                __widl_f_set_onpointerout_Document(self_, onpointerout)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerover_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerover` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerover)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpointerover(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerover_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerover_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerover_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerover_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerover` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerover)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerover(&self, onpointerover: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerover_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerover : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerover_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerover: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerover);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerover =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerover,
                    );
                __widl_f_set_onpointerover_Document(self_, onpointerover)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerenter_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerenter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerenter)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpointerenter(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerenter_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerenter_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerenter_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerenter_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerenter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerenter)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerenter(&self, onpointerenter: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerenter_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerenter_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpointerenter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerenter =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerenter,
                    );
                __widl_f_set_onpointerenter_Document(self_, onpointerenter)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerleave_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerleave` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerleave)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onpointerleave(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerleave_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerleave_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerleave_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerleave_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerleave` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerleave)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerleave(&self, onpointerleave: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerleave_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerleave_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpointerleave);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerleave =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerleave,
                    );
                __widl_f_set_onpointerleave_Document(self_, onpointerleave)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ongotpointercapture_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ongotpointercapture` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ongotpointercapture(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ongotpointercapture_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ongotpointercapture_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ongotpointercapture_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ongotpointercapture_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ongotpointercapture` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ongotpointercapture(&self, ongotpointercapture: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ongotpointercapture_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ongotpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ongotpointercapture_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ongotpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ongotpointercapture);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ongotpointercapture =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ongotpointercapture,
                    );
                __widl_f_set_ongotpointercapture_Document(self_, ongotpointercapture)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onlostpointercapture_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onlostpointercapture` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onlostpointercapture(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onlostpointercapture_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onlostpointercapture_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onlostpointercapture_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onlostpointercapture_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onlostpointercapture` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onlostpointercapture(&self, onlostpointercapture: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onlostpointercapture_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onlostpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onlostpointercapture_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onlostpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onlostpointercapture);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onlostpointercapture =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onlostpointercapture,
                    );
                __widl_f_set_onlostpointercapture_Document(self_, onlostpointercapture)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationcancel_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationcancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationcancel)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onanimationcancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationcancel_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationcancel_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationcancel_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationcancel_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationcancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationcancel)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationcancel(&self, onanimationcancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationcancel_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationcancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationcancel_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationcancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationcancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationcancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationcancel,
                    );
                __widl_f_set_onanimationcancel_Document(self_, onanimationcancel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onanimationend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationend_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationend(&self, onanimationend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationend,
                    );
                __widl_f_set_onanimationend_Document(self_, onanimationend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationiteration_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationiteration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationiteration)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onanimationiteration(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationiteration_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationiteration_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationiteration_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationiteration_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationiteration` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationiteration)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationiteration(&self, onanimationiteration: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationiteration_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationiteration_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationiteration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationiteration =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationiteration,
                    );
                __widl_f_set_onanimationiteration_Document(self_, onanimationiteration)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onanimationstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationstart_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationstart(&self, onanimationstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationstart,
                    );
                __widl_f_set_onanimationstart_Document(self_, onanimationstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitioncancel_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitioncancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ontransitioncancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitioncancel_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitioncancel_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitioncancel_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitioncancel_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitioncancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitioncancel(&self, ontransitioncancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitioncancel_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitioncancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitioncancel_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitioncancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitioncancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitioncancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitioncancel,
                    );
                __widl_f_set_ontransitioncancel_Document(self_, ontransitioncancel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitionend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ontransitionend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitionend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitionend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitionend_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitionend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitionend(&self, ontransitionend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitionend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitionend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitionend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitionend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitionend,
                    );
                __widl_f_set_ontransitionend_Document(self_, ontransitionend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitionrun_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionrun` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionrun)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ontransitionrun(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitionrun_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitionrun_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitionrun_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitionrun_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionrun` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionrun)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitionrun(&self, ontransitionrun: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitionrun_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitionrun : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitionrun_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitionrun : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitionrun);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitionrun =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitionrun,
                    );
                __widl_f_set_ontransitionrun_Document(self_, ontransitionrun)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitionstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ontransitionstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitionstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitionstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitionstart_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitionstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitionstart(&self, ontransitionstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitionstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitionstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitionstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitionstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitionstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitionstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitionstart,
                    );
                __widl_f_set_ontransitionstart_Document(self_, ontransitionstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkitanimationend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onwebkitanimationend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkitanimationend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkitanimationend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkitanimationend_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkitanimationend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkitanimationend(&self, onwebkitanimationend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkitanimationend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkitanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkitanimationend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkitanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkitanimationend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkitanimationend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkitanimationend,
                    );
                __widl_f_set_onwebkitanimationend_Document(self_, onwebkitanimationend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkitanimationiteration_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationiteration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onwebkitanimationiteration(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkitanimationiteration_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkitanimationiteration_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkitanimationiteration_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkitanimationiteration_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationiteration` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkitanimationiteration(
        &self,
        onwebkitanimationiteration: Option<&::js_sys::Function>,
    ) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkitanimationiteration_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkitanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkitanimationiteration_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkitanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkitanimationiteration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkitanimationiteration =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkitanimationiteration,
                    );
                __widl_f_set_onwebkitanimationiteration_Document(self_, onwebkitanimationiteration)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkitanimationstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onwebkitanimationstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkitanimationstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkitanimationstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkitanimationstart_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkitanimationstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkitanimationstart(&self, onwebkitanimationstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkitanimationstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkitanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkitanimationstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkitanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkitanimationstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkitanimationstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkitanimationstart,
                    );
                __widl_f_set_onwebkitanimationstart_Document(self_, onwebkitanimationstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkittransitionend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkittransitionend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onwebkittransitionend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkittransitionend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkittransitionend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkittransitionend_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkittransitionend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkittransitionend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkittransitionend(&self, onwebkittransitionend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkittransitionend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkittransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkittransitionend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkittransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkittransitionend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkittransitionend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkittransitionend,
                    );
                __widl_f_set_onwebkittransitionend_Document(self_, onwebkittransitionend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onerror)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onerror)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_Document(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn append_with_node(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_append_with_node_Document(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_0_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_0_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_0_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_append_with_node_0_Document(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_1_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_1_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_1_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_append_with_node_1_Document(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_2_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_2_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_2_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_append_with_node_2_Document(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_3_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_3_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_3_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_append_with_node_3_Document(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_4_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_4_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_4_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_append_with_node_4_Document(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_5_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_5_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_5_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_append_with_node_5_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_6_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
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
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_6_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_6_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_append_with_node_6_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_7_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
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
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_7_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_append_with_node_7_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_append_with_node_7_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn append_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_append_with_str_Document(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_0_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_0_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_0_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_append_with_str_0_Document(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_1_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_1_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_1_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_append_with_str_1_Document(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_2_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_2_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_2_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_append_with_str_2_Document(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_3_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_3_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_3_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_append_with_str_3_Document(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_4_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_4_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_4_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_append_with_str_4_Document(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_5_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_5_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_5_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_append_with_str_5_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_6_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`*"]
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
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_6_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_6_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_append_with_str_6_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_7_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)\n\n*This API requires the following crate features to be activated: `Document`*"]
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
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_7_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_append_with_str_7_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_append_with_str_7_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node(
        &self,
        nodes: &::js_sys::Array,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_prepend_with_node_Document(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_0_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_0_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_0_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prepend_with_node_0_Document(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_1_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_1_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_1_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_prepend_with_node_1_Document(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_2_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_2_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_2_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_prepend_with_node_2_Document(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_3_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_3_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_3_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_prepend_with_node_3_Document(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_4_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_4_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_4_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_prepend_with_node_4_Document(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_5_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_5_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_5_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_prepend_with_node_5_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_6_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
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
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_6_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_6_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_prepend_with_node_6_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_7_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
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
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_7_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_prepend_with_node_7_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_prepend_with_node_7_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_prepend_with_str_Document(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_0_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_0_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_0_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prepend_with_str_0_Document(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_1_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_1_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_1_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_prepend_with_str_1_Document(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_2_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_2_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_2_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_prepend_with_str_2_Document(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_3_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_3_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_3_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_prepend_with_str_3_Document(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_4_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_4_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_4_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_prepend_with_str_4_Document(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_5_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_5_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_5_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_prepend_with_str_5_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_6_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`*"]
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
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_6_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_6_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_prepend_with_str_6_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_7_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)\n\n*This API requires the following crate features to be activated: `Document`*"]
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
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_7_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_prepend_with_str_7_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_prepend_with_str_7_Document(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_children_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `children` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/children)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn children(&self) -> HtmlCollection {
        #[cfg(all(feature = "Document", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_children_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_children_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_children_Document(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_first_element_child_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `firstElementChild` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/firstElementChild)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn first_element_child(&self) -> Option<Element> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_first_element_child_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_first_element_child_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_first_element_child_Document(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_last_element_child_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `lastElementChild` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastElementChild)\n\n*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[allow(clippy::all)]
    pub fn last_element_child(&self) -> Option<Element> {
        #[cfg(all(feature = "Document", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_last_element_child_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_last_element_child_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_last_element_child_Document(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_child_element_count_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `childElementCount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/childElementCount)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn child_element_count(&self) -> u32 {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_child_element_count_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_child_element_count_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_child_element_count_Document(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ontouchstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchstart_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchstart_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchstart)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchstart(&self, ontouchstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchstart_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchstart_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchstart,
                    );
                __widl_f_set_ontouchstart_Document(self_, ontouchstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ontouchend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchend_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchend_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchend)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchend(&self, ontouchend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchend_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchend_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchend,
                    );
                __widl_f_set_ontouchend_Document(self_, ontouchend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchmove_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchmove` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchmove)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ontouchmove(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchmove_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchmove_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchmove_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchmove_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchmove` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchmove)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchmove(&self, ontouchmove: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchmove_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchmove : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchmove_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchmove: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchmove);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchmove =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchmove,
                    );
                __widl_f_set_ontouchmove_Document(self_, ontouchmove)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchcancel_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Document as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchcancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchcancel)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn ontouchcancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchcancel_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchcancel_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchcancel_Document(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchcancel_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchcancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchcancel)\n\n*This API requires the following crate features to be activated: `Document`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchcancel(&self, ontouchcancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Document",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchcancel_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchcancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchcancel_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchcancel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchcancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchcancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchcancel,
                    );
                __widl_f_set_ontouchcancel_Document(self_, ontouchcancel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document", feature = "XPathExpression",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_expression_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <XPathExpression as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "XPathExpression",))]
    #[allow(bad_style)]
    #[doc = "The `createExpression()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)\n\n*This API requires the following crate features to be activated: `Document`, `XPathExpression`*"]
    #[allow(clippy::all)]
    pub fn create_expression(
        &self,
        expression: &str,
    ) -> Result<XPathExpression, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "XPathExpression",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_expression_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathExpression as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_expression_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathExpression as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(expression);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let expression = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(expression);
                __widl_f_create_expression_Document(self_, expression)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathExpression as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "XPathExpression",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_expression_with_opt_callback_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <XPathExpression as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "XPathExpression",))]
    #[allow(bad_style)]
    #[doc = "The `createExpression()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)\n\n*This API requires the following crate features to be activated: `Document`, `XPathExpression`*"]
    #[allow(clippy::all)]
    pub fn create_expression_with_opt_callback(
        &self,
        expression: &str,
        resolver: Option<&::js_sys::Function>,
    ) -> Result<XPathExpression, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "XPathExpression",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_expression_with_opt_callback_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                resolver: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathExpression as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_expression_with_opt_callback_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            resolver: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathExpression as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(expression);
            drop(resolver);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let expression = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(expression);
                let resolver =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        resolver,
                    );
                __widl_f_create_expression_with_opt_callback_Document(self_, expression, resolver)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathExpression as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "XPathExpression",
    feature = "XPathNsResolver",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_expression_with_opt_x_path_ns_resolver_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&XPathNsResolver> as WasmDescribe>::describe();
    <XPathExpression as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "XPathExpression",
        feature = "XPathNsResolver",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createExpression()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)\n\n*This API requires the following crate features to be activated: `Document`, `XPathExpression`, `XPathNsResolver`*"]
    #[allow(clippy::all)]
    pub fn create_expression_with_opt_x_path_ns_resolver(
        &self,
        expression: &str,
        resolver: Option<&XPathNsResolver>,
    ) -> Result<XPathExpression, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "XPathExpression",
            feature = "XPathNsResolver",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_expression_with_opt_x_path_ns_resolver_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                resolver: <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathExpression as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_expression_with_opt_x_path_ns_resolver_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            resolver: <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathExpression as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(expression);
            drop(resolver);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let expression = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(expression);
                let resolver =
                    <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        resolver,
                    );
                __widl_f_create_expression_with_opt_x_path_ns_resolver_Document(
                    self_, expression, resolver,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathExpression as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_ns_resolver_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Document as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Node as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `createNSResolver()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNSResolver)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    #[allow(clippy::all)]
    pub fn create_ns_resolver(&self, node_resolver: &Node) -> Node {
        #[cfg(all(feature = "Document", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_ns_resolver_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                node_resolver: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_ns_resolver_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            node_resolver: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Node as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(node_resolver);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let node_resolver =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node_resolver);
                __widl_f_create_ns_resolver_Document(self_, node_resolver)
            };
            <Node as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_evaluate_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <XPathResult as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `evaluate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn evaluate(
        &self,
        expression: &str,
        context_node: &Node,
    ) -> Result<XPathResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_evaluate_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_evaluate_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(expression);
            drop(context_node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let expression = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(expression);
                let context_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_node);
                __widl_f_evaluate_Document(self_, expression, context_node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_evaluate_with_opt_callback_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <XPathResult as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `evaluate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn evaluate_with_opt_callback(
        &self,
        expression: &str,
        context_node: &Node,
        resolver: Option<&::js_sys::Function>,
    ) -> Result<XPathResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_evaluate_with_opt_callback_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                resolver: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_evaluate_with_opt_callback_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            resolver: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(expression);
            drop(context_node);
            drop(resolver);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let expression = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(expression);
                let context_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_node);
                let resolver =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        resolver,
                    );
                __widl_f_evaluate_with_opt_callback_Document(
                    self_,
                    expression,
                    context_node,
                    resolver,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "Node",
    feature = "XPathNsResolver",
    feature = "XPathResult",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_evaluate_with_opt_x_path_ns_resolver_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Option<&XPathNsResolver> as WasmDescribe>::describe();
    <XPathResult as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "Node",
        feature = "XPathNsResolver",
        feature = "XPathResult",
    ))]
    #[allow(bad_style)]
    #[doc = "The `evaluate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `XPathNsResolver`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn evaluate_with_opt_x_path_ns_resolver(
        &self,
        expression: &str,
        context_node: &Node,
        resolver: Option<&XPathNsResolver>,
    ) -> Result<XPathResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "Node",
            feature = "XPathNsResolver",
            feature = "XPathResult",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_evaluate_with_opt_x_path_ns_resolver_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                resolver: <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_evaluate_with_opt_x_path_ns_resolver_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            resolver: <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(expression);
            drop(context_node);
            drop(resolver);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let expression = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(expression);
                let context_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_node);
                let resolver =
                    <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        resolver,
                    );
                __widl_f_evaluate_with_opt_x_path_ns_resolver_Document(
                    self_,
                    expression,
                    context_node,
                    resolver,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_evaluate_with_opt_callback_and_type_Document() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <XPathResult as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `evaluate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn evaluate_with_opt_callback_and_type(
        &self,
        expression: &str,
        context_node: &Node,
        resolver: Option<&::js_sys::Function>,
        type_: u16,
    ) -> Result<XPathResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_evaluate_with_opt_callback_and_type_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                resolver: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_evaluate_with_opt_callback_and_type_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            resolver: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(expression);
            drop(context_node);
            drop(resolver);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let expression = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(expression);
                let context_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_node);
                let resolver =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        resolver,
                    );
                let type_ = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_evaluate_with_opt_callback_and_type_Document(
                    self_,
                    expression,
                    context_node,
                    resolver,
                    type_,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "Node",
    feature = "XPathNsResolver",
    feature = "XPathResult",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_evaluate_with_opt_x_path_ns_resolver_and_type_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Option<&XPathNsResolver> as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <XPathResult as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "Node",
        feature = "XPathNsResolver",
        feature = "XPathResult",
    ))]
    #[allow(bad_style)]
    #[doc = "The `evaluate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `XPathNsResolver`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn evaluate_with_opt_x_path_ns_resolver_and_type(
        &self,
        expression: &str,
        context_node: &Node,
        resolver: Option<&XPathNsResolver>,
        type_: u16,
    ) -> Result<XPathResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "Node",
            feature = "XPathNsResolver",
            feature = "XPathResult",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_evaluate_with_opt_x_path_ns_resolver_and_type_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                resolver: <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_evaluate_with_opt_x_path_ns_resolver_and_type_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            resolver: <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(expression);
            drop(context_node);
            drop(resolver);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let expression = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(expression);
                let context_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_node);
                let resolver =
                    <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        resolver,
                    );
                let type_ = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_evaluate_with_opt_x_path_ns_resolver_and_type_Document(
                    self_,
                    expression,
                    context_node,
                    resolver,
                    type_,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_evaluate_with_opt_callback_and_type_and_result_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <XPathResult as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
    #[allow(bad_style)]
    #[doc = "The `evaluate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn evaluate_with_opt_callback_and_type_and_result(
        &self,
        expression: &str,
        context_node: &Node,
        resolver: Option<&::js_sys::Function>,
        type_: u16,
        result: Option<&::js_sys::Object>,
    ) -> Result<XPathResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "Node", feature = "XPathResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_evaluate_with_opt_callback_and_type_and_result_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                resolver: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                result: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_evaluate_with_opt_callback_and_type_and_result_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            resolver: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            result: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(expression);
            drop(context_node);
            drop(resolver);
            drop(type_);
            drop(result);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let expression = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(expression);
                let context_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_node);
                let resolver =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        resolver,
                    );
                let type_ = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let result =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        result,
                    );
                __widl_f_evaluate_with_opt_callback_and_type_and_result_Document(
                    self_,
                    expression,
                    context_node,
                    resolver,
                    type_,
                    result,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<XPathResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "Document",
    feature = "Node",
    feature = "XPathNsResolver",
    feature = "XPathResult",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_evaluate_with_opt_x_path_ns_resolver_and_type_and_result_Document(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Document as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <Option<&XPathNsResolver> as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <XPathResult as WasmDescribe>::describe();
}
impl Document {
    #[cfg(all(
        feature = "Document",
        feature = "Node",
        feature = "XPathNsResolver",
        feature = "XPathResult",
    ))]
    #[allow(bad_style)]
    #[doc = "The `evaluate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)\n\n*This API requires the following crate features to be activated: `Document`, `Node`, `XPathNsResolver`, `XPathResult`*"]
    #[allow(clippy::all)]
    pub fn evaluate_with_opt_x_path_ns_resolver_and_type_and_result(
        &self,
        expression: &str,
        context_node: &Node,
        resolver: Option<&XPathNsResolver>,
        type_: u16,
        result: Option<&::js_sys::Object>,
    ) -> Result<XPathResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Document",
            feature = "Node",
            feature = "XPathNsResolver",
            feature = "XPathResult",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_evaluate_with_opt_x_path_ns_resolver_and_type_and_result_Document(
                self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                resolver: <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                result: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_evaluate_with_opt_x_path_ns_resolver_and_type_and_result_Document(
            self_: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            expression: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_node: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            resolver: <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            result: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <XPathResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(expression);
            drop(context_node);
            drop(resolver);
            drop(type_);
            drop(result);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let expression = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(expression);
                let context_node =
                    <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_node);
                let resolver =
                    <Option<&XPathNsResolver> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        resolver,
                    );
                let type_ = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let result =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        result,
                    );
                __widl_f_evaluate_with_opt_x_path_ns_resolver_and_type_and_result_Document(
                    self_,
                    expression,
                    context_node,
                    resolver,
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
pub static __WASM_BINDGEN_GENERATED_dcba822cdd71b977: [u8; 33628usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1A\x83\0\0\0\0\xE8\x02\0\0\x02\x08Document\x1A__widl_instanceof_Document\0\0\0\0\x15__widl_f_new_Document\x01\0\0\x01\x08Document\0\x01\0\x03new\0\0\0\x1C__widl_f_adopt_node_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x04node\tadoptNode\0\0\0+__widl_f_caret_position_from_point_Document\0\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x01x\x01y\x16caretPositionFromPoint\0\0\0\"__widl_f_create_attribute_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x04name\x0FcreateAttribute\0\0\0%__widl_f_create_attribute_ns_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\tnamespace\x04name\x11createAttributeNS\0\0\0&__widl_f_create_cdata_section_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x04data\x12createCDATASection\0\0\0 __widl_f_create_comment_Document\0\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x04data\rcreateComment\0\0\0*__widl_f_create_document_fragment_Document\0\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x16createDocumentFragment\0\0\0 __widl_f_create_element_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\nlocal_name\rcreateElement\0\0\0>__widl_f_create_element_with_element_creation_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\nlocal_name\x07options\rcreateElement\0\0\0)__widl_f_create_element_with_str_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\nlocal_name\x07options\rcreateElement\0\0\0#__widl_f_create_element_ns_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\tnamespace\x0Equalified_name\x0FcreateElementNS\0\0\0A__widl_f_create_element_ns_with_element_creation_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\tnamespace\x0Equalified_name\x07options\x0FcreateElementNS\0\0\0,__widl_f_create_element_ns_with_str_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\tnamespace\x0Equalified_name\x07options\x0FcreateElementNS\0\0\0\x1E__widl_f_create_event_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\tinterface\x0BcreateEvent\0\0\0&__widl_f_create_node_iterator_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x04root\x12createNodeIterator\0\0\08__widl_f_create_node_iterator_with_what_to_show_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x04root\x0Cwhat_to_show\x12createNodeIterator\0\0\0C__widl_f_create_node_iterator_with_what_to_show_and_filter_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x04root\x0Cwhat_to_show\x06filter\x12createNodeIterator\0\0\0/__widl_f_create_processing_instruction_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x06target\x04data\x1BcreateProcessingInstruction\0\0\0\x1E__widl_f_create_range_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x0BcreateRange\0\0\0\"__widl_f_create_text_node_Document\0\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x04data\x0EcreateTextNode\0\0\0$__widl_f_create_tree_walker_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x04root\x10createTreeWalker\0\0\06__widl_f_create_tree_walker_with_what_to_show_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x04root\x0Cwhat_to_show\x10createTreeWalker\0\0\0A__widl_f_create_tree_walker_with_what_to_show_and_filter_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x04root\x0Cwhat_to_show\x06filter\x10createTreeWalker\0\0\0-__widl_f_enable_style_sheets_for_set_Document\0\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x04name\x17enableStyleSheetsForSet\0\0\0!__widl_f_exit_fullscreen_Document\0\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x0EexitFullscreen\0\0\0#__widl_f_exit_pointer_lock_Document\0\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x0FexitPointerLock\0\0\0 __widl_f_get_animations_Document\0\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\rgetAnimations\0\0\0#__widl_f_get_element_by_id_Document\0\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\nelement_id\x0EgetElementById\0\0\0,__widl_f_get_elements_by_class_name_Document\0\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x0Bclass_names\x16getElementsByClassName\0\0\0&__widl_f_get_elements_by_name_Document\0\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x0Celement_name\x11getElementsByName\0\0\0*__widl_f_get_elements_by_tag_name_Document\0\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\nlocal_name\x14getElementsByTagName\0\0\0-__widl_f_get_elements_by_tag_name_ns_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\tnamespace\nlocal_name\x16getElementsByTagNameNS\0\0\0\x1F__widl_f_get_selection_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x0CgetSelection\0\0\0\x1B__widl_f_has_focus_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x08hasFocus\0\0\0\x1D__widl_f_import_node_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x04node\nimportNode\0\0\0'__widl_f_import_node_with_deep_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x04node\x04deep\nimportNode\0\0\0 __widl_f_query_selector_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\tselectors\rquerySelector\0\0\0$__widl_f_query_selector_all_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\tselectors\x10querySelectorAll\0\0\0!__widl_f_release_capture_Document\0\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x0EreleaseCapture\0\0\0 __widl_f_implementation_Document\x01\0\0\x01\x08Document\x01\0\x01\x0Eimplementation\x01\x01\x05self_\x0Eimplementation\0\0\0\x15__widl_f_url_Document\x01\0\0\x01\x08Document\x01\0\x01\x03URL\x01\x01\x05self_\x03URL\0\0\0\x1E__widl_f_document_uri_Document\x01\0\0\x01\x08Document\x01\0\x01\x0BdocumentURI\x01\x01\x05self_\x0BdocumentURI\0\0\0\x1D__widl_f_compat_mode_Document\0\0\0\x01\x08Document\x01\0\x01\ncompatMode\x01\x01\x05self_\ncompatMode\0\0\0\x1F__widl_f_character_set_Document\0\0\0\x01\x08Document\x01\0\x01\x0CcharacterSet\x01\x01\x05self_\x0CcharacterSet\0\0\0\x19__widl_f_charset_Document\0\0\0\x01\x08Document\x01\0\x01\x07charset\x01\x01\x05self_\x07charset\0\0\0 __widl_f_input_encoding_Document\0\0\0\x01\x08Document\x01\0\x01\rinputEncoding\x01\x01\x05self_\rinputEncoding\0\0\0\x1E__widl_f_content_type_Document\0\0\0\x01\x08Document\x01\0\x01\x0BcontentType\x01\x01\x05self_\x0BcontentType\0\0\0\x19__widl_f_doctype_Document\0\0\0\x01\x08Document\x01\0\x01\x07doctype\x01\x01\x05self_\x07doctype\0\0\0\"__widl_f_document_element_Document\0\0\0\x01\x08Document\x01\0\x01\x0FdocumentElement\x01\x01\x05self_\x0FdocumentElement\0\0\0\x1A__widl_f_location_Document\0\0\0\x01\x08Document\x01\0\x01\x08location\x01\x01\x05self_\x08location\0\0\0\x1A__widl_f_referrer_Document\0\0\0\x01\x08Document\x01\0\x01\x08referrer\x01\x01\x05self_\x08referrer\0\0\0\x1F__widl_f_last_modified_Document\0\0\0\x01\x08Document\x01\0\x01\x0ClastModified\x01\x01\x05self_\x0ClastModified\0\0\0\x1D__widl_f_ready_state_Document\0\0\0\x01\x08Document\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0\x17__widl_f_title_Document\0\0\0\x01\x08Document\x01\0\x01\x05title\x01\x01\x05self_\x05title\0\0\0\x1B__widl_f_set_title_Document\0\0\0\x01\x08Document\x01\0\x02\x05title\x01\x02\x05self_\x05title\x05title\0\0\0\x15__widl_f_dir_Document\0\0\0\x01\x08Document\x01\0\x01\x03dir\x01\x01\x05self_\x03dir\0\0\0\x19__widl_f_set_dir_Document\0\0\0\x01\x08Document\x01\0\x02\x03dir\x01\x02\x05self_\x03dir\x03dir\0\0\0\x16__widl_f_body_Document\0\0\0\x01\x08Document\x01\0\x01\x04body\x01\x01\x05self_\x04body\0\0\0\x1A__widl_f_set_body_Document\0\0\0\x01\x08Document\x01\0\x02\x04body\x01\x02\x05self_\x04body\x04body\0\0\0\x16__widl_f_head_Document\0\0\0\x01\x08Document\x01\0\x01\x04head\x01\x01\x05self_\x04head\0\0\0\x18__widl_f_images_Document\0\0\0\x01\x08Document\x01\0\x01\x06images\x01\x01\x05self_\x06images\0\0\0\x18__widl_f_embeds_Document\0\0\0\x01\x08Document\x01\0\x01\x06embeds\x01\x01\x05self_\x06embeds\0\0\0\x19__widl_f_plugins_Document\0\0\0\x01\x08Document\x01\0\x01\x07plugins\x01\x01\x05self_\x07plugins\0\0\0\x17__widl_f_links_Document\0\0\0\x01\x08Document\x01\0\x01\x05links\x01\x01\x05self_\x05links\0\0\0\x17__widl_f_forms_Document\0\0\0\x01\x08Document\x01\0\x01\x05forms\x01\x01\x05self_\x05forms\0\0\0\x19__widl_f_scripts_Document\0\0\0\x01\x08Document\x01\0\x01\x07scripts\x01\x01\x05self_\x07scripts\0\0\0\x1E__widl_f_default_view_Document\0\0\0\x01\x08Document\x01\0\x01\x0BdefaultView\x01\x01\x05self_\x0BdefaultView\0\0\0$__widl_f_onreadystatechange_Document\0\0\0\x01\x08Document\x01\0\x01\x12onreadystatechange\x01\x01\x05self_\x12onreadystatechange\0\0\0(__widl_f_set_onreadystatechange_Document\0\0\0\x01\x08Document\x01\0\x02\x12onreadystatechange\x01\x02\x05self_\x12onreadystatechange\x12onreadystatechange\0\0\0'__widl_f_onbeforescriptexecute_Document\0\0\0\x01\x08Document\x01\0\x01\x15onbeforescriptexecute\x01\x01\x05self_\x15onbeforescriptexecute\0\0\0+__widl_f_set_onbeforescriptexecute_Document\0\0\0\x01\x08Document\x01\0\x02\x15onbeforescriptexecute\x01\x02\x05self_\x15onbeforescriptexecute\x15onbeforescriptexecute\0\0\0&__widl_f_onafterscriptexecute_Document\0\0\0\x01\x08Document\x01\0\x01\x14onafterscriptexecute\x01\x01\x05self_\x14onafterscriptexecute\0\0\0*__widl_f_set_onafterscriptexecute_Document\0\0\0\x01\x08Document\x01\0\x02\x14onafterscriptexecute\x01\x02\x05self_\x14onafterscriptexecute\x14onafterscriptexecute\0\0\0#__widl_f_onselectionchange_Document\0\0\0\x01\x08Document\x01\0\x01\x11onselectionchange\x01\x01\x05self_\x11onselectionchange\0\0\0'__widl_f_set_onselectionchange_Document\0\0\0\x01\x08Document\x01\0\x02\x11onselectionchange\x01\x02\x05self_\x11onselectionchange\x11onselectionchange\0\0\0 __widl_f_current_script_Document\0\0\0\x01\x08Document\x01\0\x01\rcurrentScript\x01\x01\x05self_\rcurrentScript\0\0\0\x19__widl_f_anchors_Document\0\0\0\x01\x08Document\x01\0\x01\x07anchors\x01\x01\x05self_\x07anchors\0\0\0\x19__widl_f_applets_Document\0\0\0\x01\x08Document\x01\0\x01\x07applets\x01\x01\x05self_\x07applets\0\0\0\x1C__widl_f_fullscreen_Document\0\0\0\x01\x08Document\x01\0\x01\nfullscreen\x01\x01\x05self_\nfullscreen\0\0\0$__widl_f_fullscreen_enabled_Document\0\0\0\x01\x08Document\x01\0\x01\x11fullscreenEnabled\x01\x01\x05self_\x11fullscreenEnabled\0\0\0$__widl_f_onfullscreenchange_Document\0\0\0\x01\x08Document\x01\0\x01\x12onfullscreenchange\x01\x01\x05self_\x12onfullscreenchange\0\0\0(__widl_f_set_onfullscreenchange_Document\0\0\0\x01\x08Document\x01\0\x02\x12onfullscreenchange\x01\x02\x05self_\x12onfullscreenchange\x12onfullscreenchange\0\0\0#__widl_f_onfullscreenerror_Document\0\0\0\x01\x08Document\x01\0\x01\x11onfullscreenerror\x01\x01\x05self_\x11onfullscreenerror\0\0\0'__widl_f_set_onfullscreenerror_Document\0\0\0\x01\x08Document\x01\0\x02\x11onfullscreenerror\x01\x02\x05self_\x11onfullscreenerror\x11onfullscreenerror\0\0\0%__widl_f_onpointerlockchange_Document\0\0\0\x01\x08Document\x01\0\x01\x13onpointerlockchange\x01\x01\x05self_\x13onpointerlockchange\0\0\0)__widl_f_set_onpointerlockchange_Document\0\0\0\x01\x08Document\x01\0\x02\x13onpointerlockchange\x01\x02\x05self_\x13onpointerlockchange\x13onpointerlockchange\0\0\0$__widl_f_onpointerlockerror_Document\0\0\0\x01\x08Document\x01\0\x01\x12onpointerlockerror\x01\x01\x05self_\x12onpointerlockerror\0\0\0(__widl_f_set_onpointerlockerror_Document\0\0\0\x01\x08Document\x01\0\x02\x12onpointerlockerror\x01\x02\x05self_\x12onpointerlockerror\x12onpointerlockerror\0\0\0\x18__widl_f_hidden_Document\0\0\0\x01\x08Document\x01\0\x01\x06hidden\x01\x01\x05self_\x06hidden\0\0\0\"__widl_f_visibility_state_Document\0\0\0\x01\x08Document\x01\0\x01\x0FvisibilityState\x01\x01\x05self_\x0FvisibilityState\0\0\0$__widl_f_onvisibilitychange_Document\0\0\0\x01\x08Document\x01\0\x01\x12onvisibilitychange\x01\x01\x05self_\x12onvisibilitychange\0\0\0(__widl_f_set_onvisibilitychange_Document\0\0\0\x01\x08Document\x01\0\x02\x12onvisibilitychange\x01\x02\x05self_\x12onvisibilitychange\x12onvisibilitychange\0\0\0*__widl_f_selected_style_sheet_set_Document\0\0\0\x01\x08Document\x01\0\x01\x15selectedStyleSheetSet\x01\x01\x05self_\x15selectedStyleSheetSet\0\0\0.__widl_f_set_selected_style_sheet_set_Document\0\0\0\x01\x08Document\x01\0\x02\x15selectedStyleSheetSet\x01\x02\x05self_\x18selected_style_sheet_set\x15selectedStyleSheetSet\0\0\0&__widl_f_last_style_sheet_set_Document\0\0\0\x01\x08Document\x01\0\x01\x11lastStyleSheetSet\x01\x01\x05self_\x11lastStyleSheetSet\0\0\0+__widl_f_preferred_style_sheet_set_Document\0\0\0\x01\x08Document\x01\0\x01\x16preferredStyleSheetSet\x01\x01\x05self_\x16preferredStyleSheetSet\0\0\0\"__widl_f_style_sheet_sets_Document\0\0\0\x01\x08Document\x01\0\x01\x0EstyleSheetSets\x01\x01\x05self_\x0EstyleSheetSets\0\0\0#__widl_f_scrolling_element_Document\0\0\0\x01\x08Document\x01\0\x01\x10scrollingElement\x01\x01\x05self_\x10scrollingElement\0\0\0\x1A__widl_f_timeline_Document\0\0\0\x01\x08Document\x01\0\x01\x08timeline\x01\x01\x05self_\x08timeline\0\0\0\x1E__widl_f_root_element_Document\0\0\0\x01\x08Document\x01\0\x01\x0BrootElement\x01\x01\x05self_\x0BrootElement\0\0\0\x18__widl_f_oncopy_Document\0\0\0\x01\x08Document\x01\0\x01\x06oncopy\x01\x01\x05self_\x06oncopy\0\0\0\x1C__widl_f_set_oncopy_Document\0\0\0\x01\x08Document\x01\0\x02\x06oncopy\x01\x02\x05self_\x06oncopy\x06oncopy\0\0\0\x17__widl_f_oncut_Document\0\0\0\x01\x08Document\x01\0\x01\x05oncut\x01\x01\x05self_\x05oncut\0\0\0\x1B__widl_f_set_oncut_Document\0\0\0\x01\x08Document\x01\0\x02\x05oncut\x01\x02\x05self_\x05oncut\x05oncut\0\0\0\x19__widl_f_onpaste_Document\0\0\0\x01\x08Document\x01\0\x01\x07onpaste\x01\x01\x05self_\x07onpaste\0\0\0\x1D__widl_f_set_onpaste_Document\0\0\0\x01\x08Document\x01\0\x02\x07onpaste\x01\x02\x05self_\x07onpaste\x07onpaste\0\0\0$__widl_f_element_from_point_Document\0\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x01x\x01y\x10elementFromPoint\0\0\0%__widl_f_elements_from_point_Document\0\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x01x\x01y\x11elementsFromPoint\0\0\0 __widl_f_active_element_Document\0\0\0\x01\x08Document\x01\0\x01\ractiveElement\x01\x01\x05self_\ractiveElement\0\0\0\x1E__widl_f_style_sheets_Document\0\0\0\x01\x08Document\x01\0\x01\x0BstyleSheets\x01\x01\x05self_\x0BstyleSheets\0\0\0&__widl_f_pointer_lock_element_Document\0\0\0\x01\x08Document\x01\0\x01\x12pointerLockElement\x01\x01\x05self_\x12pointerLockElement\0\0\0$__widl_f_fullscreen_element_Document\0\0\0\x01\x08Document\x01\0\x01\x11fullscreenElement\x01\x01\x05self_\x11fullscreenElement\0\0\0\x17__widl_f_fonts_Document\0\0\0\x01\x08Document\x01\0\x01\x05fonts\x01\x01\x05self_\x05fonts\0\0\03__widl_f_convert_point_from_node_with_text_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x05point\x04from\x14convertPointFromNode\0\0\06__widl_f_convert_point_from_node_with_element_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x05point\x04from\x14convertPointFromNode\0\0\07__widl_f_convert_point_from_node_with_document_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x05point\x04from\x14convertPointFromNode\0\0\0?__widl_f_convert_point_from_node_with_text_and_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x05point\x04from\x07options\x14convertPointFromNode\0\0\0B__widl_f_convert_point_from_node_with_element_and_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x05point\x04from\x07options\x14convertPointFromNode\0\0\0C__widl_f_convert_point_from_node_with_document_and_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x05point\x04from\x07options\x14convertPointFromNode\0\0\02__widl_f_convert_quad_from_node_with_text_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x04quad\x04from\x13convertQuadFromNode\0\0\05__widl_f_convert_quad_from_node_with_element_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x04quad\x04from\x13convertQuadFromNode\0\0\06__widl_f_convert_quad_from_node_with_document_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x04quad\x04from\x13convertQuadFromNode\0\0\0>__widl_f_convert_quad_from_node_with_text_and_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x04quad\x04from\x07options\x13convertQuadFromNode\0\0\0A__widl_f_convert_quad_from_node_with_element_and_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x04quad\x04from\x07options\x13convertQuadFromNode\0\0\0B__widl_f_convert_quad_from_node_with_document_and_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x04quad\x04from\x07options\x13convertQuadFromNode\0\0\02__widl_f_convert_rect_from_node_with_text_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x04rect\x04from\x13convertRectFromNode\0\0\05__widl_f_convert_rect_from_node_with_element_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x04rect\x04from\x13convertRectFromNode\0\0\06__widl_f_convert_rect_from_node_with_document_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x04rect\x04from\x13convertRectFromNode\0\0\0>__widl_f_convert_rect_from_node_with_text_and_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x04rect\x04from\x07options\x13convertRectFromNode\0\0\0A__widl_f_convert_rect_from_node_with_element_and_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x04rect\x04from\x07options\x13convertRectFromNode\0\0\0B__widl_f_convert_rect_from_node_with_document_and_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x04rect\x04from\x07options\x13convertRectFromNode\0\0\0\x1F__widl_f_get_box_quads_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x0BgetBoxQuads\0\0\0,__widl_f_get_box_quads_with_options_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x07options\x0BgetBoxQuads\0\0\0\x19__widl_f_onabort_Document\0\0\0\x01\x08Document\x01\0\x01\x07onabort\x01\x01\x05self_\x07onabort\0\0\0\x1D__widl_f_set_onabort_Document\0\0\0\x01\x08Document\x01\0\x02\x07onabort\x01\x02\x05self_\x07onabort\x07onabort\0\0\0\x18__widl_f_onblur_Document\0\0\0\x01\x08Document\x01\0\x01\x06onblur\x01\x01\x05self_\x06onblur\0\0\0\x1C__widl_f_set_onblur_Document\0\0\0\x01\x08Document\x01\0\x02\x06onblur\x01\x02\x05self_\x06onblur\x06onblur\0\0\0\x19__widl_f_onfocus_Document\0\0\0\x01\x08Document\x01\0\x01\x07onfocus\x01\x01\x05self_\x07onfocus\0\0\0\x1D__widl_f_set_onfocus_Document\0\0\0\x01\x08Document\x01\0\x02\x07onfocus\x01\x02\x05self_\x07onfocus\x07onfocus\0\0\0\x1C__widl_f_onauxclick_Document\0\0\0\x01\x08Document\x01\0\x01\nonauxclick\x01\x01\x05self_\nonauxclick\0\0\0 __widl_f_set_onauxclick_Document\0\0\0\x01\x08Document\x01\0\x02\nonauxclick\x01\x02\x05self_\nonauxclick\nonauxclick\0\0\0\x1B__widl_f_oncanplay_Document\0\0\0\x01\x08Document\x01\0\x01\toncanplay\x01\x01\x05self_\toncanplay\0\0\0\x1F__widl_f_set_oncanplay_Document\0\0\0\x01\x08Document\x01\0\x02\toncanplay\x01\x02\x05self_\toncanplay\toncanplay\0\0\0\"__widl_f_oncanplaythrough_Document\0\0\0\x01\x08Document\x01\0\x01\x10oncanplaythrough\x01\x01\x05self_\x10oncanplaythrough\0\0\0&__widl_f_set_oncanplaythrough_Document\0\0\0\x01\x08Document\x01\0\x02\x10oncanplaythrough\x01\x02\x05self_\x10oncanplaythrough\x10oncanplaythrough\0\0\0\x1A__widl_f_onchange_Document\0\0\0\x01\x08Document\x01\0\x01\x08onchange\x01\x01\x05self_\x08onchange\0\0\0\x1E__widl_f_set_onchange_Document\0\0\0\x01\x08Document\x01\0\x02\x08onchange\x01\x02\x05self_\x08onchange\x08onchange\0\0\0\x19__widl_f_onclick_Document\0\0\0\x01\x08Document\x01\0\x01\x07onclick\x01\x01\x05self_\x07onclick\0\0\0\x1D__widl_f_set_onclick_Document\0\0\0\x01\x08Document\x01\0\x02\x07onclick\x01\x02\x05self_\x07onclick\x07onclick\0\0\0\x19__widl_f_onclose_Document\0\0\0\x01\x08Document\x01\0\x01\x07onclose\x01\x01\x05self_\x07onclose\0\0\0\x1D__widl_f_set_onclose_Document\0\0\0\x01\x08Document\x01\0\x02\x07onclose\x01\x02\x05self_\x07onclose\x07onclose\0\0\0\x1F__widl_f_oncontextmenu_Document\0\0\0\x01\x08Document\x01\0\x01\roncontextmenu\x01\x01\x05self_\roncontextmenu\0\0\0#__widl_f_set_oncontextmenu_Document\0\0\0\x01\x08Document\x01\0\x02\roncontextmenu\x01\x02\x05self_\roncontextmenu\roncontextmenu\0\0\0\x1C__widl_f_ondblclick_Document\0\0\0\x01\x08Document\x01\0\x01\nondblclick\x01\x01\x05self_\nondblclick\0\0\0 __widl_f_set_ondblclick_Document\0\0\0\x01\x08Document\x01\0\x02\nondblclick\x01\x02\x05self_\nondblclick\nondblclick\0\0\0\x18__widl_f_ondrag_Document\0\0\0\x01\x08Document\x01\0\x01\x06ondrag\x01\x01\x05self_\x06ondrag\0\0\0\x1C__widl_f_set_ondrag_Document\0\0\0\x01\x08Document\x01\0\x02\x06ondrag\x01\x02\x05self_\x06ondrag\x06ondrag\0\0\0\x1B__widl_f_ondragend_Document\0\0\0\x01\x08Document\x01\0\x01\tondragend\x01\x01\x05self_\tondragend\0\0\0\x1F__widl_f_set_ondragend_Document\0\0\0\x01\x08Document\x01\0\x02\tondragend\x01\x02\x05self_\tondragend\tondragend\0\0\0\x1D__widl_f_ondragenter_Document\0\0\0\x01\x08Document\x01\0\x01\x0Bondragenter\x01\x01\x05self_\x0Bondragenter\0\0\0!__widl_f_set_ondragenter_Document\0\0\0\x01\x08Document\x01\0\x02\x0Bondragenter\x01\x02\x05self_\x0Bondragenter\x0Bondragenter\0\0\0\x1C__widl_f_ondragexit_Document\0\0\0\x01\x08Document\x01\0\x01\nondragexit\x01\x01\x05self_\nondragexit\0\0\0 __widl_f_set_ondragexit_Document\0\0\0\x01\x08Document\x01\0\x02\nondragexit\x01\x02\x05self_\nondragexit\nondragexit\0\0\0\x1D__widl_f_ondragleave_Document\0\0\0\x01\x08Document\x01\0\x01\x0Bondragleave\x01\x01\x05self_\x0Bondragleave\0\0\0!__widl_f_set_ondragleave_Document\0\0\0\x01\x08Document\x01\0\x02\x0Bondragleave\x01\x02\x05self_\x0Bondragleave\x0Bondragleave\0\0\0\x1C__widl_f_ondragover_Document\0\0\0\x01\x08Document\x01\0\x01\nondragover\x01\x01\x05self_\nondragover\0\0\0 __widl_f_set_ondragover_Document\0\0\0\x01\x08Document\x01\0\x02\nondragover\x01\x02\x05self_\nondragover\nondragover\0\0\0\x1D__widl_f_ondragstart_Document\0\0\0\x01\x08Document\x01\0\x01\x0Bondragstart\x01\x01\x05self_\x0Bondragstart\0\0\0!__widl_f_set_ondragstart_Document\0\0\0\x01\x08Document\x01\0\x02\x0Bondragstart\x01\x02\x05self_\x0Bondragstart\x0Bondragstart\0\0\0\x18__widl_f_ondrop_Document\0\0\0\x01\x08Document\x01\0\x01\x06ondrop\x01\x01\x05self_\x06ondrop\0\0\0\x1C__widl_f_set_ondrop_Document\0\0\0\x01\x08Document\x01\0\x02\x06ondrop\x01\x02\x05self_\x06ondrop\x06ondrop\0\0\0\"__widl_f_ondurationchange_Document\0\0\0\x01\x08Document\x01\0\x01\x10ondurationchange\x01\x01\x05self_\x10ondurationchange\0\0\0&__widl_f_set_ondurationchange_Document\0\0\0\x01\x08Document\x01\0\x02\x10ondurationchange\x01\x02\x05self_\x10ondurationchange\x10ondurationchange\0\0\0\x1B__widl_f_onemptied_Document\0\0\0\x01\x08Document\x01\0\x01\tonemptied\x01\x01\x05self_\tonemptied\0\0\0\x1F__widl_f_set_onemptied_Document\0\0\0\x01\x08Document\x01\0\x02\tonemptied\x01\x02\x05self_\tonemptied\tonemptied\0\0\0\x19__widl_f_onended_Document\0\0\0\x01\x08Document\x01\0\x01\x07onended\x01\x01\x05self_\x07onended\0\0\0\x1D__widl_f_set_onended_Document\0\0\0\x01\x08Document\x01\0\x02\x07onended\x01\x02\x05self_\x07onended\x07onended\0\0\0\x19__widl_f_oninput_Document\0\0\0\x01\x08Document\x01\0\x01\x07oninput\x01\x01\x05self_\x07oninput\0\0\0\x1D__widl_f_set_oninput_Document\0\0\0\x01\x08Document\x01\0\x02\x07oninput\x01\x02\x05self_\x07oninput\x07oninput\0\0\0\x1B__widl_f_oninvalid_Document\0\0\0\x01\x08Document\x01\0\x01\toninvalid\x01\x01\x05self_\toninvalid\0\0\0\x1F__widl_f_set_oninvalid_Document\0\0\0\x01\x08Document\x01\0\x02\toninvalid\x01\x02\x05self_\toninvalid\toninvalid\0\0\0\x1B__widl_f_onkeydown_Document\0\0\0\x01\x08Document\x01\0\x01\tonkeydown\x01\x01\x05self_\tonkeydown\0\0\0\x1F__widl_f_set_onkeydown_Document\0\0\0\x01\x08Document\x01\0\x02\tonkeydown\x01\x02\x05self_\tonkeydown\tonkeydown\0\0\0\x1C__widl_f_onkeypress_Document\0\0\0\x01\x08Document\x01\0\x01\nonkeypress\x01\x01\x05self_\nonkeypress\0\0\0 __widl_f_set_onkeypress_Document\0\0\0\x01\x08Document\x01\0\x02\nonkeypress\x01\x02\x05self_\nonkeypress\nonkeypress\0\0\0\x19__widl_f_onkeyup_Document\0\0\0\x01\x08Document\x01\0\x01\x07onkeyup\x01\x01\x05self_\x07onkeyup\0\0\0\x1D__widl_f_set_onkeyup_Document\0\0\0\x01\x08Document\x01\0\x02\x07onkeyup\x01\x02\x05self_\x07onkeyup\x07onkeyup\0\0\0\x18__widl_f_onload_Document\0\0\0\x01\x08Document\x01\0\x01\x06onload\x01\x01\x05self_\x06onload\0\0\0\x1C__widl_f_set_onload_Document\0\0\0\x01\x08Document\x01\0\x02\x06onload\x01\x02\x05self_\x06onload\x06onload\0\0\0\x1E__widl_f_onloadeddata_Document\0\0\0\x01\x08Document\x01\0\x01\x0Conloadeddata\x01\x01\x05self_\x0Conloadeddata\0\0\0\"__widl_f_set_onloadeddata_Document\0\0\0\x01\x08Document\x01\0\x02\x0Conloadeddata\x01\x02\x05self_\x0Conloadeddata\x0Conloadeddata\0\0\0\"__widl_f_onloadedmetadata_Document\0\0\0\x01\x08Document\x01\0\x01\x10onloadedmetadata\x01\x01\x05self_\x10onloadedmetadata\0\0\0&__widl_f_set_onloadedmetadata_Document\0\0\0\x01\x08Document\x01\0\x02\x10onloadedmetadata\x01\x02\x05self_\x10onloadedmetadata\x10onloadedmetadata\0\0\0\x1B__widl_f_onloadend_Document\0\0\0\x01\x08Document\x01\0\x01\tonloadend\x01\x01\x05self_\tonloadend\0\0\0\x1F__widl_f_set_onloadend_Document\0\0\0\x01\x08Document\x01\0\x02\tonloadend\x01\x02\x05self_\tonloadend\tonloadend\0\0\0\x1D__widl_f_onloadstart_Document\0\0\0\x01\x08Document\x01\0\x01\x0Bonloadstart\x01\x01\x05self_\x0Bonloadstart\0\0\0!__widl_f_set_onloadstart_Document\0\0\0\x01\x08Document\x01\0\x02\x0Bonloadstart\x01\x02\x05self_\x0Bonloadstart\x0Bonloadstart\0\0\0\x1D__widl_f_onmousedown_Document\0\0\0\x01\x08Document\x01\0\x01\x0Bonmousedown\x01\x01\x05self_\x0Bonmousedown\0\0\0!__widl_f_set_onmousedown_Document\0\0\0\x01\x08Document\x01\0\x02\x0Bonmousedown\x01\x02\x05self_\x0Bonmousedown\x0Bonmousedown\0\0\0\x1E__widl_f_onmouseenter_Document\0\0\0\x01\x08Document\x01\0\x01\x0Conmouseenter\x01\x01\x05self_\x0Conmouseenter\0\0\0\"__widl_f_set_onmouseenter_Document\0\0\0\x01\x08Document\x01\0\x02\x0Conmouseenter\x01\x02\x05self_\x0Conmouseenter\x0Conmouseenter\0\0\0\x1E__widl_f_onmouseleave_Document\0\0\0\x01\x08Document\x01\0\x01\x0Conmouseleave\x01\x01\x05self_\x0Conmouseleave\0\0\0\"__widl_f_set_onmouseleave_Document\0\0\0\x01\x08Document\x01\0\x02\x0Conmouseleave\x01\x02\x05self_\x0Conmouseleave\x0Conmouseleave\0\0\0\x1D__widl_f_onmousemove_Document\0\0\0\x01\x08Document\x01\0\x01\x0Bonmousemove\x01\x01\x05self_\x0Bonmousemove\0\0\0!__widl_f_set_onmousemove_Document\0\0\0\x01\x08Document\x01\0\x02\x0Bonmousemove\x01\x02\x05self_\x0Bonmousemove\x0Bonmousemove\0\0\0\x1C__widl_f_onmouseout_Document\0\0\0\x01\x08Document\x01\0\x01\nonmouseout\x01\x01\x05self_\nonmouseout\0\0\0 __widl_f_set_onmouseout_Document\0\0\0\x01\x08Document\x01\0\x02\nonmouseout\x01\x02\x05self_\nonmouseout\nonmouseout\0\0\0\x1D__widl_f_onmouseover_Document\0\0\0\x01\x08Document\x01\0\x01\x0Bonmouseover\x01\x01\x05self_\x0Bonmouseover\0\0\0!__widl_f_set_onmouseover_Document\0\0\0\x01\x08Document\x01\0\x02\x0Bonmouseover\x01\x02\x05self_\x0Bonmouseover\x0Bonmouseover\0\0\0\x1B__widl_f_onmouseup_Document\0\0\0\x01\x08Document\x01\0\x01\tonmouseup\x01\x01\x05self_\tonmouseup\0\0\0\x1F__widl_f_set_onmouseup_Document\0\0\0\x01\x08Document\x01\0\x02\tonmouseup\x01\x02\x05self_\tonmouseup\tonmouseup\0\0\0\x19__widl_f_onwheel_Document\0\0\0\x01\x08Document\x01\0\x01\x07onwheel\x01\x01\x05self_\x07onwheel\0\0\0\x1D__widl_f_set_onwheel_Document\0\0\0\x01\x08Document\x01\0\x02\x07onwheel\x01\x02\x05self_\x07onwheel\x07onwheel\0\0\0\x19__widl_f_onpause_Document\0\0\0\x01\x08Document\x01\0\x01\x07onpause\x01\x01\x05self_\x07onpause\0\0\0\x1D__widl_f_set_onpause_Document\0\0\0\x01\x08Document\x01\0\x02\x07onpause\x01\x02\x05self_\x07onpause\x07onpause\0\0\0\x18__widl_f_onplay_Document\0\0\0\x01\x08Document\x01\0\x01\x06onplay\x01\x01\x05self_\x06onplay\0\0\0\x1C__widl_f_set_onplay_Document\0\0\0\x01\x08Document\x01\0\x02\x06onplay\x01\x02\x05self_\x06onplay\x06onplay\0\0\0\x1B__widl_f_onplaying_Document\0\0\0\x01\x08Document\x01\0\x01\tonplaying\x01\x01\x05self_\tonplaying\0\0\0\x1F__widl_f_set_onplaying_Document\0\0\0\x01\x08Document\x01\0\x02\tonplaying\x01\x02\x05self_\tonplaying\tonplaying\0\0\0\x1C__widl_f_onprogress_Document\0\0\0\x01\x08Document\x01\0\x01\nonprogress\x01\x01\x05self_\nonprogress\0\0\0 __widl_f_set_onprogress_Document\0\0\0\x01\x08Document\x01\0\x02\nonprogress\x01\x02\x05self_\nonprogress\nonprogress\0\0\0\x1E__widl_f_onratechange_Document\0\0\0\x01\x08Document\x01\0\x01\x0Conratechange\x01\x01\x05self_\x0Conratechange\0\0\0\"__widl_f_set_onratechange_Document\0\0\0\x01\x08Document\x01\0\x02\x0Conratechange\x01\x02\x05self_\x0Conratechange\x0Conratechange\0\0\0\x19__widl_f_onreset_Document\0\0\0\x01\x08Document\x01\0\x01\x07onreset\x01\x01\x05self_\x07onreset\0\0\0\x1D__widl_f_set_onreset_Document\0\0\0\x01\x08Document\x01\0\x02\x07onreset\x01\x02\x05self_\x07onreset\x07onreset\0\0\0\x1A__widl_f_onresize_Document\0\0\0\x01\x08Document\x01\0\x01\x08onresize\x01\x01\x05self_\x08onresize\0\0\0\x1E__widl_f_set_onresize_Document\0\0\0\x01\x08Document\x01\0\x02\x08onresize\x01\x02\x05self_\x08onresize\x08onresize\0\0\0\x1A__widl_f_onscroll_Document\0\0\0\x01\x08Document\x01\0\x01\x08onscroll\x01\x01\x05self_\x08onscroll\0\0\0\x1E__widl_f_set_onscroll_Document\0\0\0\x01\x08Document\x01\0\x02\x08onscroll\x01\x02\x05self_\x08onscroll\x08onscroll\0\0\0\x1A__widl_f_onseeked_Document\0\0\0\x01\x08Document\x01\0\x01\x08onseeked\x01\x01\x05self_\x08onseeked\0\0\0\x1E__widl_f_set_onseeked_Document\0\0\0\x01\x08Document\x01\0\x02\x08onseeked\x01\x02\x05self_\x08onseeked\x08onseeked\0\0\0\x1B__widl_f_onseeking_Document\0\0\0\x01\x08Document\x01\0\x01\tonseeking\x01\x01\x05self_\tonseeking\0\0\0\x1F__widl_f_set_onseeking_Document\0\0\0\x01\x08Document\x01\0\x02\tonseeking\x01\x02\x05self_\tonseeking\tonseeking\0\0\0\x1A__widl_f_onselect_Document\0\0\0\x01\x08Document\x01\0\x01\x08onselect\x01\x01\x05self_\x08onselect\0\0\0\x1E__widl_f_set_onselect_Document\0\0\0\x01\x08Document\x01\0\x02\x08onselect\x01\x02\x05self_\x08onselect\x08onselect\0\0\0\x18__widl_f_onshow_Document\0\0\0\x01\x08Document\x01\0\x01\x06onshow\x01\x01\x05self_\x06onshow\0\0\0\x1C__widl_f_set_onshow_Document\0\0\0\x01\x08Document\x01\0\x02\x06onshow\x01\x02\x05self_\x06onshow\x06onshow\0\0\0\x1B__widl_f_onstalled_Document\0\0\0\x01\x08Document\x01\0\x01\tonstalled\x01\x01\x05self_\tonstalled\0\0\0\x1F__widl_f_set_onstalled_Document\0\0\0\x01\x08Document\x01\0\x02\tonstalled\x01\x02\x05self_\tonstalled\tonstalled\0\0\0\x1A__widl_f_onsubmit_Document\0\0\0\x01\x08Document\x01\0\x01\x08onsubmit\x01\x01\x05self_\x08onsubmit\0\0\0\x1E__widl_f_set_onsubmit_Document\0\0\0\x01\x08Document\x01\0\x02\x08onsubmit\x01\x02\x05self_\x08onsubmit\x08onsubmit\0\0\0\x1B__widl_f_onsuspend_Document\0\0\0\x01\x08Document\x01\0\x01\tonsuspend\x01\x01\x05self_\tonsuspend\0\0\0\x1F__widl_f_set_onsuspend_Document\0\0\0\x01\x08Document\x01\0\x02\tonsuspend\x01\x02\x05self_\tonsuspend\tonsuspend\0\0\0\x1E__widl_f_ontimeupdate_Document\0\0\0\x01\x08Document\x01\0\x01\x0Contimeupdate\x01\x01\x05self_\x0Contimeupdate\0\0\0\"__widl_f_set_ontimeupdate_Document\0\0\0\x01\x08Document\x01\0\x02\x0Contimeupdate\x01\x02\x05self_\x0Contimeupdate\x0Contimeupdate\0\0\0 __widl_f_onvolumechange_Document\0\0\0\x01\x08Document\x01\0\x01\x0Eonvolumechange\x01\x01\x05self_\x0Eonvolumechange\0\0\0$__widl_f_set_onvolumechange_Document\0\0\0\x01\x08Document\x01\0\x02\x0Eonvolumechange\x01\x02\x05self_\x0Eonvolumechange\x0Eonvolumechange\0\0\0\x1B__widl_f_onwaiting_Document\0\0\0\x01\x08Document\x01\0\x01\tonwaiting\x01\x01\x05self_\tonwaiting\0\0\0\x1F__widl_f_set_onwaiting_Document\0\0\0\x01\x08Document\x01\0\x02\tonwaiting\x01\x02\x05self_\tonwaiting\tonwaiting\0\0\0\x1F__widl_f_onselectstart_Document\0\0\0\x01\x08Document\x01\0\x01\ronselectstart\x01\x01\x05self_\ronselectstart\0\0\0#__widl_f_set_onselectstart_Document\0\0\0\x01\x08Document\x01\0\x02\ronselectstart\x01\x02\x05self_\ronselectstart\ronselectstart\0\0\0\x1A__widl_f_ontoggle_Document\0\0\0\x01\x08Document\x01\0\x01\x08ontoggle\x01\x01\x05self_\x08ontoggle\0\0\0\x1E__widl_f_set_ontoggle_Document\0\0\0\x01\x08Document\x01\0\x02\x08ontoggle\x01\x02\x05self_\x08ontoggle\x08ontoggle\0\0\0!__widl_f_onpointercancel_Document\0\0\0\x01\x08Document\x01\0\x01\x0Fonpointercancel\x01\x01\x05self_\x0Fonpointercancel\0\0\0%__widl_f_set_onpointercancel_Document\0\0\0\x01\x08Document\x01\0\x02\x0Fonpointercancel\x01\x02\x05self_\x0Fonpointercancel\x0Fonpointercancel\0\0\0\x1F__widl_f_onpointerdown_Document\0\0\0\x01\x08Document\x01\0\x01\ronpointerdown\x01\x01\x05self_\ronpointerdown\0\0\0#__widl_f_set_onpointerdown_Document\0\0\0\x01\x08Document\x01\0\x02\ronpointerdown\x01\x02\x05self_\ronpointerdown\ronpointerdown\0\0\0\x1D__widl_f_onpointerup_Document\0\0\0\x01\x08Document\x01\0\x01\x0Bonpointerup\x01\x01\x05self_\x0Bonpointerup\0\0\0!__widl_f_set_onpointerup_Document\0\0\0\x01\x08Document\x01\0\x02\x0Bonpointerup\x01\x02\x05self_\x0Bonpointerup\x0Bonpointerup\0\0\0\x1F__widl_f_onpointermove_Document\0\0\0\x01\x08Document\x01\0\x01\ronpointermove\x01\x01\x05self_\ronpointermove\0\0\0#__widl_f_set_onpointermove_Document\0\0\0\x01\x08Document\x01\0\x02\ronpointermove\x01\x02\x05self_\ronpointermove\ronpointermove\0\0\0\x1E__widl_f_onpointerout_Document\0\0\0\x01\x08Document\x01\0\x01\x0Conpointerout\x01\x01\x05self_\x0Conpointerout\0\0\0\"__widl_f_set_onpointerout_Document\0\0\0\x01\x08Document\x01\0\x02\x0Conpointerout\x01\x02\x05self_\x0Conpointerout\x0Conpointerout\0\0\0\x1F__widl_f_onpointerover_Document\0\0\0\x01\x08Document\x01\0\x01\ronpointerover\x01\x01\x05self_\ronpointerover\0\0\0#__widl_f_set_onpointerover_Document\0\0\0\x01\x08Document\x01\0\x02\ronpointerover\x01\x02\x05self_\ronpointerover\ronpointerover\0\0\0 __widl_f_onpointerenter_Document\0\0\0\x01\x08Document\x01\0\x01\x0Eonpointerenter\x01\x01\x05self_\x0Eonpointerenter\0\0\0$__widl_f_set_onpointerenter_Document\0\0\0\x01\x08Document\x01\0\x02\x0Eonpointerenter\x01\x02\x05self_\x0Eonpointerenter\x0Eonpointerenter\0\0\0 __widl_f_onpointerleave_Document\0\0\0\x01\x08Document\x01\0\x01\x0Eonpointerleave\x01\x01\x05self_\x0Eonpointerleave\0\0\0$__widl_f_set_onpointerleave_Document\0\0\0\x01\x08Document\x01\0\x02\x0Eonpointerleave\x01\x02\x05self_\x0Eonpointerleave\x0Eonpointerleave\0\0\0%__widl_f_ongotpointercapture_Document\0\0\0\x01\x08Document\x01\0\x01\x13ongotpointercapture\x01\x01\x05self_\x13ongotpointercapture\0\0\0)__widl_f_set_ongotpointercapture_Document\0\0\0\x01\x08Document\x01\0\x02\x13ongotpointercapture\x01\x02\x05self_\x13ongotpointercapture\x13ongotpointercapture\0\0\0&__widl_f_onlostpointercapture_Document\0\0\0\x01\x08Document\x01\0\x01\x14onlostpointercapture\x01\x01\x05self_\x14onlostpointercapture\0\0\0*__widl_f_set_onlostpointercapture_Document\0\0\0\x01\x08Document\x01\0\x02\x14onlostpointercapture\x01\x02\x05self_\x14onlostpointercapture\x14onlostpointercapture\0\0\0#__widl_f_onanimationcancel_Document\0\0\0\x01\x08Document\x01\0\x01\x11onanimationcancel\x01\x01\x05self_\x11onanimationcancel\0\0\0'__widl_f_set_onanimationcancel_Document\0\0\0\x01\x08Document\x01\0\x02\x11onanimationcancel\x01\x02\x05self_\x11onanimationcancel\x11onanimationcancel\0\0\0 __widl_f_onanimationend_Document\0\0\0\x01\x08Document\x01\0\x01\x0Eonanimationend\x01\x01\x05self_\x0Eonanimationend\0\0\0$__widl_f_set_onanimationend_Document\0\0\0\x01\x08Document\x01\0\x02\x0Eonanimationend\x01\x02\x05self_\x0Eonanimationend\x0Eonanimationend\0\0\0&__widl_f_onanimationiteration_Document\0\0\0\x01\x08Document\x01\0\x01\x14onanimationiteration\x01\x01\x05self_\x14onanimationiteration\0\0\0*__widl_f_set_onanimationiteration_Document\0\0\0\x01\x08Document\x01\0\x02\x14onanimationiteration\x01\x02\x05self_\x14onanimationiteration\x14onanimationiteration\0\0\0\"__widl_f_onanimationstart_Document\0\0\0\x01\x08Document\x01\0\x01\x10onanimationstart\x01\x01\x05self_\x10onanimationstart\0\0\0&__widl_f_set_onanimationstart_Document\0\0\0\x01\x08Document\x01\0\x02\x10onanimationstart\x01\x02\x05self_\x10onanimationstart\x10onanimationstart\0\0\0$__widl_f_ontransitioncancel_Document\0\0\0\x01\x08Document\x01\0\x01\x12ontransitioncancel\x01\x01\x05self_\x12ontransitioncancel\0\0\0(__widl_f_set_ontransitioncancel_Document\0\0\0\x01\x08Document\x01\0\x02\x12ontransitioncancel\x01\x02\x05self_\x12ontransitioncancel\x12ontransitioncancel\0\0\0!__widl_f_ontransitionend_Document\0\0\0\x01\x08Document\x01\0\x01\x0Fontransitionend\x01\x01\x05self_\x0Fontransitionend\0\0\0%__widl_f_set_ontransitionend_Document\0\0\0\x01\x08Document\x01\0\x02\x0Fontransitionend\x01\x02\x05self_\x0Fontransitionend\x0Fontransitionend\0\0\0!__widl_f_ontransitionrun_Document\0\0\0\x01\x08Document\x01\0\x01\x0Fontransitionrun\x01\x01\x05self_\x0Fontransitionrun\0\0\0%__widl_f_set_ontransitionrun_Document\0\0\0\x01\x08Document\x01\0\x02\x0Fontransitionrun\x01\x02\x05self_\x0Fontransitionrun\x0Fontransitionrun\0\0\0#__widl_f_ontransitionstart_Document\0\0\0\x01\x08Document\x01\0\x01\x11ontransitionstart\x01\x01\x05self_\x11ontransitionstart\0\0\0'__widl_f_set_ontransitionstart_Document\0\0\0\x01\x08Document\x01\0\x02\x11ontransitionstart\x01\x02\x05self_\x11ontransitionstart\x11ontransitionstart\0\0\0&__widl_f_onwebkitanimationend_Document\0\0\0\x01\x08Document\x01\0\x01\x14onwebkitanimationend\x01\x01\x05self_\x14onwebkitanimationend\0\0\0*__widl_f_set_onwebkitanimationend_Document\0\0\0\x01\x08Document\x01\0\x02\x14onwebkitanimationend\x01\x02\x05self_\x14onwebkitanimationend\x14onwebkitanimationend\0\0\0,__widl_f_onwebkitanimationiteration_Document\0\0\0\x01\x08Document\x01\0\x01\x1Aonwebkitanimationiteration\x01\x01\x05self_\x1Aonwebkitanimationiteration\0\0\00__widl_f_set_onwebkitanimationiteration_Document\0\0\0\x01\x08Document\x01\0\x02\x1Aonwebkitanimationiteration\x01\x02\x05self_\x1Aonwebkitanimationiteration\x1Aonwebkitanimationiteration\0\0\0(__widl_f_onwebkitanimationstart_Document\0\0\0\x01\x08Document\x01\0\x01\x16onwebkitanimationstart\x01\x01\x05self_\x16onwebkitanimationstart\0\0\0,__widl_f_set_onwebkitanimationstart_Document\0\0\0\x01\x08Document\x01\0\x02\x16onwebkitanimationstart\x01\x02\x05self_\x16onwebkitanimationstart\x16onwebkitanimationstart\0\0\0'__widl_f_onwebkittransitionend_Document\0\0\0\x01\x08Document\x01\0\x01\x15onwebkittransitionend\x01\x01\x05self_\x15onwebkittransitionend\0\0\0+__widl_f_set_onwebkittransitionend_Document\0\0\0\x01\x08Document\x01\0\x02\x15onwebkittransitionend\x01\x02\x05self_\x15onwebkittransitionend\x15onwebkittransitionend\0\0\0\x19__widl_f_onerror_Document\0\0\0\x01\x08Document\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\x1D__widl_f_set_onerror_Document\0\0\0\x01\x08Document\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\"__widl_f_append_with_node_Document\x01\x01\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x05nodes\x06append\0\0\0$__widl_f_append_with_node_0_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x06append\0\0\0$__widl_f_append_with_node_1_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x07nodes_1\x06append\0\0\0$__widl_f_append_with_node_2_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06append\0\0\0$__widl_f_append_with_node_3_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06append\0\0\0$__widl_f_append_with_node_4_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06append\0\0\0$__widl_f_append_with_node_5_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06append\0\0\0$__widl_f_append_with_node_6_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06append\0\0\0$__widl_f_append_with_node_7_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06append\0\0\0!__widl_f_append_with_str_Document\x01\x01\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x05nodes\x06append\0\0\0#__widl_f_append_with_str_0_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x06append\0\0\0#__widl_f_append_with_str_1_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x07nodes_1\x06append\0\0\0#__widl_f_append_with_str_2_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06append\0\0\0#__widl_f_append_with_str_3_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06append\0\0\0#__widl_f_append_with_str_4_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06append\0\0\0#__widl_f_append_with_str_5_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06append\0\0\0#__widl_f_append_with_str_6_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06append\0\0\0#__widl_f_append_with_str_7_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06append\0\0\0#__widl_f_prepend_with_node_Document\x01\x01\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x05nodes\x07prepend\0\0\0%__widl_f_prepend_with_node_0_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x07prepend\0\0\0%__widl_f_prepend_with_node_1_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x07nodes_1\x07prepend\0\0\0%__widl_f_prepend_with_node_2_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x07prepend\0\0\0%__widl_f_prepend_with_node_3_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07prepend\0\0\0%__widl_f_prepend_with_node_4_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07prepend\0\0\0%__widl_f_prepend_with_node_5_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07prepend\0\0\0%__widl_f_prepend_with_node_6_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07prepend\0\0\0%__widl_f_prepend_with_node_7_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x07prepend\0\0\0\"__widl_f_prepend_with_str_Document\x01\x01\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x05nodes\x07prepend\0\0\0$__widl_f_prepend_with_str_0_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x01\x05self_\x07prepend\0\0\0$__widl_f_prepend_with_str_1_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\x07nodes_1\x07prepend\0\0\0$__widl_f_prepend_with_str_2_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x07prepend\0\0\0$__widl_f_prepend_with_str_3_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07prepend\0\0\0$__widl_f_prepend_with_str_4_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07prepend\0\0\0$__widl_f_prepend_with_str_5_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07prepend\0\0\0$__widl_f_prepend_with_str_6_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07prepend\0\0\0$__widl_f_prepend_with_str_7_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x07prepend\0\0\0\x1A__widl_f_children_Document\0\0\0\x01\x08Document\x01\0\x01\x08children\x01\x01\x05self_\x08children\0\0\0%__widl_f_first_element_child_Document\0\0\0\x01\x08Document\x01\0\x01\x11firstElementChild\x01\x01\x05self_\x11firstElementChild\0\0\0$__widl_f_last_element_child_Document\0\0\0\x01\x08Document\x01\0\x01\x10lastElementChild\x01\x01\x05self_\x10lastElementChild\0\0\0%__widl_f_child_element_count_Document\0\0\0\x01\x08Document\x01\0\x01\x11childElementCount\x01\x01\x05self_\x11childElementCount\0\0\0\x1E__widl_f_ontouchstart_Document\0\0\0\x01\x08Document\x01\0\x01\x0Contouchstart\x01\x01\x05self_\x0Contouchstart\0\0\0\"__widl_f_set_ontouchstart_Document\0\0\0\x01\x08Document\x01\0\x02\x0Contouchstart\x01\x02\x05self_\x0Contouchstart\x0Contouchstart\0\0\0\x1C__widl_f_ontouchend_Document\0\0\0\x01\x08Document\x01\0\x01\nontouchend\x01\x01\x05self_\nontouchend\0\0\0 __widl_f_set_ontouchend_Document\0\0\0\x01\x08Document\x01\0\x02\nontouchend\x01\x02\x05self_\nontouchend\nontouchend\0\0\0\x1D__widl_f_ontouchmove_Document\0\0\0\x01\x08Document\x01\0\x01\x0Bontouchmove\x01\x01\x05self_\x0Bontouchmove\0\0\0!__widl_f_set_ontouchmove_Document\0\0\0\x01\x08Document\x01\0\x02\x0Bontouchmove\x01\x02\x05self_\x0Bontouchmove\x0Bontouchmove\0\0\0\x1F__widl_f_ontouchcancel_Document\0\0\0\x01\x08Document\x01\0\x01\rontouchcancel\x01\x01\x05self_\rontouchcancel\0\0\0#__widl_f_set_ontouchcancel_Document\0\0\0\x01\x08Document\x01\0\x02\rontouchcancel\x01\x02\x05self_\rontouchcancel\rontouchcancel\0\0\0#__widl_f_create_expression_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\nexpression\x10createExpression\0\0\05__widl_f_create_expression_with_opt_callback_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\nexpression\x08resolver\x10createExpression\0\0\0?__widl_f_create_expression_with_opt_x_path_ns_resolver_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\nexpression\x08resolver\x10createExpression\0\0\0$__widl_f_create_ns_resolver_Document\0\0\0\x01\x08Document\x01\0\0\x01\x02\x05self_\rnode_resolver\x10createNSResolver\0\0\0\x1A__widl_f_evaluate_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x03\x05self_\nexpression\x0Ccontext_node\x08evaluate\0\0\0,__widl_f_evaluate_with_opt_callback_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\nexpression\x0Ccontext_node\x08resolver\x08evaluate\0\0\06__widl_f_evaluate_with_opt_x_path_ns_resolver_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x04\x05self_\nexpression\x0Ccontext_node\x08resolver\x08evaluate\0\0\05__widl_f_evaluate_with_opt_callback_and_type_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x05\x05self_\nexpression\x0Ccontext_node\x08resolver\x05type_\x08evaluate\0\0\0?__widl_f_evaluate_with_opt_x_path_ns_resolver_and_type_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x05\x05self_\nexpression\x0Ccontext_node\x08resolver\x05type_\x08evaluate\0\0\0@__widl_f_evaluate_with_opt_callback_and_type_and_result_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x06\x05self_\nexpression\x0Ccontext_node\x08resolver\x05type_\x06result\x08evaluate\0\0\0J__widl_f_evaluate_with_opt_x_path_ns_resolver_and_type_and_result_Document\x01\0\0\x01\x08Document\x01\0\0\x01\x06\x05self_\nexpression\x0Ccontext_node\x08resolver\x05type_\x06result\x08evaluate\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
