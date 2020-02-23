use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Element` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element)\n\n*This API requires the following crate features to be activated: `Element`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Element {
    obj: Node,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Element: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Element {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for Element {
        type Target = Node;
        #[inline]
        fn deref(&self) -> &Node {
            &self.obj
        }
    }
    impl IntoWasmAbi for Element {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Element {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Element {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Element {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Element {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Element {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Element {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Element {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Element>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Element {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Element {
        #[inline]
        fn from(obj: JsValue) -> Element {
            Element { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Element {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Element> for Element {
        #[inline]
        fn as_ref(&self) -> &Element {
            self
        }
    }
    impl From<Element> for JsValue {
        #[inline]
        fn from(obj: Element) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Element {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Element(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Element(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Element(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Element { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Element) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Element> for Node {
    #[inline]
    fn from(obj: Element) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for Element {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Element> for EventTarget {
    #[inline]
    fn from(obj: Element) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for Element {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Element> for ::js_sys::Object {
    #[inline]
    fn from(obj: Element) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Element {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "Element",
    feature = "ShadowRoot",
    feature = "ShadowRootInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_attach_shadow_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&ShadowRootInit as WasmDescribe>::describe();
    <ShadowRoot as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "Element",
        feature = "ShadowRoot",
        feature = "ShadowRootInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `attachShadow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`, `ShadowRootInit`*"]
    #[allow(clippy::all)]
    pub fn attach_shadow(
        &self,
        shadow_root_init_dict: &ShadowRootInit,
    ) -> Result<ShadowRoot, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Element",
            feature = "ShadowRoot",
            feature = "ShadowRootInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_attach_shadow_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shadow_root_init_dict: <&ShadowRootInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ShadowRoot as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_attach_shadow_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shadow_root_init_dict: <&ShadowRootInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ShadowRoot as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(shadow_root_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shadow_root_init_dict =
                    <&ShadowRootInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        shadow_root_init_dict,
                    );
                __widl_f_attach_shadow_Element(self_, shadow_root_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ShadowRoot as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_closest_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `closest()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/closest)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn closest(&self, selector: &str) -> Result<Option<Element>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_closest_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selector: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_closest_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selector: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(selector);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selector = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selector);
                __widl_f_closest_Element(self_, selector)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_attribute_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `getAttribute()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn get_attribute(&self, name: &str) -> Option<String> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_attribute_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_attribute_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_attribute_Element(self_, name)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_attribute_ns_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `getAttributeNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNS)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn get_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<String> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_attribute_ns_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_attribute_ns_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_get_attribute_ns_Element(self_, namespace, local_name)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_attribute_names_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `getAttributeNames()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNames)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn get_attribute_names(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_attribute_names_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_attribute_names_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_attribute_names_Element(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Attr", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_attribute_node_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Attr", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `getAttributeNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNode)\n\n*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    #[allow(clippy::all)]
    pub fn get_attribute_node(&self, name: &str) -> Option<Attr> {
        #[cfg(all(feature = "Attr", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_attribute_node_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_attribute_node_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_attribute_node_Element(self_, name)
            };
            <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Attr", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_attribute_node_ns_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Attr", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `getAttributeNodeNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNodeNS)\n\n*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    #[allow(clippy::all)]
    pub fn get_attribute_node_ns(
        &self,
        namespace_uri: Option<&str>,
        local_name: &str,
    ) -> Option<Attr> {
        #[cfg(all(feature = "Attr", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_attribute_node_ns_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace_uri: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_attribute_node_ns_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace_uri: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace_uri =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace_uri);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_get_attribute_node_ns_Element(self_, namespace_uri, local_name)
            };
            <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRect", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_bounding_client_rect_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <DomRect as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "DomRect", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `getBoundingClientRect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect)\n\n*This API requires the following crate features to be activated: `DomRect`, `Element`*"]
    #[allow(clippy::all)]
    pub fn get_bounding_client_rect(&self) -> DomRect {
        #[cfg(all(feature = "DomRect", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_bounding_client_rect_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_bounding_client_rect_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_bounding_client_rect_Element(self_)
            };
            <DomRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRectList", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_client_rects_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <DomRectList as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "DomRectList", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `getClientRects()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getClientRects)\n\n*This API requires the following crate features to be activated: `DomRectList`, `Element`*"]
    #[allow(clippy::all)]
    pub fn get_client_rects(&self) -> DomRectList {
        #[cfg(all(feature = "DomRectList", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_client_rects_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRectList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_client_rects_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRectList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_client_rects_Element(self_)
            };
            <DomRectList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_elements_by_class_name_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `getElementsByClassName()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByClassName)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn get_elements_by_class_name(&self, class_names: &str) -> HtmlCollection {
        #[cfg(all(feature = "Element", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_elements_by_class_name_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                class_names: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_elements_by_class_name_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let class_names =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(class_names);
                __widl_f_get_elements_by_class_name_Element(self_, class_names)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_elements_by_tag_name_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `getElementsByTagName()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagName)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn get_elements_by_tag_name(&self, local_name: &str) -> HtmlCollection {
        #[cfg(all(feature = "Element", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_elements_by_tag_name_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_elements_by_tag_name_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_get_elements_by_tag_name_Element(self_, local_name)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_elements_by_tag_name_ns_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `getElementsByTagNameNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagNameNS)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn get_elements_by_tag_name_ns(
        &self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<HtmlCollection, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_elements_by_tag_name_ns_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_elements_by_tag_name_ns_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_get_elements_by_tag_name_ns_Element(self_, namespace, local_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_attribute_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `hasAttribute()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn has_attribute(&self, name: &str) -> bool {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_attribute_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_attribute_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_has_attribute_Element(self_, name)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_attribute_ns_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `hasAttributeNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributeNS)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn has_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> bool {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_attribute_ns_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_attribute_ns_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_has_attribute_ns_Element(self_, namespace, local_name)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_attributes_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `hasAttributes()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributes)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn has_attributes(&self) -> bool {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_attributes_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_attributes_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_has_attributes_Element(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_pointer_capture_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `hasPointerCapture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasPointerCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn has_pointer_capture(&self, pointer_id: i32) -> bool {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_pointer_capture_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pointer_id: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_pointer_capture_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pointer_id: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(pointer_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pointer_id = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pointer_id);
                __widl_f_has_pointer_capture_Element(self_, pointer_id)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_adjacent_element_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `insertAdjacentElement()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentElement)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn insert_adjacent_element(
        &self,
        where_: &str,
        element: &Element,
    ) -> Result<Option<Element>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_adjacent_element_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                where_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_adjacent_element_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            where_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(where_);
            drop(element);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let where_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(where_);
                let element = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                __widl_f_insert_adjacent_element_Element(self_, where_, element)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_adjacent_html_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `insertAdjacentHTML()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn insert_adjacent_html(
        &self,
        position: &str,
        text: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_adjacent_html_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                position: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_adjacent_html_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            position: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(position);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let position = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(position);
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_insert_adjacent_html_Element(self_, position, text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_adjacent_text_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `insertAdjacentText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentText)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn insert_adjacent_text(
        &self,
        where_: &str,
        data: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_adjacent_text_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                where_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_adjacent_text_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            where_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(where_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let where_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(where_);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_insert_adjacent_text_Element(self_, where_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_matches_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `matches()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/matches)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn matches(&self, selector: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_matches_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selector: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_matches_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selector: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(selector);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selector = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selector);
                __widl_f_matches_Element(self_, selector)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_selector_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `querySelector()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn query_selector(
        &self,
        selectors: &str,
    ) -> Result<Option<Element>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_selector_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selectors: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_selector_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selectors = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selectors);
                __widl_f_query_selector_Element(self_, selectors)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_selector_all_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `querySelectorAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll)\n\n*This API requires the following crate features to be activated: `Element`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn query_selector_all(&self, selectors: &str) -> Result<NodeList, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_selector_all_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selectors: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_selector_all_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selectors = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selectors);
                __widl_f_query_selector_all_Element(self_, selectors)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<NodeList as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_release_capture_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `releaseCapture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/releaseCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn release_capture(&self) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_release_capture_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_release_capture_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_release_capture_Element(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_release_pointer_capture_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `releasePointerCapture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/releasePointerCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn release_pointer_capture(&self, pointer_id: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_release_pointer_capture_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pointer_id: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_release_pointer_capture_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pointer_id: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(pointer_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pointer_id = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pointer_id);
                __widl_f_release_pointer_capture_Element(self_, pointer_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_attribute_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `removeAttribute()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn remove_attribute(&self, name: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_attribute_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_attribute_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_remove_attribute_Element(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_attribute_ns_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `removeAttributeNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNS)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn remove_attribute_ns(
        &self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_attribute_ns_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_attribute_ns_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_remove_attribute_ns_Element(self_, namespace, local_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Attr", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_attribute_node_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&Attr as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Attr", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `removeAttributeNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNode)\n\n*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    #[allow(clippy::all)]
    pub fn remove_attribute_node(
        &self,
        old_attr: &Attr,
    ) -> Result<Option<Attr>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Attr", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_attribute_node_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                old_attr: <&Attr as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_attribute_node_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            old_attr: <&Attr as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(old_attr);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let old_attr = <&Attr as wasm_bindgen::convert::IntoWasmAbi>::into_abi(old_attr);
                __widl_f_remove_attribute_node_Element(self_, old_attr)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_fullscreen_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `requestFullscreen()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestFullscreen)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn request_fullscreen(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_fullscreen_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_fullscreen_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_request_fullscreen_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_pointer_lock_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `requestPointerLock()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestPointerLock)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn request_pointer_lock(&self) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_pointer_lock_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_pointer_lock_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_request_pointer_lock_Element(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_with_x_and_y_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scroll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll_with_x_and_y(&self, x: f64, y: f64) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_with_x_and_y_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_with_x_and_y_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_scroll_with_x_and_y_Element(self_, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scroll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll(&self) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scroll_Element(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element", feature = "ScrollToOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_with_scroll_to_options_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&ScrollToOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "ScrollToOptions",))]
    #[allow(bad_style)]
    #[doc = "The `scroll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
    #[allow(clippy::all)]
    pub fn scroll_with_scroll_to_options(&self, options: &ScrollToOptions) {
        #[cfg(all(feature = "Element", feature = "ScrollToOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_with_scroll_to_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ScrollToOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_with_scroll_to_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ScrollToOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&ScrollToOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_scroll_with_scroll_to_options_Element(self_, options)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_by_with_x_and_y_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollBy()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll_by_with_x_and_y(&self, x: f64, y: f64) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_by_with_x_and_y_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_by_with_x_and_y_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_scroll_by_with_x_and_y_Element(self_, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_by_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollBy()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll_by(&self) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_by_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_by_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scroll_by_Element(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element", feature = "ScrollToOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_by_with_scroll_to_options_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&ScrollToOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "ScrollToOptions",))]
    #[allow(bad_style)]
    #[doc = "The `scrollBy()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
    #[allow(clippy::all)]
    pub fn scroll_by_with_scroll_to_options(&self, options: &ScrollToOptions) {
        #[cfg(all(feature = "Element", feature = "ScrollToOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_by_with_scroll_to_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ScrollToOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_by_with_scroll_to_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ScrollToOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&ScrollToOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_scroll_by_with_scroll_to_options_Element(self_, options)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_into_view_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollIntoView()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll_into_view(&self) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_into_view_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_into_view_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scroll_into_view_Element(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_into_view_with_bool_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollIntoView()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll_into_view_with_bool(&self, arg: bool) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_into_view_with_bool_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_into_view_with_bool_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let arg = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(arg);
                __widl_f_scroll_into_view_with_bool_Element(self_, arg)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element", feature = "ScrollIntoViewOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_into_view_with_scroll_into_view_options_Element(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&ScrollIntoViewOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "ScrollIntoViewOptions",))]
    #[allow(bad_style)]
    #[doc = "The `scrollIntoView()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollIntoViewOptions`*"]
    #[allow(clippy::all)]
    pub fn scroll_into_view_with_scroll_into_view_options(&self, arg: &ScrollIntoViewOptions) {
        #[cfg(all(feature = "Element", feature = "ScrollIntoViewOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_into_view_with_scroll_into_view_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arg: <&ScrollIntoViewOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_into_view_with_scroll_into_view_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arg: <&ScrollIntoViewOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let arg =
                    <&ScrollIntoViewOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(arg);
                __widl_f_scroll_into_view_with_scroll_into_view_options_Element(self_, arg)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_to_with_x_and_y_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll_to_with_x_and_y(&self, x: f64, y: f64) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_to_with_x_and_y_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_to_with_x_and_y_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_scroll_to_with_x_and_y_Element(self_, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_to_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll_to(&self) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_to_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_to_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scroll_to_Element(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element", feature = "ScrollToOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_to_with_scroll_to_options_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&ScrollToOptions as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "ScrollToOptions",))]
    #[allow(bad_style)]
    #[doc = "The `scrollTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
    #[allow(clippy::all)]
    pub fn scroll_to_with_scroll_to_options(&self, options: &ScrollToOptions) {
        #[cfg(all(feature = "Element", feature = "ScrollToOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_to_with_scroll_to_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ScrollToOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_to_with_scroll_to_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ScrollToOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&ScrollToOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_scroll_to_with_scroll_to_options_Element(self_, options)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_attribute_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `setAttribute()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_attribute(&self, name: &str, value: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_attribute_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_attribute_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_attribute_Element(self_, name, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_attribute_ns_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `setAttributeNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNS)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_attribute_ns(
        &self,
        namespace: Option<&str>,
        name: &str,
        value: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_attribute_ns_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_attribute_ns_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(namespace);
            drop(name);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_attribute_ns_Element(self_, namespace, name, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Attr", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_attribute_node_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&Attr as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Attr", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `setAttributeNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNode)\n\n*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    #[allow(clippy::all)]
    pub fn set_attribute_node(
        &self,
        new_attr: &Attr,
    ) -> Result<Option<Attr>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Attr", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_attribute_node_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_attr: <&Attr as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_attribute_node_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_attr: <&Attr as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(new_attr);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_attr = <&Attr as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_attr);
                __widl_f_set_attribute_node_Element(self_, new_attr)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Attr", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_attribute_node_ns_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&Attr as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Attr", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `setAttributeNodeNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNodeNS)\n\n*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    #[allow(clippy::all)]
    pub fn set_attribute_node_ns(
        &self,
        new_attr: &Attr,
    ) -> Result<Option<Attr>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Attr", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_attribute_node_ns_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_attr: <&Attr as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_attribute_node_ns_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_attr: <&Attr as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(new_attr);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let new_attr = <&Attr as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_attr);
                __widl_f_set_attribute_node_ns_Element(self_, new_attr)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_capture_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `setCapture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_capture(&self) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_capture_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_capture_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_set_capture_Element(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_capture_with_retarget_to_element_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `setCapture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_capture_with_retarget_to_element(&self, retarget_to_element: bool) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_capture_with_retarget_to_element_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                retarget_to_element: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_capture_with_retarget_to_element_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            retarget_to_element: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(retarget_to_element);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let retarget_to_element =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(retarget_to_element);
                __widl_f_set_capture_with_retarget_to_element_Element(self_, retarget_to_element)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_pointer_capture_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `setPointerCapture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_pointer_capture(&self, pointer_id: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_pointer_capture_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pointer_id: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_pointer_capture_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pointer_id: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(pointer_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pointer_id = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pointer_id);
                __widl_f_set_pointer_capture_Element(self_, pointer_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_toggle_attribute_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `toggleAttribute()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn toggle_attribute(&self, name: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_toggle_attribute_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_toggle_attribute_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_toggle_attribute_Element(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_toggle_attribute_with_force_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `toggleAttribute()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn toggle_attribute_with_force(
        &self,
        name: &str,
        force: bool,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_toggle_attribute_with_force_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                force: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_toggle_attribute_with_force_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            force: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(force);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let force = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(force);
                __widl_f_toggle_attribute_with_force_Element(self_, name, force)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_webkit_matches_selector_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `webkitMatchesSelector()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/webkitMatchesSelector)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn webkit_matches_selector(&self, selector: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_webkit_matches_selector_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selector: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_webkit_matches_selector_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selector: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(selector);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selector = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selector);
                __widl_f_webkit_matches_selector_Element(self_, selector)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_namespace_uri_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `namespaceURI` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn namespace_uri(&self) -> Option<String> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_namespace_uri_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_namespace_uri_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_namespace_uri_Element(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prefix_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prefix` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prefix)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn prefix(&self) -> Option<String> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prefix_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prefix_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prefix_Element(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_local_name_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `localName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/localName)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn local_name(&self) -> String {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_local_name_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_local_name_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_local_name_Element(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tag_name_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `tagName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn tag_name(&self) -> String {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tag_name_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tag_name_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_tag_name_Element(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_Element(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_id_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `id` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_id(&self, id: &str) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_id_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_id_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(id);
                __widl_f_set_id_Element(self_, id)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_class_name_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `className` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn class_name(&self) -> String {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_class_name_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_class_name_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_class_name_Element(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_class_name_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `className` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_class_name(&self, class_name: &str) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_class_name_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                class_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_class_name_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            class_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(class_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let class_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(class_name);
                __widl_f_set_class_name_Element(self_, class_name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomTokenList", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_class_list_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <DomTokenList as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "DomTokenList", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `classList` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `Element`*"]
    #[allow(clippy::all)]
    pub fn class_list(&self) -> DomTokenList {
        #[cfg(all(feature = "DomTokenList", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_class_list_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_class_list_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_class_list_Element(self_)
            };
            <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "NamedNodeMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_attributes_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <NamedNodeMap as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "NamedNodeMap",))]
    #[allow(bad_style)]
    #[doc = "The `attributes` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attributes)\n\n*This API requires the following crate features to be activated: `Element`, `NamedNodeMap`*"]
    #[allow(clippy::all)]
    pub fn attributes(&self) -> NamedNodeMap {
        #[cfg(all(feature = "Element", feature = "NamedNodeMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_attributes_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NamedNodeMap as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_attributes_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NamedNodeMap as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_attributes_Element(self_)
            };
            <NamedNodeMap as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_top_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollTop` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll_top(&self) -> i32 {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_top_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_top_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scroll_top_Element(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_scroll_top_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollTop` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_scroll_top(&self, scroll_top: i32) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_scroll_top_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scroll_top: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_scroll_top_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scroll_top: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(scroll_top);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scroll_top = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scroll_top);
                __widl_f_set_scroll_top_Element(self_, scroll_top)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_left_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollLeft` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll_left(&self) -> i32 {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_left_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_left_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scroll_left_Element(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_scroll_left_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollLeft` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_scroll_left(&self, scroll_left: i32) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_scroll_left_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scroll_left: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_scroll_left_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scroll_left: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(scroll_left);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scroll_left =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scroll_left);
                __widl_f_set_scroll_left_Element(self_, scroll_left)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_width_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollWidth)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll_width(&self) -> i32 {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_width_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_width_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scroll_width_Element(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_height_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `scrollHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollHeight)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn scroll_height(&self) -> i32 {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_height_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_height_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scroll_height_Element(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_client_top_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `clientTop` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientTop)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn client_top(&self) -> i32 {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_client_top_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_client_top_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_client_top_Element(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_client_left_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `clientLeft` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientLeft)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn client_left(&self) -> i32 {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_client_left_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_client_left_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_client_left_Element(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_client_width_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `clientWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientWidth)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn client_width(&self) -> i32 {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_client_width_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_client_width_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_client_width_Element(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_client_height_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `clientHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientHeight)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn client_height(&self) -> i32 {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_client_height_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_client_height_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_client_height_Element(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_inner_html_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `innerHTML` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn inner_html(&self) -> String {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_inner_html_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_inner_html_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_inner_html_Element(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_inner_html_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `innerHTML` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_inner_html(&self, inner_html: &str) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_inner_html_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                inner_html: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_inner_html_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            inner_html: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(inner_html);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let inner_html = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(inner_html);
                __widl_f_set_inner_html_Element(self_, inner_html)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_outer_html_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `outerHTML` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn outer_html(&self) -> String {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_outer_html_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_outer_html_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_outer_html_Element(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_outer_html_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `outerHTML` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_outer_html(&self, outer_html: &str) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_outer_html_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                outer_html: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_outer_html_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            outer_html: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(outer_html);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let outer_html = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(outer_html);
                __widl_f_set_outer_html_Element(self_, outer_html)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element", feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shadow_root_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <Option<ShadowRoot> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `shadowRoot` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn shadow_root(&self) -> Option<ShadowRoot> {
        #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shadow_root_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<ShadowRoot> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shadow_root_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<ShadowRoot> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_shadow_root_Element(self_)
            };
            <Option<ShadowRoot> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "HtmlSlotElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_assigned_slot_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <Option<HtmlSlotElement> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "HtmlSlotElement",))]
    #[allow(bad_style)]
    #[doc = "The `assignedSlot` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/assignedSlot)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlSlotElement`*"]
    #[allow(clippy::all)]
    pub fn assigned_slot(&self) -> Option<HtmlSlotElement> {
        #[cfg(all(feature = "Element", feature = "HtmlSlotElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assigned_slot_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlSlotElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assigned_slot_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<HtmlSlotElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_assigned_slot_Element(self_)
            };
            <Option<HtmlSlotElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_slot_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `slot` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn slot(&self) -> String {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_slot_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_slot_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_slot_Element(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_slot_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `slot` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn set_slot(&self, slot: &str) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_slot_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                slot: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_slot_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            slot: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(slot);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let slot = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(slot);
                __widl_f_set_slot_Element(self_, slot)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn after_with_node(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_after_with_node_Element(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_0_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_0_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_0_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_after_with_node_0_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_1_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_1_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_1_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_after_with_node_1_Element(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_2_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_2_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_2_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_after_with_node_2_Element(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_3_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_3_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_3_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_after_with_node_3_Element(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_4_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_4_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_4_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_after_with_node_4_Element(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_5_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn after_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_5_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_5_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_after_with_node_5_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_6_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_6_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_node_6_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_after_with_node_6_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_node_7_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_node_7_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_after_with_node_7_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_after_with_node_7_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn after_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_after_with_str_Element(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_0_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_0_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_0_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_after_with_str_0_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_1_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_1_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_1_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_after_with_str_1_Element(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_2_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_2_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_2_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_after_with_str_2_Element(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_3_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_3_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_3_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_after_with_str_3_Element(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_4_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_4_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_4_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_after_with_str_4_Element(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_5_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn after_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_5_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_5_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_after_with_str_5_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_6_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_6_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_after_with_str_6_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_after_with_str_6_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_after_with_str_7_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `after()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_after_with_str_7_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_after_with_str_7_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_after_with_str_7_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn before_with_node(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_before_with_node_Element(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_0_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_0_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_0_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_before_with_node_0_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_1_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_1_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_1_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_before_with_node_1_Element(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_2_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_2_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_2_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_before_with_node_2_Element(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_3_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_3_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_3_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_before_with_node_3_Element(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_4_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_4_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_4_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_before_with_node_4_Element(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_5_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn before_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_5_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_5_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_before_with_node_5_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_6_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_6_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_node_6_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_before_with_node_6_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_node_7_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_node_7_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_before_with_node_7_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_before_with_node_7_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn before_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_before_with_str_Element(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_0_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_0_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_0_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_before_with_str_0_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_1_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_1_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_1_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_before_with_str_1_Element(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_2_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_2_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_2_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_before_with_str_2_Element(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_3_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_3_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_3_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_before_with_str_3_Element(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_4_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_4_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_4_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_before_with_str_4_Element(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_5_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn before_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_5_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_5_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_before_with_str_5_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_6_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_6_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_before_with_str_6_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_before_with_str_6_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_before_with_str_7_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `before()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_before_with_str_7_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_before_with_str_7_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_before_with_str_7_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/remove)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn remove(&self) {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_remove_Element(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node(
        &self,
        nodes: &::js_sys::Array,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_replace_with_with_node_Element(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_0_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_0_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_0_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_replace_with_with_node_0_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_1_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_1_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_1_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_replace_with_with_node_1_Element(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_2_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_2_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_2_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_replace_with_with_node_2_Element(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_3_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_3_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_3_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_replace_with_with_node_3_Element(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_4_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_4_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_4_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_replace_with_with_node_4_Element(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_5_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_5_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_5_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_replace_with_with_node_5_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_6_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_6_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_node_6_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_replace_with_with_node_6_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_node_7_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_node_7_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_replace_with_with_node_7_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_replace_with_with_node_7_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str(
        &self,
        nodes: &::js_sys::Array,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_replace_with_with_str_Element(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_0_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_0_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_0_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_replace_with_with_str_0_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_1_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_1_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_1_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_replace_with_with_str_1_Element(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_2_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_2_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_2_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_replace_with_with_str_2_Element(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_3_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_3_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_3_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_replace_with_with_str_3_Element(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_4_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_4_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_4_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_replace_with_with_str_4_Element(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_5_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn replace_with_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_5_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_5_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_replace_with_with_str_5_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_6_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_6_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_with_with_str_6_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_replace_with_with_str_6_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_with_with_str_7_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `replaceWith()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_with_with_str_7_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_replace_with_with_str_7_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_replace_with_with_str_7_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Element",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_text_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_text(
        &self,
        point: &DomPointInit,
        from: &Text,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Element",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_text_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_text_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_point_from_node_with_text_Element(self_, point, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_element_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_element(
        &self,
        point: &DomPointInit,
        from: &Element,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_element_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_element_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_point_from_node_with_element_Element(self_, point, from)
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
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_document_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_document(
        &self,
        point: &DomPointInit,
        from: &Document,
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
            fn __widl_f_convert_point_from_node_with_document_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_document_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_point_from_node_with_document_Element(self_, point, from)
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
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Element",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_text_and_options_Element(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_text_and_options(
        &self,
        point: &DomPointInit,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Element",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_text_and_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_text_and_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_point_from_node_with_text_and_options_Element(
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
    feature = "DomPoint",
    feature = "DomPointInit",
    feature = "Element",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_element_and_options_Element(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_point_from_node_with_element_and_options(
        &self,
        point: &DomPointInit,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomPoint",
            feature = "DomPointInit",
            feature = "Element",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_element_and_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_element_and_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_point_from_node_with_element_and_options_Element(
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
pub extern "C" fn __wbindgen_describe___widl_f_convert_point_from_node_with_document_and_options_Element(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertPointFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
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
            feature = "Element",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_point_from_node_with_document_and_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_point_from_node_with_document_and_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_point_from_node_with_document_and_options_Element(
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
#[cfg(all(feature = "DomQuad", feature = "Element", feature = "Text",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_text_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "DomQuad", feature = "Element", feature = "Text",))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_text(
        &self,
        quad: &DomQuad,
        from: &Text,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomQuad", feature = "Element", feature = "Text",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_text_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_text_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_quad_from_node_with_text_Element(self_, quad, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomQuad", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_element_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "DomQuad", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_element(
        &self,
        quad: &DomQuad,
        from: &Element,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomQuad", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_element_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_element_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_quad_from_node_with_element_Element(self_, quad, from)
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
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_document_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Document", feature = "DomQuad", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_document(
        &self,
        quad: &DomQuad,
        from: &Document,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Document", feature = "DomQuad", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_document_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_document_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_quad_from_node_with_document_Element(self_, quad, from)
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
    feature = "DomQuad",
    feature = "Element",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_text_and_options_Element(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Element",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_text_and_options(
        &self,
        quad: &DomQuad,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomQuad",
            feature = "Element",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_text_and_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_text_and_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_quad_from_node_with_text_and_options_Element(
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
    feature = "DomQuad",
    feature = "Element",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_element_and_options_Element(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_quad_from_node_with_element_and_options(
        &self,
        quad: &DomQuad,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomQuad",
            feature = "Element",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_element_and_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_element_and_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_quad_from_node_with_element_and_options_Element(
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
pub extern "C" fn __wbindgen_describe___widl_f_convert_quad_from_node_with_document_and_options_Element(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&DomQuad as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertQuadFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Element`*"]
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
            feature = "Element",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_quad_from_node_with_document_and_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                quad: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_quad_from_node_with_document_and_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let quad = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(quad);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_quad_from_node_with_document_and_options_Element(
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
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Element",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_text_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_text(
        &self,
        rect: &DomRectReadOnly,
        from: &Text,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Element",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_text_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_text_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_rect_from_node_with_text_Element(self_, rect, from)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_element_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_element(
        &self,
        rect: &DomRectReadOnly,
        from: &Element,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_element_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_element_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_rect_from_node_with_element_Element(self_, rect, from)
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
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_document_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_document(
        &self,
        rect: &DomRectReadOnly,
        from: &Document,
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
            fn __widl_f_convert_rect_from_node_with_document_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_document_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                __widl_f_convert_rect_from_node_with_document_Element(self_, rect, from)
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
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Element",
    feature = "Text",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_text_and_options_Element(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Text as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
        feature = "Text",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_text_and_options(
        &self,
        rect: &DomRectReadOnly,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Element",
            feature = "Text",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_text_and_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Text as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_text_and_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Text as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_rect_from_node_with_text_and_options_Element(
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
    feature = "DomQuad",
    feature = "DomRectReadOnly",
    feature = "Element",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_element_and_options_Element(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Element as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
    #[allow(clippy::all)]
    pub fn convert_rect_from_node_with_element_and_options(
        &self,
        rect: &DomRectReadOnly,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ConvertCoordinateOptions",
            feature = "DomQuad",
            feature = "DomRectReadOnly",
            feature = "Element",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_element_and_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_element_and_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_rect_from_node_with_element_and_options_Element(
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
pub extern "C" fn __wbindgen_describe___widl_f_convert_rect_from_node_with_document_and_options_Element(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&DomRectReadOnly as WasmDescribe>::describe();
    <&Document as WasmDescribe>::describe();
    <&ConvertCoordinateOptions as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
    ))]
    #[allow(bad_style)]
    #[doc = "The `convertRectFromNode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
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
            feature = "Element",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_rect_from_node_with_document_and_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                from: <&Document as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_rect_from_node_with_document_and_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                let from = <&Document as wasm_bindgen::convert::IntoWasmAbi>::into_abi(from);
                let options =
                    <&ConvertCoordinateOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_convert_rect_from_node_with_document_and_options_Element(
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
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_box_quads_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `getBoxQuads()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn get_box_quads(&self) -> Result<::js_sys::Array, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_box_quads_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_box_quads_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_box_quads_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "BoxQuadOptions", feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_box_quads_with_options_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&BoxQuadOptions as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "BoxQuadOptions", feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `getBoxQuads()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)\n\n*This API requires the following crate features to be activated: `BoxQuadOptions`, `Element`*"]
    #[allow(clippy::all)]
    pub fn get_box_quads_with_options(
        &self,
        options: &BoxQuadOptions,
    ) -> Result<::js_sys::Array, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BoxQuadOptions", feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_box_quads_with_options_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&BoxQuadOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_box_quads_with_options_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&BoxQuadOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_get_box_quads_with_options_Element(self_, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_previous_element_sibling_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `previousElementSibling` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/previousElementSibling)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn previous_element_sibling(&self) -> Option<Element> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_previous_element_sibling_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_previous_element_sibling_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_previous_element_sibling_Element(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_next_element_sibling_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `nextElementSibling` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/nextElementSibling)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn next_element_sibling(&self) -> Option<Element> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_next_element_sibling_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_next_element_sibling_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_next_element_sibling_Element(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn append_with_node(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_append_with_node_Element(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_0_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_0_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_0_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_append_with_node_0_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_1_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_1_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_1_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_append_with_node_1_Element(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_2_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_2_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_2_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_append_with_node_2_Element(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_3_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_3_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_3_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_append_with_node_3_Element(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_4_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_4_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_4_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_append_with_node_4_Element(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_5_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn append_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_5_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_5_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_append_with_node_5_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_6_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_6_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_node_6_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_append_with_node_6_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_node_7_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_node_7_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_append_with_node_7_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_append_with_node_7_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn append_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_append_with_str_Element(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_0_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_0_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_0_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_append_with_str_0_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_1_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_1_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_1_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_append_with_str_1_Element(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_2_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_2_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_2_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_append_with_str_2_Element(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_3_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_3_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_3_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_append_with_str_3_Element(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_4_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_4_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_4_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_append_with_str_4_Element(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_5_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn append_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_5_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_5_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_append_with_str_5_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_6_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_6_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_6_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_append_with_str_6_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_7_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_7_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_append_with_str_7_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_append_with_str_7_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node(
        &self,
        nodes: &::js_sys::Array,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_prepend_with_node_Element(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_0_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_0_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_0_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prepend_with_node_0_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_1_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_1(&self, nodes_1: &Node) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_1_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_1_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_prepend_with_node_1_Element(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_2_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_2(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_2_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_2_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_prepend_with_node_2_Element(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_3_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_3(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_3_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_3_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_prepend_with_node_3_Element(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_4_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_4(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_4_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_4_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_prepend_with_node_4_Element(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_5_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_node_5(
        &self,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_5_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_5_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_prepend_with_node_5_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_6_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_6_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&Node as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_node_6_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_prepend_with_node_6_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_node_7_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Element as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <&Node as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
        #[cfg(all(feature = "Element", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_node_7_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_prepend_with_node_7_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&Node as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_prepend_with_node_7_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str(&self, nodes: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes);
                __widl_f_prepend_with_str_Element(self_, nodes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_0_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_0_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_0_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prepend_with_str_0_Element(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_1_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_1(&self, nodes_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_1_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_1_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                __widl_f_prepend_with_str_1_Element(self_, nodes_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_2_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_2(
        &self,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_2_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_2_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                __widl_f_prepend_with_str_2_Element(self_, nodes_1, nodes_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_3_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_3(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_3_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_3_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                __widl_f_prepend_with_str_3_Element(self_, nodes_1, nodes_2, nodes_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_4_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_4(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_4_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_4_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                __widl_f_prepend_with_str_4_Element(self_, nodes_1, nodes_2, nodes_3, nodes_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_5_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn prepend_with_str_5(
        &self,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_5_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_5_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                __widl_f_prepend_with_str_5_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_6_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_6_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nodes_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prepend_with_str_6_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                __widl_f_prepend_with_str_6_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prepend_with_str_7_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&Element as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `prepend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prepend_with_str_7_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
        unsafe fn __widl_f_prepend_with_str_7_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let nodes_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_1);
                let nodes_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_2);
                let nodes_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_3);
                let nodes_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_4);
                let nodes_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_5);
                let nodes_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_6);
                let nodes_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nodes_7);
                __widl_f_prepend_with_str_7_Element(
                    self_, nodes_1, nodes_2, nodes_3, nodes_4, nodes_5, nodes_6, nodes_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "HtmlCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_children_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element", feature = "HtmlCollection",))]
    #[allow(bad_style)]
    #[doc = "The `children` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/children)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    #[allow(clippy::all)]
    pub fn children(&self) -> HtmlCollection {
        #[cfg(all(feature = "Element", feature = "HtmlCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_children_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_children_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_children_Element(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_first_element_child_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `firstElementChild` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/firstElementChild)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn first_element_child(&self) -> Option<Element> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_first_element_child_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_first_element_child_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_first_element_child_Element(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_last_element_child_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `lastElementChild` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/lastElementChild)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn last_element_child(&self) -> Option<Element> {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_last_element_child_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_last_element_child_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_last_element_child_Element(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_child_element_count_Element() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Element as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl Element {
    #[cfg(all(feature = "Element",))]
    #[allow(bad_style)]
    #[doc = "The `childElementCount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/childElementCount)\n\n*This API requires the following crate features to be activated: `Element`*"]
    #[allow(clippy::all)]
    pub fn child_element_count(&self) -> u32 {
        #[cfg(all(feature = "Element",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_child_element_count_Element(
                self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_child_element_count_Element(
            self_: <&Element as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Element as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_child_element_count_Element(self_)
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
pub static __WASM_BINDGEN_GENERATED_e31980b199a2be22: [u8; 18303usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}=G\0\0\0\0\xC5\x01\0\0\x02\x07Element\x19__widl_instanceof_Element\0\0\0\0\x1E__widl_f_attach_shadow_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x15shadow_root_init_dict\x0CattachShadow\0\0\0\x18__widl_f_closest_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x08selector\x07closest\0\0\0\x1E__widl_f_get_attribute_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x04name\x0CgetAttribute\0\0\0!__widl_f_get_attribute_ns_Element\0\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\tnamespace\nlocal_name\x0EgetAttributeNS\0\0\0$__widl_f_get_attribute_names_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x11getAttributeNames\0\0\0#__widl_f_get_attribute_node_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x04name\x10getAttributeNode\0\0\0&__widl_f_get_attribute_node_ns_Element\0\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\rnamespace_uri\nlocal_name\x12getAttributeNodeNS\0\0\0)__widl_f_get_bounding_client_rect_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x15getBoundingClientRect\0\0\0!__widl_f_get_client_rects_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x0EgetClientRects\0\0\0+__widl_f_get_elements_by_class_name_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x0Bclass_names\x16getElementsByClassName\0\0\0)__widl_f_get_elements_by_tag_name_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\nlocal_name\x14getElementsByTagName\0\0\0,__widl_f_get_elements_by_tag_name_ns_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\tnamespace\nlocal_name\x16getElementsByTagNameNS\0\0\0\x1E__widl_f_has_attribute_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x04name\x0ChasAttribute\0\0\0!__widl_f_has_attribute_ns_Element\0\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\tnamespace\nlocal_name\x0EhasAttributeNS\0\0\0\x1F__widl_f_has_attributes_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\rhasAttributes\0\0\0$__widl_f_has_pointer_capture_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\npointer_id\x11hasPointerCapture\0\0\0(__widl_f_insert_adjacent_element_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x06where_\x07element\x15insertAdjacentElement\0\0\0%__widl_f_insert_adjacent_html_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x08position\x04text\x12insertAdjacentHTML\0\0\0%__widl_f_insert_adjacent_text_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x06where_\x04data\x12insertAdjacentText\0\0\0\x18__widl_f_matches_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x08selector\x07matches\0\0\0\x1F__widl_f_query_selector_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\tselectors\rquerySelector\0\0\0#__widl_f_query_selector_all_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\tselectors\x10querySelectorAll\0\0\0 __widl_f_release_capture_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x0EreleaseCapture\0\0\0(__widl_f_release_pointer_capture_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\npointer_id\x15releasePointerCapture\0\0\0!__widl_f_remove_attribute_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x04name\x0FremoveAttribute\0\0\0$__widl_f_remove_attribute_ns_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\tnamespace\nlocal_name\x11removeAttributeNS\0\0\0&__widl_f_remove_attribute_node_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x08old_attr\x13removeAttributeNode\0\0\0#__widl_f_request_fullscreen_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x11requestFullscreen\0\0\0%__widl_f_request_pointer_lock_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x12requestPointerLock\0\0\0$__widl_f_scroll_with_x_and_y_Element\0\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x01x\x01y\x06scroll\0\0\0\x17__widl_f_scroll_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x06scroll\0\0\0.__widl_f_scroll_with_scroll_to_options_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07options\x06scroll\0\0\0'__widl_f_scroll_by_with_x_and_y_Element\0\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x01x\x01y\x08scrollBy\0\0\0\x1A__widl_f_scroll_by_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x08scrollBy\0\0\01__widl_f_scroll_by_with_scroll_to_options_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07options\x08scrollBy\0\0\0!__widl_f_scroll_into_view_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x0EscrollIntoView\0\0\0+__widl_f_scroll_into_view_with_bool_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x03arg\x0EscrollIntoView\0\0\0?__widl_f_scroll_into_view_with_scroll_into_view_options_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x03arg\x0EscrollIntoView\0\0\0'__widl_f_scroll_to_with_x_and_y_Element\0\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x01x\x01y\x08scrollTo\0\0\0\x1A__widl_f_scroll_to_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x08scrollTo\0\0\01__widl_f_scroll_to_with_scroll_to_options_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07options\x08scrollTo\0\0\0\x1E__widl_f_set_attribute_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x04name\x05value\x0CsetAttribute\0\0\0!__widl_f_set_attribute_ns_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\tnamespace\x04name\x05value\x0EsetAttributeNS\0\0\0#__widl_f_set_attribute_node_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x08new_attr\x10setAttributeNode\0\0\0&__widl_f_set_attribute_node_ns_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x08new_attr\x12setAttributeNodeNS\0\0\0\x1C__widl_f_set_capture_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\nsetCapture\0\0\05__widl_f_set_capture_with_retarget_to_element_Element\0\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x13retarget_to_element\nsetCapture\0\0\0$__widl_f_set_pointer_capture_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\npointer_id\x11setPointerCapture\0\0\0!__widl_f_toggle_attribute_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x04name\x0FtoggleAttribute\0\0\0,__widl_f_toggle_attribute_with_force_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x04name\x05force\x0FtoggleAttribute\0\0\0(__widl_f_webkit_matches_selector_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x08selector\x15webkitMatchesSelector\0\0\0\x1E__widl_f_namespace_uri_Element\0\0\0\x01\x07Element\x01\0\x01\x0CnamespaceURI\x01\x01\x05self_\x0CnamespaceURI\0\0\0\x17__widl_f_prefix_Element\0\0\0\x01\x07Element\x01\0\x01\x06prefix\x01\x01\x05self_\x06prefix\0\0\0\x1B__widl_f_local_name_Element\0\0\0\x01\x07Element\x01\0\x01\tlocalName\x01\x01\x05self_\tlocalName\0\0\0\x19__widl_f_tag_name_Element\0\0\0\x01\x07Element\x01\0\x01\x07tagName\x01\x01\x05self_\x07tagName\0\0\0\x13__widl_f_id_Element\0\0\0\x01\x07Element\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0\x17__widl_f_set_id_Element\0\0\0\x01\x07Element\x01\0\x02\x02id\x01\x02\x05self_\x02id\x02id\0\0\0\x1B__widl_f_class_name_Element\0\0\0\x01\x07Element\x01\0\x01\tclassName\x01\x01\x05self_\tclassName\0\0\0\x1F__widl_f_set_class_name_Element\0\0\0\x01\x07Element\x01\0\x02\tclassName\x01\x02\x05self_\nclass_name\tclassName\0\0\0\x1B__widl_f_class_list_Element\0\0\0\x01\x07Element\x01\0\x01\tclassList\x01\x01\x05self_\tclassList\0\0\0\x1B__widl_f_attributes_Element\0\0\0\x01\x07Element\x01\0\x01\nattributes\x01\x01\x05self_\nattributes\0\0\0\x1B__widl_f_scroll_top_Element\0\0\0\x01\x07Element\x01\0\x01\tscrollTop\x01\x01\x05self_\tscrollTop\0\0\0\x1F__widl_f_set_scroll_top_Element\0\0\0\x01\x07Element\x01\0\x02\tscrollTop\x01\x02\x05self_\nscroll_top\tscrollTop\0\0\0\x1C__widl_f_scroll_left_Element\0\0\0\x01\x07Element\x01\0\x01\nscrollLeft\x01\x01\x05self_\nscrollLeft\0\0\0 __widl_f_set_scroll_left_Element\0\0\0\x01\x07Element\x01\0\x02\nscrollLeft\x01\x02\x05self_\x0Bscroll_left\nscrollLeft\0\0\0\x1D__widl_f_scroll_width_Element\0\0\0\x01\x07Element\x01\0\x01\x0BscrollWidth\x01\x01\x05self_\x0BscrollWidth\0\0\0\x1E__widl_f_scroll_height_Element\0\0\0\x01\x07Element\x01\0\x01\x0CscrollHeight\x01\x01\x05self_\x0CscrollHeight\0\0\0\x1B__widl_f_client_top_Element\0\0\0\x01\x07Element\x01\0\x01\tclientTop\x01\x01\x05self_\tclientTop\0\0\0\x1C__widl_f_client_left_Element\0\0\0\x01\x07Element\x01\0\x01\nclientLeft\x01\x01\x05self_\nclientLeft\0\0\0\x1D__widl_f_client_width_Element\0\0\0\x01\x07Element\x01\0\x01\x0BclientWidth\x01\x01\x05self_\x0BclientWidth\0\0\0\x1E__widl_f_client_height_Element\0\0\0\x01\x07Element\x01\0\x01\x0CclientHeight\x01\x01\x05self_\x0CclientHeight\0\0\0\x1B__widl_f_inner_html_Element\0\0\0\x01\x07Element\x01\0\x01\tinnerHTML\x01\x01\x05self_\tinnerHTML\0\0\0\x1F__widl_f_set_inner_html_Element\0\0\0\x01\x07Element\x01\0\x02\tinnerHTML\x01\x02\x05self_\ninner_html\tinnerHTML\0\0\0\x1B__widl_f_outer_html_Element\0\0\0\x01\x07Element\x01\0\x01\touterHTML\x01\x01\x05self_\touterHTML\0\0\0\x1F__widl_f_set_outer_html_Element\0\0\0\x01\x07Element\x01\0\x02\touterHTML\x01\x02\x05self_\nouter_html\touterHTML\0\0\0\x1C__widl_f_shadow_root_Element\0\0\0\x01\x07Element\x01\0\x01\nshadowRoot\x01\x01\x05self_\nshadowRoot\0\0\0\x1E__widl_f_assigned_slot_Element\0\0\0\x01\x07Element\x01\0\x01\x0CassignedSlot\x01\x01\x05self_\x0CassignedSlot\0\0\0\x15__widl_f_slot_Element\0\0\0\x01\x07Element\x01\0\x01\x04slot\x01\x01\x05self_\x04slot\0\0\0\x19__widl_f_set_slot_Element\0\0\0\x01\x07Element\x01\0\x02\x04slot\x01\x02\x05self_\x04slot\x04slot\0\0\0 __widl_f_after_with_node_Element\x01\x01\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x05nodes\x05after\0\0\0\"__widl_f_after_with_node_0_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x05after\0\0\0\"__widl_f_after_with_node_1_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07nodes_1\x05after\0\0\0\"__widl_f_after_with_node_2_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x05after\0\0\0\"__widl_f_after_with_node_3_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x05after\0\0\0\"__widl_f_after_with_node_4_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x05after\0\0\0\"__widl_f_after_with_node_5_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x05after\0\0\0\"__widl_f_after_with_node_6_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x05after\0\0\0\"__widl_f_after_with_node_7_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x05after\0\0\0\x1F__widl_f_after_with_str_Element\x01\x01\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x05nodes\x05after\0\0\0!__widl_f_after_with_str_0_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x05after\0\0\0!__widl_f_after_with_str_1_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07nodes_1\x05after\0\0\0!__widl_f_after_with_str_2_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x05after\0\0\0!__widl_f_after_with_str_3_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x05after\0\0\0!__widl_f_after_with_str_4_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x05after\0\0\0!__widl_f_after_with_str_5_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x05after\0\0\0!__widl_f_after_with_str_6_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x05after\0\0\0!__widl_f_after_with_str_7_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x05after\0\0\0!__widl_f_before_with_node_Element\x01\x01\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x05nodes\x06before\0\0\0#__widl_f_before_with_node_0_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x06before\0\0\0#__widl_f_before_with_node_1_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07nodes_1\x06before\0\0\0#__widl_f_before_with_node_2_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06before\0\0\0#__widl_f_before_with_node_3_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06before\0\0\0#__widl_f_before_with_node_4_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06before\0\0\0#__widl_f_before_with_node_5_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06before\0\0\0#__widl_f_before_with_node_6_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06before\0\0\0#__widl_f_before_with_node_7_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06before\0\0\0 __widl_f_before_with_str_Element\x01\x01\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x05nodes\x06before\0\0\0\"__widl_f_before_with_str_0_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x06before\0\0\0\"__widl_f_before_with_str_1_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07nodes_1\x06before\0\0\0\"__widl_f_before_with_str_2_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06before\0\0\0\"__widl_f_before_with_str_3_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06before\0\0\0\"__widl_f_before_with_str_4_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06before\0\0\0\"__widl_f_before_with_str_5_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06before\0\0\0\"__widl_f_before_with_str_6_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06before\0\0\0\"__widl_f_before_with_str_7_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06before\0\0\0\x17__widl_f_remove_Element\0\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x06remove\0\0\0'__widl_f_replace_with_with_node_Element\x01\x01\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x05nodes\x0BreplaceWith\0\0\0)__widl_f_replace_with_with_node_0_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x0BreplaceWith\0\0\0)__widl_f_replace_with_with_node_1_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07nodes_1\x0BreplaceWith\0\0\0)__widl_f_replace_with_with_node_2_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x0BreplaceWith\0\0\0)__widl_f_replace_with_with_node_3_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x0BreplaceWith\0\0\0)__widl_f_replace_with_with_node_4_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x0BreplaceWith\0\0\0)__widl_f_replace_with_with_node_5_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x0BreplaceWith\0\0\0)__widl_f_replace_with_with_node_6_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x0BreplaceWith\0\0\0)__widl_f_replace_with_with_node_7_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x0BreplaceWith\0\0\0&__widl_f_replace_with_with_str_Element\x01\x01\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x05nodes\x0BreplaceWith\0\0\0(__widl_f_replace_with_with_str_0_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x0BreplaceWith\0\0\0(__widl_f_replace_with_with_str_1_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07nodes_1\x0BreplaceWith\0\0\0(__widl_f_replace_with_with_str_2_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x0BreplaceWith\0\0\0(__widl_f_replace_with_with_str_3_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x0BreplaceWith\0\0\0(__widl_f_replace_with_with_str_4_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x0BreplaceWith\0\0\0(__widl_f_replace_with_with_str_5_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x0BreplaceWith\0\0\0(__widl_f_replace_with_with_str_6_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x0BreplaceWith\0\0\0(__widl_f_replace_with_with_str_7_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x0BreplaceWith\0\0\02__widl_f_convert_point_from_node_with_text_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x05point\x04from\x14convertPointFromNode\0\0\05__widl_f_convert_point_from_node_with_element_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x05point\x04from\x14convertPointFromNode\0\0\06__widl_f_convert_point_from_node_with_document_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x05point\x04from\x14convertPointFromNode\0\0\0>__widl_f_convert_point_from_node_with_text_and_options_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x05point\x04from\x07options\x14convertPointFromNode\0\0\0A__widl_f_convert_point_from_node_with_element_and_options_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x05point\x04from\x07options\x14convertPointFromNode\0\0\0B__widl_f_convert_point_from_node_with_document_and_options_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x05point\x04from\x07options\x14convertPointFromNode\0\0\01__widl_f_convert_quad_from_node_with_text_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x04quad\x04from\x13convertQuadFromNode\0\0\04__widl_f_convert_quad_from_node_with_element_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x04quad\x04from\x13convertQuadFromNode\0\0\05__widl_f_convert_quad_from_node_with_document_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x04quad\x04from\x13convertQuadFromNode\0\0\0=__widl_f_convert_quad_from_node_with_text_and_options_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x04quad\x04from\x07options\x13convertQuadFromNode\0\0\0@__widl_f_convert_quad_from_node_with_element_and_options_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x04quad\x04from\x07options\x13convertQuadFromNode\0\0\0A__widl_f_convert_quad_from_node_with_document_and_options_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x04quad\x04from\x07options\x13convertQuadFromNode\0\0\01__widl_f_convert_rect_from_node_with_text_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x04rect\x04from\x13convertRectFromNode\0\0\04__widl_f_convert_rect_from_node_with_element_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x04rect\x04from\x13convertRectFromNode\0\0\05__widl_f_convert_rect_from_node_with_document_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x04rect\x04from\x13convertRectFromNode\0\0\0=__widl_f_convert_rect_from_node_with_text_and_options_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x04rect\x04from\x07options\x13convertRectFromNode\0\0\0@__widl_f_convert_rect_from_node_with_element_and_options_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x04rect\x04from\x07options\x13convertRectFromNode\0\0\0A__widl_f_convert_rect_from_node_with_document_and_options_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x04rect\x04from\x07options\x13convertRectFromNode\0\0\0\x1E__widl_f_get_box_quads_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x0BgetBoxQuads\0\0\0+__widl_f_get_box_quads_with_options_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07options\x0BgetBoxQuads\0\0\0)__widl_f_previous_element_sibling_Element\0\0\0\x01\x07Element\x01\0\x01\x16previousElementSibling\x01\x01\x05self_\x16previousElementSibling\0\0\0%__widl_f_next_element_sibling_Element\0\0\0\x01\x07Element\x01\0\x01\x12nextElementSibling\x01\x01\x05self_\x12nextElementSibling\0\0\0!__widl_f_append_with_node_Element\x01\x01\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x05nodes\x06append\0\0\0#__widl_f_append_with_node_0_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x06append\0\0\0#__widl_f_append_with_node_1_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07nodes_1\x06append\0\0\0#__widl_f_append_with_node_2_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06append\0\0\0#__widl_f_append_with_node_3_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06append\0\0\0#__widl_f_append_with_node_4_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06append\0\0\0#__widl_f_append_with_node_5_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06append\0\0\0#__widl_f_append_with_node_6_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06append\0\0\0#__widl_f_append_with_node_7_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06append\0\0\0 __widl_f_append_with_str_Element\x01\x01\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x05nodes\x06append\0\0\0\"__widl_f_append_with_str_0_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x06append\0\0\0\"__widl_f_append_with_str_1_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07nodes_1\x06append\0\0\0\"__widl_f_append_with_str_2_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x06append\0\0\0\"__widl_f_append_with_str_3_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x06append\0\0\0\"__widl_f_append_with_str_4_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x06append\0\0\0\"__widl_f_append_with_str_5_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x06append\0\0\0\"__widl_f_append_with_str_6_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x06append\0\0\0\"__widl_f_append_with_str_7_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x06append\0\0\0\"__widl_f_prepend_with_node_Element\x01\x01\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x05nodes\x07prepend\0\0\0$__widl_f_prepend_with_node_0_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x07prepend\0\0\0$__widl_f_prepend_with_node_1_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07nodes_1\x07prepend\0\0\0$__widl_f_prepend_with_node_2_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x07prepend\0\0\0$__widl_f_prepend_with_node_3_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07prepend\0\0\0$__widl_f_prepend_with_node_4_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07prepend\0\0\0$__widl_f_prepend_with_node_5_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07prepend\0\0\0$__widl_f_prepend_with_node_6_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07prepend\0\0\0$__widl_f_prepend_with_node_7_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x07prepend\0\0\0!__widl_f_prepend_with_str_Element\x01\x01\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x05nodes\x07prepend\0\0\0#__widl_f_prepend_with_str_0_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x01\x05self_\x07prepend\0\0\0#__widl_f_prepend_with_str_1_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x02\x05self_\x07nodes_1\x07prepend\0\0\0#__widl_f_prepend_with_str_2_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x03\x05self_\x07nodes_1\x07nodes_2\x07prepend\0\0\0#__widl_f_prepend_with_str_3_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x04\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07prepend\0\0\0#__widl_f_prepend_with_str_4_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x05\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07prepend\0\0\0#__widl_f_prepend_with_str_5_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x06\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07prepend\0\0\0#__widl_f_prepend_with_str_6_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x07\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07prepend\0\0\0#__widl_f_prepend_with_str_7_Element\x01\0\0\x01\x07Element\x01\0\0\x01\x08\x05self_\x07nodes_1\x07nodes_2\x07nodes_3\x07nodes_4\x07nodes_5\x07nodes_6\x07nodes_7\x07prepend\0\0\0\x19__widl_f_children_Element\0\0\0\x01\x07Element\x01\0\x01\x08children\x01\x01\x05self_\x08children\0\0\0$__widl_f_first_element_child_Element\0\0\0\x01\x07Element\x01\0\x01\x11firstElementChild\x01\x01\x05self_\x11firstElementChild\0\0\0#__widl_f_last_element_child_Element\0\0\0\x01\x07Element\x01\0\x01\x10lastElementChild\x01\x01\x05self_\x10lastElementChild\0\0\0$__widl_f_child_element_count_Element\0\0\0\x01\x07Element\x01\0\x01\x11childElementCount\x01\x01\x05self_\x11childElementCount\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
