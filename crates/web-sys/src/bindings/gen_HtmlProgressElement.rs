use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLProgressElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlProgressElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlProgressElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlProgressElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(80u32);
            inform(114u32);
            inform(111u32);
            inform(103u32);
            inform(114u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlProgressElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlProgressElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlProgressElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlProgressElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlProgressElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlProgressElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlProgressElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlProgressElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlProgressElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlProgressElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlProgressElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlProgressElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlProgressElement {
            HtmlProgressElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlProgressElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlProgressElement> for HtmlProgressElement {
        #[inline]
        fn as_ref(&self) -> &HtmlProgressElement {
            self
        }
    }
    impl From<HtmlProgressElement> for JsValue {
        #[inline]
        fn from(obj: HtmlProgressElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlProgressElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLProgressElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLProgressElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLProgressElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlProgressElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlProgressElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlProgressElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlProgressElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlProgressElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlProgressElement> for Element {
    #[inline]
    fn from(obj: HtmlProgressElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlProgressElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlProgressElement> for Node {
    #[inline]
    fn from(obj: HtmlProgressElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlProgressElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlProgressElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlProgressElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlProgressElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlProgressElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlProgressElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlProgressElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlProgressElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_HTMLProgressElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlProgressElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlProgressElement {
    #[cfg(all(feature = "HtmlProgressElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> f64 {
        #[cfg(all(feature = "HtmlProgressElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_HTMLProgressElement(
                self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_HTMLProgressElement(
            self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_HTMLProgressElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlProgressElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_HTMLProgressElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlProgressElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlProgressElement {
    #[cfg(all(feature = "HtmlProgressElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: f64) {
        #[cfg(all(feature = "HtmlProgressElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_HTMLProgressElement(
                self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_HTMLProgressElement(
            self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_HTMLProgressElement(self_, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlProgressElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_HTMLProgressElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlProgressElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlProgressElement {
    #[cfg(all(feature = "HtmlProgressElement",))]
    #[allow(bad_style)]
    #[doc = "The `max` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    #[allow(clippy::all)]
    pub fn max(&self) -> f64 {
        #[cfg(all(feature = "HtmlProgressElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_HTMLProgressElement(
                self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_HTMLProgressElement(
            self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_HTMLProgressElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlProgressElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_max_HTMLProgressElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlProgressElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlProgressElement {
    #[cfg(all(feature = "HtmlProgressElement",))]
    #[allow(bad_style)]
    #[doc = "The `max` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    #[allow(clippy::all)]
    pub fn set_max(&self, max: f64) {
        #[cfg(all(feature = "HtmlProgressElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_max_HTMLProgressElement(
                self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_max_HTMLProgressElement(
            self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(max);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max);
                __widl_f_set_max_HTMLProgressElement(self_, max)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlProgressElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_position_HTMLProgressElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlProgressElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlProgressElement {
    #[cfg(all(feature = "HtmlProgressElement",))]
    #[allow(bad_style)]
    #[doc = "The `position` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/position)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    #[allow(clippy::all)]
    pub fn position(&self) -> f64 {
        #[cfg(all(feature = "HtmlProgressElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_position_HTMLProgressElement(
                self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_position_HTMLProgressElement(
            self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_position_HTMLProgressElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlProgressElement", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_labels_HTMLProgressElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlProgressElement as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl HtmlProgressElement {
    #[cfg(all(feature = "HtmlProgressElement", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `labels` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn labels(&self) -> NodeList {
        #[cfg(all(feature = "HtmlProgressElement", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_labels_HTMLProgressElement(
                self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_labels_HTMLProgressElement(
            self_: <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlProgressElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_labels_HTMLProgressElement(self_)
            };
            <NodeList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d507b7724242f46d: [u8; 695usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}u\x02\0\0\0\0\x07\0\0\x02\x13HTMLProgressElement%__widl_instanceof_HTMLProgressElement\0\0\0\0\"__widl_f_value_HTMLProgressElement\0\0\0\x01\x13HTMLProgressElement\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0&__widl_f_set_value_HTMLProgressElement\0\0\0\x01\x13HTMLProgressElement\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0 __widl_f_max_HTMLProgressElement\0\0\0\x01\x13HTMLProgressElement\x01\0\x01\x03max\x01\x01\x05self_\x03max\0\0\0$__widl_f_set_max_HTMLProgressElement\0\0\0\x01\x13HTMLProgressElement\x01\0\x02\x03max\x01\x02\x05self_\x03max\x03max\0\0\0%__widl_f_position_HTMLProgressElement\0\0\0\x01\x13HTMLProgressElement\x01\0\x01\x08position\x01\x01\x05self_\x08position\0\0\0#__widl_f_labels_HTMLProgressElement\0\0\0\x01\x13HTMLProgressElement\x01\0\x01\x06labels\x01\x01\x05self_\x06labels\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
