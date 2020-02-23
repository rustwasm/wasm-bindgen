use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLImageElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlImageElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlImageElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlImageElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(73u32);
            inform(109u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlImageElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlImageElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlImageElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlImageElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlImageElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlImageElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlImageElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlImageElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlImageElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlImageElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlImageElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlImageElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlImageElement {
            HtmlImageElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlImageElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlImageElement> for HtmlImageElement {
        #[inline]
        fn as_ref(&self) -> &HtmlImageElement {
            self
        }
    }
    impl From<HtmlImageElement> for JsValue {
        #[inline]
        fn from(obj: HtmlImageElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlImageElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLImageElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLImageElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLImageElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlImageElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlImageElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlImageElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlImageElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlImageElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlImageElement> for Element {
    #[inline]
    fn from(obj: HtmlImageElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlImageElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlImageElement> for Node {
    #[inline]
    fn from(obj: HtmlImageElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlImageElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlImageElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlImageElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlImageElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlImageElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlImageElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlImageElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Image() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <HtmlImageElement as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `new HTMLImageElement(..)` constructor, creating a new instance of `HTMLImageElement`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/HTMLImageElement)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<HtmlImageElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Image() -> <HtmlImageElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Image(
        ) -> <HtmlImageElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_Image() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlImageElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_width_Image() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <u32 as WasmDescribe>::describe();
    <HtmlImageElement as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `new HTMLImageElement(..)` constructor, creating a new instance of `HTMLImageElement`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/HTMLImageElement)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn new_with_width(width: u32) -> Result<HtmlImageElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_width_Image(
                width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlImageElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_width_Image(
            width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlImageElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let width = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_new_with_width_Image(width)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlImageElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_width_and_height_Image() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <HtmlImageElement as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `new HTMLImageElement(..)` constructor, creating a new instance of `HTMLImageElement`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/HTMLImageElement)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn new_with_width_and_height(
        width: u32,
        height: u32,
    ) -> Result<HtmlImageElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_width_and_height_Image(
                width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlImageElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_width_and_height_Image(
            width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlImageElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(width);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let width = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_new_with_width_and_height_Image(width, height)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlImageElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `decode()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decode)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn decode(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_decode_HTMLImageElement(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_alt_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `alt` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn alt(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_alt_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_alt_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_alt_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_alt_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `alt` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_alt(&self, alt: &str) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_alt_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_alt_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let alt = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt);
                __widl_f_set_alt_HTMLImageElement(self_, alt)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_src_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/src)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn src(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_src_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_src_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_src_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_src_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/src)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_src(&self, src: &str) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_src_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_src_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(src);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_set_src_HTMLImageElement(self_, src)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_srcset_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `srcset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn srcset(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_srcset_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_srcset_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_srcset_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_srcset_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `srcset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_srcset(&self, srcset: &str) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_srcset_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                srcset: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_srcset_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            srcset: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(srcset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let srcset = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(srcset);
                __widl_f_set_srcset_HTMLImageElement(self_, srcset)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cross_origin_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `crossOrigin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn cross_origin(&self) -> Option<String> {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cross_origin_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cross_origin_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cross_origin_HTMLImageElement(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cross_origin_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `crossOrigin` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_cross_origin(&self, cross_origin: Option<&str>) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cross_origin_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cross_origin: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cross_origin_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cross_origin: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cross_origin);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cross_origin =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cross_origin);
                __widl_f_set_cross_origin_HTMLImageElement(self_, cross_origin)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_use_map_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `useMap` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn use_map(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_use_map_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_use_map_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_use_map_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_use_map_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `useMap` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_use_map(&self, use_map: &str) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_use_map_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                use_map: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_use_map_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            use_map: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(use_map);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let use_map = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(use_map);
                __widl_f_set_use_map_HTMLImageElement(self_, use_map)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_referrer_policy_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn referrer_policy(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_referrer_policy_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_referrer_policy_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_referrer_policy_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_referrer_policy_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_referrer_policy(&self, referrer_policy: &str) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_referrer_policy_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                referrer_policy: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_referrer_policy_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let referrer_policy =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(referrer_policy);
                __widl_f_set_referrer_policy_HTMLImageElement(self_, referrer_policy)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_map_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `isMap` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/isMap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn is_map(&self) -> bool {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_map_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_map_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_map_HTMLImageElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_is_map_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `isMap` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/isMap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_is_map(&self, is_map: bool) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_is_map_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                is_map: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_is_map_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            is_map: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(is_map);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let is_map = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(is_map);
                __widl_f_set_is_map_HTMLImageElement(self_, is_map)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/width)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> u32 {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_HTMLImageElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/width)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: u32) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_HTMLImageElement(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/height)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> u32 {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_HTMLImageElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_height_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/height)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_height(&self, height: u32) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_height_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_height_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let height = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_set_height_HTMLImageElement(self_, height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decoding_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `decoding` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decoding)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn decoding(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decoding_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decoding_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_decoding_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_decoding_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `decoding` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decoding)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_decoding(&self, decoding: &str) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_decoding_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                decoding: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_decoding_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            decoding: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(decoding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let decoding = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(decoding);
                __widl_f_set_decoding_HTMLImageElement(self_, decoding)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_natural_width_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `naturalWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalWidth)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn natural_width(&self) -> u32 {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_natural_width_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_natural_width_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_natural_width_HTMLImageElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_natural_height_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `naturalHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalHeight)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn natural_height(&self) -> u32 {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_natural_height_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_natural_height_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_natural_height_HTMLImageElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_complete_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `complete` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/complete)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn complete(&self) -> bool {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_complete_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_complete_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_complete_HTMLImageElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/name)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/name)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLImageElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/align)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/align)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(align);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLImageElement(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hspace_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `hspace` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/hspace)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn hspace(&self) -> u32 {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hspace_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hspace_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hspace_HTMLImageElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hspace_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `hspace` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/hspace)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_hspace(&self, hspace: u32) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hspace_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hspace: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hspace_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hspace: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hspace);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hspace = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hspace);
                __widl_f_set_hspace_HTMLImageElement(self_, hspace)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vspace_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `vspace` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/vspace)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn vspace(&self) -> u32 {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vspace_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vspace_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_vspace_HTMLImageElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_vspace_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `vspace` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/vspace)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_vspace(&self, vspace: u32) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_vspace_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                vspace: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_vspace_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            vspace: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(vspace);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let vspace = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(vspace);
                __widl_f_set_vspace_HTMLImageElement(self_, vspace)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_long_desc_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `longDesc` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn long_desc(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_long_desc_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_long_desc_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_long_desc_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_long_desc_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `longDesc` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_long_desc(&self, long_desc: &str) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_long_desc_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                long_desc: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_long_desc_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            long_desc: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(long_desc);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let long_desc = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(long_desc);
                __widl_f_set_long_desc_HTMLImageElement(self_, long_desc)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_border_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `border` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/border)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn border(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_border_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_border_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_border_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_border_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `border` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/border)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_border(&self, border: &str) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_border_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                border: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_border_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            border: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(border);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let border = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(border);
                __widl_f_set_border_HTMLImageElement(self_, border)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sizes_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `sizes` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/sizes)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn sizes(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sizes_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sizes_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sizes_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_sizes_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `sizes` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/sizes)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn set_sizes(&self, sizes: &str) {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_sizes_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sizes: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_sizes_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sizes: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(sizes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sizes = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sizes);
                __widl_f_set_sizes_HTMLImageElement(self_, sizes)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlImageElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_src_HTMLImageElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlImageElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlImageElement {
    #[cfg(all(feature = "HtmlImageElement",))]
    #[allow(bad_style)]
    #[doc = "The `currentSrc` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/currentSrc)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`*"]
    #[allow(clippy::all)]
    pub fn current_src(&self) -> String {
        #[cfg(all(feature = "HtmlImageElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_src_HTMLImageElement(
                self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_src_HTMLImageElement(
            self_: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_src_HTMLImageElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f6af8f99069a2aec: [u8; 3818usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA8\x0E\0\0\0\0+\0\0\x02\x10HTMLImageElement\"__widl_instanceof_HTMLImageElement\0\0\0\0\x12__widl_f_new_Image\x01\0\0\x01\x05Image\0\x01\0\x03new\0\0\0\x1D__widl_f_new_with_width_Image\x01\0\0\x01\x05Image\0\x01\x01\x05width\x03new\0\0\0(__widl_f_new_with_width_and_height_Image\x01\0\0\x01\x05Image\0\x01\x02\x05width\x06height\x03new\0\0\0 __widl_f_decode_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\0\x01\x01\x05self_\x06decode\0\0\0\x1D__widl_f_alt_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x03alt\x01\x01\x05self_\x03alt\0\0\0!__widl_f_set_alt_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x03alt\x01\x02\x05self_\x03alt\x03alt\0\0\0\x1D__widl_f_src_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x03src\x01\x01\x05self_\x03src\0\0\0!__widl_f_set_src_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x03src\x01\x02\x05self_\x03src\x03src\0\0\0 __widl_f_srcset_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x06srcset\x01\x01\x05self_\x06srcset\0\0\0$__widl_f_set_srcset_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x06srcset\x01\x02\x05self_\x06srcset\x06srcset\0\0\0&__widl_f_cross_origin_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x0BcrossOrigin\x01\x01\x05self_\x0BcrossOrigin\0\0\0*__widl_f_set_cross_origin_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x0BcrossOrigin\x01\x02\x05self_\x0Ccross_origin\x0BcrossOrigin\0\0\0!__widl_f_use_map_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x06useMap\x01\x01\x05self_\x06useMap\0\0\0%__widl_f_set_use_map_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x06useMap\x01\x02\x05self_\x07use_map\x06useMap\0\0\0)__widl_f_referrer_policy_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x0EreferrerPolicy\x01\x01\x05self_\x0EreferrerPolicy\0\0\0-__widl_f_set_referrer_policy_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x0EreferrerPolicy\x01\x02\x05self_\x0Freferrer_policy\x0EreferrerPolicy\0\0\0 __widl_f_is_map_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x05isMap\x01\x01\x05self_\x05isMap\0\0\0$__widl_f_set_is_map_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x05isMap\x01\x02\x05self_\x06is_map\x05isMap\0\0\0\x1F__widl_f_width_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0#__widl_f_set_width_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0 __widl_f_height_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0$__widl_f_set_height_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x06height\x01\x02\x05self_\x06height\x06height\0\0\0\"__widl_f_decoding_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x08decoding\x01\x01\x05self_\x08decoding\0\0\0&__widl_f_set_decoding_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x08decoding\x01\x02\x05self_\x08decoding\x08decoding\0\0\0'__widl_f_natural_width_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x0CnaturalWidth\x01\x01\x05self_\x0CnaturalWidth\0\0\0(__widl_f_natural_height_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\rnaturalHeight\x01\x01\x05self_\rnaturalHeight\0\0\0\"__widl_f_complete_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x08complete\x01\x01\x05self_\x08complete\0\0\0\x1E__widl_f_name_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\"__widl_f_set_name_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0\x1F__widl_f_align_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0#__widl_f_set_align_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0 __widl_f_hspace_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x06hspace\x01\x01\x05self_\x06hspace\0\0\0$__widl_f_set_hspace_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x06hspace\x01\x02\x05self_\x06hspace\x06hspace\0\0\0 __widl_f_vspace_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x06vspace\x01\x01\x05self_\x06vspace\0\0\0$__widl_f_set_vspace_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x06vspace\x01\x02\x05self_\x06vspace\x06vspace\0\0\0#__widl_f_long_desc_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x08longDesc\x01\x01\x05self_\x08longDesc\0\0\0'__widl_f_set_long_desc_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x08longDesc\x01\x02\x05self_\tlong_desc\x08longDesc\0\0\0 __widl_f_border_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x06border\x01\x01\x05self_\x06border\0\0\0$__widl_f_set_border_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x06border\x01\x02\x05self_\x06border\x06border\0\0\0\x1F__widl_f_sizes_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\x05sizes\x01\x01\x05self_\x05sizes\0\0\0#__widl_f_set_sizes_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x02\x05sizes\x01\x02\x05self_\x05sizes\x05sizes\0\0\0%__widl_f_current_src_HTMLImageElement\0\0\0\x01\x10HTMLImageElement\x01\0\x01\ncurrentSrc\x01\x01\x05self_\ncurrentSrc\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
