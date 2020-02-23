use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLFrameElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlFrameElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlFrameElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlFrameElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(70u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
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
    impl core::ops::Deref for HtmlFrameElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlFrameElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlFrameElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlFrameElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlFrameElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlFrameElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlFrameElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlFrameElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlFrameElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlFrameElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlFrameElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlFrameElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlFrameElement {
            HtmlFrameElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlFrameElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlFrameElement> for HtmlFrameElement {
        #[inline]
        fn as_ref(&self) -> &HtmlFrameElement {
            self
        }
    }
    impl From<HtmlFrameElement> for JsValue {
        #[inline]
        fn from(obj: HtmlFrameElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlFrameElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLFrameElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLFrameElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLFrameElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlFrameElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlFrameElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlFrameElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlFrameElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlFrameElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFrameElement> for Element {
    #[inline]
    fn from(obj: HtmlFrameElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlFrameElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFrameElement> for Node {
    #[inline]
    fn from(obj: HtmlFrameElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlFrameElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFrameElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlFrameElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlFrameElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFrameElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlFrameElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlFrameElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLFrameElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scrolling_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `scrolling` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/scrolling)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn scrolling(&self) -> String {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scrolling_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scrolling_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scrolling_HTMLFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_scrolling_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `scrolling` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/scrolling)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_scrolling(&self, scrolling: &str) {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_scrolling_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scrolling: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_scrolling_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scrolling: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(scrolling);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scrolling = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scrolling);
                __widl_f_set_scrolling_HTMLFrameElement(self_, scrolling)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_src_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/src)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn src(&self) -> String {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_src_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_src_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_src_HTMLFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_src_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/src)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_src(&self, src: &str) {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_src_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_src_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_set_src_HTMLFrameElement(self_, src)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_frame_border_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `frameBorder` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/frameBorder)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn frame_border(&self) -> String {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_frame_border_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_frame_border_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_frame_border_HTMLFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_frame_border_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `frameBorder` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/frameBorder)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_frame_border(&self, frame_border: &str) {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_frame_border_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                frame_border: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_frame_border_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            frame_border: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(frame_border);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let frame_border =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(frame_border);
                __widl_f_set_frame_border_HTMLFrameElement(self_, frame_border)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_long_desc_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `longDesc` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn long_desc(&self) -> String {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_long_desc_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_long_desc_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_long_desc_HTMLFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_long_desc_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `longDesc` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_long_desc(&self, long_desc: &str) {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_long_desc_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                long_desc: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_long_desc_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let long_desc = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(long_desc);
                __widl_f_set_long_desc_HTMLFrameElement(self_, long_desc)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_no_resize_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `noResize` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/noResize)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn no_resize(&self) -> bool {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_no_resize_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_no_resize_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_no_resize_HTMLFrameElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_no_resize_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `noResize` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/noResize)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_no_resize(&self, no_resize: bool) {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_no_resize_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                no_resize: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_no_resize_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            no_resize: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(no_resize);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let no_resize = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(no_resize);
                __widl_f_set_no_resize_HTMLFrameElement(self_, no_resize)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_content_document_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <Option<Document> as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "Document", feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `contentDocument` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/contentDocument)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn content_document(&self) -> Option<Document> {
        #[cfg(all(feature = "Document", feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_content_document_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_content_document_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_content_document_HTMLFrameElement(self_)
            };
            <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_content_window_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <Option<Window> as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `contentWindow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/contentWindow)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`, `Window`*"]
    #[allow(clippy::all)]
    pub fn content_window(&self) -> Option<Window> {
        #[cfg(all(feature = "HtmlFrameElement", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_content_window_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_content_window_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_content_window_HTMLFrameElement(self_)
            };
            <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_margin_height_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `marginHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginHeight)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn margin_height(&self) -> String {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_margin_height_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_margin_height_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_margin_height_HTMLFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_margin_height_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `marginHeight` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginHeight)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_margin_height(&self, margin_height: &str) {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_margin_height_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                margin_height: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_margin_height_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            margin_height: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(margin_height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let margin_height =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(margin_height);
                __widl_f_set_margin_height_HTMLFrameElement(self_, margin_height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_margin_width_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `marginWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginWidth)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn margin_width(&self) -> String {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_margin_width_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_margin_width_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_margin_width_HTMLFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_margin_width_HTMLFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameElement {
    #[cfg(all(feature = "HtmlFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `marginWidth` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginWidth)\n\n*This API requires the following crate features to be activated: `HtmlFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_margin_width(&self, margin_width: &str) {
        #[cfg(all(feature = "HtmlFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_margin_width_HTMLFrameElement(
                self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                margin_width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_margin_width_HTMLFrameElement(
            self_: <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            margin_width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(margin_width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let margin_width =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(margin_width);
                __widl_f_set_margin_width_HTMLFrameElement(self_, margin_width)
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
pub static __WASM_BINDGEN_GENERATED_d6ff5a85c408c1ad: [u8; 1916usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}:\x07\0\0\0\0\x13\0\0\x02\x10HTMLFrameElement\"__widl_instanceof_HTMLFrameElement\0\0\0\0\x1E__widl_f_name_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\"__widl_f_set_name_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0#__widl_f_scrolling_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x01\tscrolling\x01\x01\x05self_\tscrolling\0\0\0'__widl_f_set_scrolling_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x02\tscrolling\x01\x02\x05self_\tscrolling\tscrolling\0\0\0\x1D__widl_f_src_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x01\x03src\x01\x01\x05self_\x03src\0\0\0!__widl_f_set_src_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x02\x03src\x01\x02\x05self_\x03src\x03src\0\0\0&__widl_f_frame_border_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x01\x0BframeBorder\x01\x01\x05self_\x0BframeBorder\0\0\0*__widl_f_set_frame_border_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x02\x0BframeBorder\x01\x02\x05self_\x0Cframe_border\x0BframeBorder\0\0\0#__widl_f_long_desc_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x01\x08longDesc\x01\x01\x05self_\x08longDesc\0\0\0'__widl_f_set_long_desc_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x02\x08longDesc\x01\x02\x05self_\tlong_desc\x08longDesc\0\0\0#__widl_f_no_resize_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x01\x08noResize\x01\x01\x05self_\x08noResize\0\0\0'__widl_f_set_no_resize_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x02\x08noResize\x01\x02\x05self_\tno_resize\x08noResize\0\0\0*__widl_f_content_document_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x01\x0FcontentDocument\x01\x01\x05self_\x0FcontentDocument\0\0\0(__widl_f_content_window_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x01\rcontentWindow\x01\x01\x05self_\rcontentWindow\0\0\0'__widl_f_margin_height_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x01\x0CmarginHeight\x01\x01\x05self_\x0CmarginHeight\0\0\0+__widl_f_set_margin_height_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x02\x0CmarginHeight\x01\x02\x05self_\rmargin_height\x0CmarginHeight\0\0\0&__widl_f_margin_width_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x01\x0BmarginWidth\x01\x01\x05self_\x0BmarginWidth\0\0\0*__widl_f_set_margin_width_HTMLFrameElement\0\0\0\x01\x10HTMLFrameElement\x01\0\x02\x0BmarginWidth\x01\x02\x05self_\x0Cmargin_width\x0BmarginWidth\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
