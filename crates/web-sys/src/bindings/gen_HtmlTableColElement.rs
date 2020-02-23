use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLTableColElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlTableColElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlTableColElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlTableColElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(84u32);
            inform(97u32);
            inform(98u32);
            inform(108u32);
            inform(101u32);
            inform(67u32);
            inform(111u32);
            inform(108u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlTableColElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlTableColElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlTableColElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlTableColElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlTableColElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlTableColElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlTableColElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlTableColElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlTableColElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlTableColElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlTableColElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlTableColElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlTableColElement {
            HtmlTableColElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlTableColElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlTableColElement> for HtmlTableColElement {
        #[inline]
        fn as_ref(&self) -> &HtmlTableColElement {
            self
        }
    }
    impl From<HtmlTableColElement> for JsValue {
        #[inline]
        fn from(obj: HtmlTableColElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlTableColElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLTableColElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLTableColElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLTableColElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlTableColElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlTableColElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlTableColElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlTableColElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlTableColElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableColElement> for Element {
    #[inline]
    fn from(obj: HtmlTableColElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlTableColElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableColElement> for Node {
    #[inline]
    fn from(obj: HtmlTableColElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlTableColElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableColElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlTableColElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlTableColElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableColElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlTableColElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlTableColElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_span_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `span` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/span)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn span(&self) -> u32 {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_span_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_span_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_span_HTMLTableColElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_span_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `span` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/span)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn set_span(&self, span: u32) {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_span_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                span: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_span_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            span: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(span);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let span = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(span);
                __widl_f_set_span_HTMLTableColElement(self_, span)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_HTMLTableColElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLTableColElement(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ch_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `ch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn ch(&self) -> String {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ch_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ch_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ch_HTMLTableColElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ch_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `ch` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn set_ch(&self, ch: &str) {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ch_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ch: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ch_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ch: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ch);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ch = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ch);
                __widl_f_set_ch_HTMLTableColElement(self_, ch)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ch_off_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `chOff` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn ch_off(&self) -> String {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ch_off_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ch_off_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ch_off_HTMLTableColElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ch_off_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `chOff` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn set_ch_off(&self, ch_off: &str) {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ch_off_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ch_off: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ch_off_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ch_off: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ch_off);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ch_off = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ch_off);
                __widl_f_set_ch_off_HTMLTableColElement(self_, ch_off)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_v_align_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `vAlign` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn v_align(&self) -> String {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_v_align_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_v_align_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_v_align_HTMLTableColElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_v_align_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `vAlign` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn set_v_align(&self, v_align: &str) {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_v_align_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                v_align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_v_align_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            v_align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(v_align);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let v_align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(v_align);
                __widl_f_set_v_align_HTMLTableColElement(self_, v_align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> String {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_HTMLTableColElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableColElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_HTMLTableColElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableColElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableColElement {
    #[cfg(all(feature = "HtmlTableColElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: &str) {
        #[cfg(all(feature = "HtmlTableColElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_HTMLTableColElement(
                self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_HTMLTableColElement(
            self_: <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableColElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_HTMLTableColElement(self_, width)
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
pub static __WASM_BINDGEN_GENERATED_bcd2f23159e033c6: [u8; 1232usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x8E\x04\0\0\0\0\r\0\0\x02\x13HTMLTableColElement%__widl_instanceof_HTMLTableColElement\0\0\0\0!__widl_f_span_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x01\x04span\x01\x01\x05self_\x04span\0\0\0%__widl_f_set_span_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x02\x04span\x01\x02\x05self_\x04span\x04span\0\0\0\"__widl_f_align_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0&__widl_f_set_align_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0\x1F__widl_f_ch_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x01\x02ch\x01\x01\x05self_\x02ch\0\0\0#__widl_f_set_ch_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x02\x02ch\x01\x02\x05self_\x02ch\x02ch\0\0\0#__widl_f_ch_off_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x01\x05chOff\x01\x01\x05self_\x05chOff\0\0\0'__widl_f_set_ch_off_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x02\x05chOff\x01\x02\x05self_\x06ch_off\x05chOff\0\0\0$__widl_f_v_align_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x01\x06vAlign\x01\x01\x05self_\x06vAlign\0\0\0(__widl_f_set_v_align_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x02\x06vAlign\x01\x02\x05self_\x07v_align\x06vAlign\0\0\0\"__widl_f_width_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0&__widl_f_set_width_HTMLTableColElement\0\0\0\x01\x13HTMLTableColElement\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
