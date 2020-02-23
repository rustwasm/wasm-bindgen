use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ShadowRoot` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot)\n\n*This API requires the following crate features to be activated: `ShadowRoot`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ShadowRoot {
    obj: DocumentFragment,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ShadowRoot: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ShadowRoot {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(83u32);
            inform(104u32);
            inform(97u32);
            inform(100u32);
            inform(111u32);
            inform(119u32);
            inform(82u32);
            inform(111u32);
            inform(111u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for ShadowRoot {
        type Target = DocumentFragment;
        #[inline]
        fn deref(&self) -> &DocumentFragment {
            &self.obj
        }
    }
    impl IntoWasmAbi for ShadowRoot {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ShadowRoot {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ShadowRoot {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ShadowRoot {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ShadowRoot {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ShadowRoot {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ShadowRoot {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ShadowRoot {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ShadowRoot>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ShadowRoot {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ShadowRoot {
        #[inline]
        fn from(obj: JsValue) -> ShadowRoot {
            ShadowRoot { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ShadowRoot {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ShadowRoot> for ShadowRoot {
        #[inline]
        fn as_ref(&self) -> &ShadowRoot {
            self
        }
    }
    impl From<ShadowRoot> for JsValue {
        #[inline]
        fn from(obj: ShadowRoot) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ShadowRoot {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ShadowRoot(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ShadowRoot(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ShadowRoot(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ShadowRoot { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ShadowRoot) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ShadowRoot> for DocumentFragment {
    #[inline]
    fn from(obj: ShadowRoot) -> DocumentFragment {
        use wasm_bindgen::JsCast;
        DocumentFragment::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<DocumentFragment> for ShadowRoot {
    #[inline]
    fn as_ref(&self) -> &DocumentFragment {
        use wasm_bindgen::JsCast;
        DocumentFragment::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ShadowRoot> for Node {
    #[inline]
    fn from(obj: ShadowRoot) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for ShadowRoot {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ShadowRoot> for EventTarget {
    #[inline]
    fn from(obj: ShadowRoot) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ShadowRoot {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ShadowRoot> for ::js_sys::Object {
    #[inline]
    fn from(obj: ShadowRoot) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ShadowRoot {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Element", feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_element_by_id_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `getElementById()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementById)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn get_element_by_id(&self, element_id: &str) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_element_by_id_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_element_by_id_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element_id);
                __widl_f_get_element_by_id_ShadowRoot(self_, element_id)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlCollection", feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_elements_by_class_name_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "HtmlCollection", feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `getElementsByClassName()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByClassName)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn get_elements_by_class_name(&self, class_names: &str) -> HtmlCollection {
        #[cfg(all(feature = "HtmlCollection", feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_elements_by_class_name_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                class_names: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_elements_by_class_name_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let class_names =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(class_names);
                __widl_f_get_elements_by_class_name_ShadowRoot(self_, class_names)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlCollection", feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_elements_by_tag_name_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "HtmlCollection", feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `getElementsByTagName()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByTagName)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn get_elements_by_tag_name(&self, local_name: &str) -> HtmlCollection {
        #[cfg(all(feature = "HtmlCollection", feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_elements_by_tag_name_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_elements_by_tag_name_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_get_elements_by_tag_name_ShadowRoot(self_, local_name)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlCollection", feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_elements_by_tag_name_ns_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "HtmlCollection", feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `getElementsByTagNameNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByTagNameNS)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn get_elements_by_tag_name_ns(
        &self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> HtmlCollection {
        #[cfg(all(feature = "HtmlCollection", feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_elements_by_tag_name_ns_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_elements_by_tag_name_ns_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_get_elements_by_tag_name_ns_ShadowRoot(self_, namespace, local_name)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ShadowRoot", feature = "ShadowRootMode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_mode_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <ShadowRootMode as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "ShadowRoot", feature = "ShadowRootMode",))]
    #[allow(bad_style)]
    #[doc = "The `mode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/mode)\n\n*This API requires the following crate features to be activated: `ShadowRoot`, `ShadowRootMode`*"]
    #[allow(clippy::all)]
    pub fn mode(&self) -> ShadowRootMode {
        #[cfg(all(feature = "ShadowRoot", feature = "ShadowRootMode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_mode_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ShadowRootMode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_mode_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ShadowRootMode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_mode_ShadowRoot(self_)
            };
            <ShadowRootMode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_host_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <Element as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `host` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/host)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn host(&self) -> Element {
        #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_host_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_host_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Element as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_host_ShadowRoot(self_)
            };
            <Element as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_inner_html_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `innerHTML` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/innerHTML)\n\n*This API requires the following crate features to be activated: `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn inner_html(&self) -> String {
        #[cfg(all(feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_inner_html_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_inner_html_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_inner_html_ShadowRoot(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_inner_html_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `innerHTML` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/innerHTML)\n\n*This API requires the following crate features to be activated: `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn set_inner_html(&self, inner_html: &str) {
        #[cfg(all(feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_inner_html_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                inner_html: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_inner_html_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let inner_html = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(inner_html);
                __widl_f_set_inner_html_ShadowRoot(self_, inner_html)
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
pub extern "C" fn __wbindgen_describe___widl_f_element_from_point_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `elementFromPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/elementFromPoint)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn element_from_point(&self, x: f32, y: f32) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_element_from_point_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_element_from_point_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_element_from_point_ShadowRoot(self_, x, y)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_elements_from_point_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `elementsFromPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/elementsFromPoint)\n\n*This API requires the following crate features to be activated: `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn elements_from_point(&self, x: f32, y: f32) -> ::js_sys::Array {
        #[cfg(all(feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_elements_from_point_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_elements_from_point_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_elements_from_point_ShadowRoot(self_, x, y)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_active_element_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `activeElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/activeElement)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn active_element(&self) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_active_element_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_active_element_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_active_element_ShadowRoot(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ShadowRoot", feature = "StyleSheetList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_style_sheets_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <StyleSheetList as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "ShadowRoot", feature = "StyleSheetList",))]
    #[allow(bad_style)]
    #[doc = "The `styleSheets` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/styleSheets)\n\n*This API requires the following crate features to be activated: `ShadowRoot`, `StyleSheetList`*"]
    #[allow(clippy::all)]
    pub fn style_sheets(&self) -> StyleSheetList {
        #[cfg(all(feature = "ShadowRoot", feature = "StyleSheetList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_style_sheets_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <StyleSheetList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_style_sheets_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <StyleSheetList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_style_sheets_ShadowRoot(self_)
            };
            <StyleSheetList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pointer_lock_element_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `pointerLockElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/pointerLockElement)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn pointer_lock_element(&self) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pointer_lock_element_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pointer_lock_element_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pointer_lock_element_ShadowRoot(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "ShadowRoot",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fullscreen_element_ShadowRoot() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ShadowRoot as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl ShadowRoot {
    #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
    #[allow(bad_style)]
    #[doc = "The `fullscreenElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/fullscreenElement)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    #[allow(clippy::all)]
    pub fn fullscreen_element(&self) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "ShadowRoot",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fullscreen_element_ShadowRoot(
                self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fullscreen_element_ShadowRoot(
            self_: <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ShadowRoot as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_fullscreen_element_ShadowRoot(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5a53c17e52c3ae42: [u8; 1453usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}k\x05\0\0\0\0\x0F\0\0\x02\nShadowRoot\x1C__widl_instanceof_ShadowRoot\0\0\0\0%__widl_f_get_element_by_id_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\0\x01\x02\x05self_\nelement_id\x0EgetElementById\0\0\0.__widl_f_get_elements_by_class_name_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\0\x01\x02\x05self_\x0Bclass_names\x16getElementsByClassName\0\0\0,__widl_f_get_elements_by_tag_name_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\0\x01\x02\x05self_\nlocal_name\x14getElementsByTagName\0\0\0/__widl_f_get_elements_by_tag_name_ns_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\0\x01\x03\x05self_\tnamespace\nlocal_name\x16getElementsByTagNameNS\0\0\0\x18__widl_f_mode_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\x01\x04mode\x01\x01\x05self_\x04mode\0\0\0\x18__widl_f_host_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\x01\x04host\x01\x01\x05self_\x04host\0\0\0\x1E__widl_f_inner_html_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\x01\tinnerHTML\x01\x01\x05self_\tinnerHTML\0\0\0\"__widl_f_set_inner_html_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\x02\tinnerHTML\x01\x02\x05self_\ninner_html\tinnerHTML\0\0\0&__widl_f_element_from_point_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\0\x01\x03\x05self_\x01x\x01y\x10elementFromPoint\0\0\0'__widl_f_elements_from_point_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\0\x01\x03\x05self_\x01x\x01y\x11elementsFromPoint\0\0\0\"__widl_f_active_element_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\x01\ractiveElement\x01\x01\x05self_\ractiveElement\0\0\0 __widl_f_style_sheets_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\x01\x0BstyleSheets\x01\x01\x05self_\x0BstyleSheets\0\0\0(__widl_f_pointer_lock_element_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\x01\x12pointerLockElement\x01\x01\x05self_\x12pointerLockElement\0\0\0&__widl_f_fullscreen_element_ShadowRoot\0\0\0\x01\nShadowRoot\x01\0\x01\x11fullscreenElement\x01\x01\x05self_\x11fullscreenElement\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
