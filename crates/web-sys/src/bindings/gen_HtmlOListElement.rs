use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLOListElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement)\n\n*This API requires the following crate features to be activated: `HtmlOListElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlOListElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlOListElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlOListElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(79u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlOListElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlOListElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlOListElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlOListElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlOListElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlOListElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlOListElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlOListElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlOListElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlOListElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlOListElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlOListElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlOListElement {
            HtmlOListElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlOListElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlOListElement> for HtmlOListElement {
        #[inline]
        fn as_ref(&self) -> &HtmlOListElement {
            self
        }
    }
    impl From<HtmlOListElement> for JsValue {
        #[inline]
        fn from(obj: HtmlOListElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlOListElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLOListElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLOListElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLOListElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlOListElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlOListElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlOListElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlOListElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlOListElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOListElement> for Element {
    #[inline]
    fn from(obj: HtmlOListElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlOListElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOListElement> for Node {
    #[inline]
    fn from(obj: HtmlOListElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlOListElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOListElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlOListElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlOListElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOListElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlOListElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlOListElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlOListElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reversed_HTMLOListElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOListElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlOListElement {
    #[cfg(all(feature = "HtmlOListElement",))]
    #[allow(bad_style)]
    #[doc = "The `reversed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/reversed)\n\n*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    #[allow(clippy::all)]
    pub fn reversed(&self) -> bool {
        #[cfg(all(feature = "HtmlOListElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reversed_HTMLOListElement(
                self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reversed_HTMLOListElement(
            self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_reversed_HTMLOListElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOListElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_reversed_HTMLOListElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOListElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOListElement {
    #[cfg(all(feature = "HtmlOListElement",))]
    #[allow(bad_style)]
    #[doc = "The `reversed` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/reversed)\n\n*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    #[allow(clippy::all)]
    pub fn set_reversed(&self, reversed: bool) {
        #[cfg(all(feature = "HtmlOListElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_reversed_HTMLOListElement(
                self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                reversed: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_reversed_HTMLOListElement(
            self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            reversed: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(reversed);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let reversed = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(reversed);
                __widl_f_set_reversed_HTMLOListElement(self_, reversed)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOListElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_HTMLOListElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOListElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlOListElement {
    #[cfg(all(feature = "HtmlOListElement",))]
    #[allow(bad_style)]
    #[doc = "The `start` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/start)\n\n*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    #[allow(clippy::all)]
    pub fn start(&self) -> i32 {
        #[cfg(all(feature = "HtmlOListElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_HTMLOListElement(
                self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_HTMLOListElement(
            self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_HTMLOListElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOListElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_start_HTMLOListElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOListElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOListElement {
    #[cfg(all(feature = "HtmlOListElement",))]
    #[allow(bad_style)]
    #[doc = "The `start` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/start)\n\n*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    #[allow(clippy::all)]
    pub fn set_start(&self, start: i32) {
        #[cfg(all(feature = "HtmlOListElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_start_HTMLOListElement(
                self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_start_HTMLOListElement(
            self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(start);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                __widl_f_set_start_HTMLOListElement(self_, start)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOListElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLOListElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOListElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlOListElement {
    #[cfg(all(feature = "HtmlOListElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/type)\n\n*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlOListElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLOListElement(
                self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLOListElement(
            self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLOListElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOListElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_HTMLOListElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOListElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOListElement {
    #[cfg(all(feature = "HtmlOListElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/type)\n\n*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "HtmlOListElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_HTMLOListElement(
                self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_HTMLOListElement(
            self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_HTMLOListElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOListElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_compact_HTMLOListElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOListElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlOListElement {
    #[cfg(all(feature = "HtmlOListElement",))]
    #[allow(bad_style)]
    #[doc = "The `compact` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/compact)\n\n*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    #[allow(clippy::all)]
    pub fn compact(&self) -> bool {
        #[cfg(all(feature = "HtmlOListElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_compact_HTMLOListElement(
                self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_compact_HTMLOListElement(
            self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_compact_HTMLOListElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOListElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_compact_HTMLOListElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOListElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOListElement {
    #[cfg(all(feature = "HtmlOListElement",))]
    #[allow(bad_style)]
    #[doc = "The `compact` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/compact)\n\n*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    #[allow(clippy::all)]
    pub fn set_compact(&self, compact: bool) {
        #[cfg(all(feature = "HtmlOListElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_compact_HTMLOListElement(
                self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                compact: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_compact_HTMLOListElement(
            self_: <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            compact: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(compact);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOListElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let compact = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(compact);
                __widl_f_set_compact_HTMLOListElement(self_, compact)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_29d9aa6c0db8c547: [u8; 862usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1C\x03\0\0\0\0\t\0\0\x02\x10HTMLOListElement\"__widl_instanceof_HTMLOListElement\0\0\0\0\"__widl_f_reversed_HTMLOListElement\0\0\0\x01\x10HTMLOListElement\x01\0\x01\x08reversed\x01\x01\x05self_\x08reversed\0\0\0&__widl_f_set_reversed_HTMLOListElement\0\0\0\x01\x10HTMLOListElement\x01\0\x02\x08reversed\x01\x02\x05self_\x08reversed\x08reversed\0\0\0\x1F__widl_f_start_HTMLOListElement\0\0\0\x01\x10HTMLOListElement\x01\0\x01\x05start\x01\x01\x05self_\x05start\0\0\0#__widl_f_set_start_HTMLOListElement\0\0\0\x01\x10HTMLOListElement\x01\0\x02\x05start\x01\x02\x05self_\x05start\x05start\0\0\0\x1E__widl_f_type_HTMLOListElement\0\0\0\x01\x10HTMLOListElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\"__widl_f_set_type_HTMLOListElement\0\0\0\x01\x10HTMLOListElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0!__widl_f_compact_HTMLOListElement\0\0\0\x01\x10HTMLOListElement\x01\0\x01\x07compact\x01\x01\x05self_\x07compact\0\0\0%__widl_f_set_compact_HTMLOListElement\0\0\0\x01\x10HTMLOListElement\x01\0\x02\x07compact\x01\x02\x05self_\x07compact\x07compact\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
