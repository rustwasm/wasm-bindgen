use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLAreaElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlAreaElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlAreaElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlAreaElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(65u32);
            inform(114u32);
            inform(101u32);
            inform(97u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlAreaElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlAreaElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlAreaElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlAreaElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlAreaElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlAreaElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlAreaElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlAreaElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlAreaElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlAreaElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlAreaElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlAreaElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlAreaElement {
            HtmlAreaElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlAreaElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlAreaElement> for HtmlAreaElement {
        #[inline]
        fn as_ref(&self) -> &HtmlAreaElement {
            self
        }
    }
    impl From<HtmlAreaElement> for JsValue {
        #[inline]
        fn from(obj: HtmlAreaElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlAreaElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLAreaElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLAreaElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLAreaElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlAreaElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlAreaElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlAreaElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlAreaElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlAreaElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAreaElement> for Element {
    #[inline]
    fn from(obj: HtmlAreaElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlAreaElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAreaElement> for Node {
    #[inline]
    fn from(obj: HtmlAreaElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlAreaElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAreaElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlAreaElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlAreaElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAreaElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlAreaElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlAreaElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_alt_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `alt` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn alt(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_alt_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_alt_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_alt_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_alt_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `alt` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_alt(&self, alt: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_alt_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_alt_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(alt);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let alt = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt);
                __widl_f_set_alt_HTMLAreaElement(self_, alt)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_coords_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `coords` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/coords)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn coords(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_coords_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_coords_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_coords_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_coords_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `coords` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/coords)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_coords(&self, coords: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_coords_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                coords: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_coords_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            coords: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(coords);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let coords = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(coords);
                __widl_f_set_coords_HTMLAreaElement(self_, coords)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shape_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `shape` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/shape)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn shape(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shape_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shape_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_shape_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_shape_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `shape` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/shape)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_shape(&self, shape: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_shape_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shape: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_shape_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shape: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(shape);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shape = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shape);
                __widl_f_set_shape_HTMLAreaElement(self_, shape)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/target)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_target_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `target` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/target)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_target(&self, target: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_target_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_target_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_set_target_HTMLAreaElement(self_, target)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_download_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `download` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/download)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn download(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_download_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_download_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_download_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_download_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `download` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/download)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_download(&self, download: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_download_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                download: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_download_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            download: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(download);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let download = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(download);
                __widl_f_set_download_HTMLAreaElement(self_, download)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ping_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `ping` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/ping)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn ping(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ping_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ping_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ping_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ping_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `ping` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/ping)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_ping(&self, ping: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ping_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ping: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ping_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ping: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ping);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ping = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ping);
                __widl_f_set_ping_HTMLAreaElement(self_, ping)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rel_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `rel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn rel(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rel_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rel_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rel_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rel_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `rel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_rel(&self, rel: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rel_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rel: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rel_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rel: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(rel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rel = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rel);
                __widl_f_set_rel_HTMLAreaElement(self_, rel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_referrer_policy_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn referrer_policy(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_referrer_policy_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_referrer_policy_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_referrer_policy_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_referrer_policy_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_referrer_policy(&self, referrer_policy: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_referrer_policy_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                referrer_policy: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_referrer_policy_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            referrer_policy: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(referrer_policy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let referrer_policy =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(referrer_policy);
                __widl_f_set_referrer_policy_HTMLAreaElement(self_, referrer_policy)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomTokenList", feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rel_list_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <DomTokenList as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "DomTokenList", feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `relList` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/relList)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn rel_list(&self) -> DomTokenList {
        #[cfg(all(feature = "DomTokenList", feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rel_list_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rel_list_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rel_list_HTMLAreaElement(self_)
            };
            <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_no_href_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `noHref` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/noHref)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn no_href(&self) -> bool {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_no_href_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_no_href_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_no_href_HTMLAreaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_no_href_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `noHref` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/noHref)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_no_href(&self, no_href: bool) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_no_href_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                no_href: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_no_href_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            no_href: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(no_href);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let no_href = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(no_href);
                __widl_f_set_no_href_HTMLAreaElement(self_, no_href)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_href_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `href` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/href)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn href(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_href_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_href_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_href_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_href_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `href` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/href)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_href(&self, href: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_href_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                href: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_href_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            href: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(href);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let href = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(href);
                __widl_f_set_href_HTMLAreaElement(self_, href)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_origin_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `origin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/origin)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn origin(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_origin_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_origin_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_origin_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_protocol_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `protocol` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/protocol)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn protocol(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_protocol_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_protocol_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_protocol_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_protocol_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `protocol` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/protocol)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_protocol(&self, protocol: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_protocol_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                protocol: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_protocol_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            protocol: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(protocol);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let protocol = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(protocol);
                __widl_f_set_protocol_HTMLAreaElement(self_, protocol)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_username_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `username` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/username)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn username(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_username_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_username_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_username_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_username_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `username` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/username)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_username(&self, username: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_username_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                username: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_username_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            username: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(username);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let username = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(username);
                __widl_f_set_username_HTMLAreaElement(self_, username)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_password_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `password` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/password)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn password(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_password_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_password_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_password_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_password_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `password` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/password)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_password(&self, password: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_password_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                password: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_password_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            password: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(password);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let password = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(password);
                __widl_f_set_password_HTMLAreaElement(self_, password)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_host_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `host` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/host)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn host(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_host_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_host_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_host_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_host_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `host` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/host)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_host(&self, host: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_host_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_host_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(host);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let host = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(host);
                __widl_f_set_host_HTMLAreaElement(self_, host)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hostname_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `hostname` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hostname)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn hostname(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hostname_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hostname_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hostname_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hostname_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `hostname` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hostname)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_hostname(&self, hostname: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hostname_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hostname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hostname_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hostname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hostname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hostname = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hostname);
                __widl_f_set_hostname_HTMLAreaElement(self_, hostname)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_port_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `port` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/port)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn port(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_port_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_port_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_port_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_port_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `port` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/port)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_port(&self, port: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_port_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                port: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_port_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            port: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(port);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let port = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(port);
                __widl_f_set_port_HTMLAreaElement(self_, port)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pathname_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `pathname` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/pathname)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn pathname(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pathname_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pathname_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pathname_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_pathname_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `pathname` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/pathname)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_pathname(&self, pathname: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_pathname_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pathname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_pathname_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pathname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(pathname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pathname = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pathname);
                __widl_f_set_pathname_HTMLAreaElement(self_, pathname)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_search_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `search` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/search)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn search(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_search_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_search_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_search_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_search_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `search` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/search)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_search(&self, search: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_search_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                search: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_search_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            search: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(search);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let search = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(search);
                __widl_f_set_search_HTMLAreaElement(self_, search)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hash_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `hash` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hash)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn hash(&self) -> String {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hash_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hash_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hash_HTMLAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hash_HTMLAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAreaElement {
    #[cfg(all(feature = "HtmlAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `hash` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hash)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_hash(&self, hash: &str) {
        #[cfg(all(feature = "HtmlAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hash_HTMLAreaElement(
                self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hash: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hash_HTMLAreaElement(
            self_: <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hash: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hash);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hash = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hash);
                __widl_f_set_hash_HTMLAreaElement(self_, hash)
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
pub static __WASM_BINDGEN_GENERATED_9a06f12cb16e1cc1: [u8; 3599usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCD\r\0\0\0\0)\0\0\x02\x0FHTMLAreaElement!__widl_instanceof_HTMLAreaElement\0\0\0\0\x1C__widl_f_alt_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x03alt\x01\x01\x05self_\x03alt\0\0\0 __widl_f_set_alt_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x03alt\x01\x02\x05self_\x03alt\x03alt\0\0\0\x1F__widl_f_coords_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x06coords\x01\x01\x05self_\x06coords\0\0\0#__widl_f_set_coords_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x06coords\x01\x02\x05self_\x06coords\x06coords\0\0\0\x1E__widl_f_shape_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x05shape\x01\x01\x05self_\x05shape\0\0\0\"__widl_f_set_shape_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x05shape\x01\x02\x05self_\x05shape\x05shape\0\0\0\x1F__widl_f_target_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0#__widl_f_set_target_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x06target\x01\x02\x05self_\x06target\x06target\0\0\0!__widl_f_download_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x08download\x01\x01\x05self_\x08download\0\0\0%__widl_f_set_download_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x08download\x01\x02\x05self_\x08download\x08download\0\0\0\x1D__widl_f_ping_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x04ping\x01\x01\x05self_\x04ping\0\0\0!__widl_f_set_ping_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x04ping\x01\x02\x05self_\x04ping\x04ping\0\0\0\x1C__widl_f_rel_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x03rel\x01\x01\x05self_\x03rel\0\0\0 __widl_f_set_rel_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x03rel\x01\x02\x05self_\x03rel\x03rel\0\0\0(__widl_f_referrer_policy_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x0EreferrerPolicy\x01\x01\x05self_\x0EreferrerPolicy\0\0\0,__widl_f_set_referrer_policy_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x0EreferrerPolicy\x01\x02\x05self_\x0Freferrer_policy\x0EreferrerPolicy\0\0\0!__widl_f_rel_list_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x07relList\x01\x01\x05self_\x07relList\0\0\0 __widl_f_no_href_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x06noHref\x01\x01\x05self_\x06noHref\0\0\0$__widl_f_set_no_href_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x06noHref\x01\x02\x05self_\x07no_href\x06noHref\0\0\0\x1D__widl_f_href_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x04href\x01\x01\x05self_\x04href\0\0\0!__widl_f_set_href_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x04href\x01\x02\x05self_\x04href\x04href\0\0\0\x1F__widl_f_origin_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x06origin\x01\x01\x05self_\x06origin\0\0\0!__widl_f_protocol_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x08protocol\x01\x01\x05self_\x08protocol\0\0\0%__widl_f_set_protocol_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x08protocol\x01\x02\x05self_\x08protocol\x08protocol\0\0\0!__widl_f_username_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x08username\x01\x01\x05self_\x08username\0\0\0%__widl_f_set_username_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x08username\x01\x02\x05self_\x08username\x08username\0\0\0!__widl_f_password_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x08password\x01\x01\x05self_\x08password\0\0\0%__widl_f_set_password_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x08password\x01\x02\x05self_\x08password\x08password\0\0\0\x1D__widl_f_host_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x04host\x01\x01\x05self_\x04host\0\0\0!__widl_f_set_host_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x04host\x01\x02\x05self_\x04host\x04host\0\0\0!__widl_f_hostname_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x08hostname\x01\x01\x05self_\x08hostname\0\0\0%__widl_f_set_hostname_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x08hostname\x01\x02\x05self_\x08hostname\x08hostname\0\0\0\x1D__widl_f_port_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x04port\x01\x01\x05self_\x04port\0\0\0!__widl_f_set_port_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x04port\x01\x02\x05self_\x04port\x04port\0\0\0!__widl_f_pathname_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x08pathname\x01\x01\x05self_\x08pathname\0\0\0%__widl_f_set_pathname_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x08pathname\x01\x02\x05self_\x08pathname\x08pathname\0\0\0\x1F__widl_f_search_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x06search\x01\x01\x05self_\x06search\0\0\0#__widl_f_set_search_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x06search\x01\x02\x05self_\x06search\x06search\0\0\0\x1D__widl_f_hash_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x01\x04hash\x01\x01\x05self_\x04hash\0\0\0!__widl_f_set_hash_HTMLAreaElement\0\0\0\x01\x0FHTMLAreaElement\x01\0\x02\x04hash\x01\x02\x05self_\x04hash\x04hash\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
