use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLTrackElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlTrackElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlTrackElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlTrackElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlTrackElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlTrackElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlTrackElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlTrackElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlTrackElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlTrackElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlTrackElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlTrackElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlTrackElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlTrackElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlTrackElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlTrackElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlTrackElement {
            HtmlTrackElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlTrackElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlTrackElement> for HtmlTrackElement {
        #[inline]
        fn as_ref(&self) -> &HtmlTrackElement {
            self
        }
    }
    impl From<HtmlTrackElement> for JsValue {
        #[inline]
        fn from(obj: HtmlTrackElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlTrackElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLTrackElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLTrackElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLTrackElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlTrackElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlTrackElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlTrackElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlTrackElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlTrackElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTrackElement> for Element {
    #[inline]
    fn from(obj: HtmlTrackElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlTrackElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTrackElement> for Node {
    #[inline]
    fn from(obj: HtmlTrackElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlTrackElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTrackElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlTrackElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlTrackElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTrackElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlTrackElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlTrackElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlTrackElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kind_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement",))]
    #[allow(bad_style)]
    #[doc = "The `kind` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    #[allow(clippy::all)]
    pub fn kind(&self) -> String {
        #[cfg(all(feature = "HtmlTrackElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kind_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kind_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_kind_HTMLTrackElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTrackElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_kind_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement",))]
    #[allow(bad_style)]
    #[doc = "The `kind` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    #[allow(clippy::all)]
    pub fn set_kind(&self, kind: &str) {
        #[cfg(all(feature = "HtmlTrackElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_kind_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                kind: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_kind_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            kind: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(kind);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let kind = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(kind);
                __widl_f_set_kind_HTMLTrackElement(self_, kind)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTrackElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_src_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    #[allow(clippy::all)]
    pub fn src(&self) -> String {
        #[cfg(all(feature = "HtmlTrackElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_src_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_src_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_src_HTMLTrackElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTrackElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_src_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    #[allow(clippy::all)]
    pub fn set_src(&self, src: &str) {
        #[cfg(all(feature = "HtmlTrackElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_src_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_src_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_set_src_HTMLTrackElement(self_, src)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTrackElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_srclang_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement",))]
    #[allow(bad_style)]
    #[doc = "The `srclang` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    #[allow(clippy::all)]
    pub fn srclang(&self) -> String {
        #[cfg(all(feature = "HtmlTrackElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_srclang_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_srclang_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_srclang_HTMLTrackElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTrackElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_srclang_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement",))]
    #[allow(bad_style)]
    #[doc = "The `srclang` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    #[allow(clippy::all)]
    pub fn set_srclang(&self, srclang: &str) {
        #[cfg(all(feature = "HtmlTrackElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_srclang_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                srclang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_srclang_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            srclang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(srclang);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let srclang = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(srclang);
                __widl_f_set_srclang_HTMLTrackElement(self_, srclang)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTrackElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_label_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement",))]
    #[allow(bad_style)]
    #[doc = "The `label` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    #[allow(clippy::all)]
    pub fn label(&self) -> String {
        #[cfg(all(feature = "HtmlTrackElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_label_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_label_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_label_HTMLTrackElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTrackElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_label_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement",))]
    #[allow(bad_style)]
    #[doc = "The `label` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    #[allow(clippy::all)]
    pub fn set_label(&self, label: &str) {
        #[cfg(all(feature = "HtmlTrackElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_label_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_label_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_set_label_HTMLTrackElement(self_, label)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTrackElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement",))]
    #[allow(bad_style)]
    #[doc = "The `default` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    #[allow(clippy::all)]
    pub fn default(&self) -> bool {
        #[cfg(all(feature = "HtmlTrackElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_HTMLTrackElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTrackElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_default_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement",))]
    #[allow(bad_style)]
    #[doc = "The `default` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    #[allow(clippy::all)]
    pub fn set_default(&self, default: bool) {
        #[cfg(all(feature = "HtmlTrackElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_default_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_default_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            default: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(default);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let default = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(default);
                __widl_f_set_default_HTMLTrackElement(self_, default)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTrackElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/readyState)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> u16 {
        #[cfg(all(feature = "HtmlTrackElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_HTMLTrackElement(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTrackElement", feature = "TextTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_track_HTMLTrackElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTrackElement as WasmDescribe>::describe();
    <Option<TextTrack> as WasmDescribe>::describe();
}
impl HtmlTrackElement {
    #[cfg(all(feature = "HtmlTrackElement", feature = "TextTrack",))]
    #[allow(bad_style)]
    #[doc = "The `track` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/track)\n\n*This API requires the following crate features to be activated: `HtmlTrackElement`, `TextTrack`*"]
    #[allow(clippy::all)]
    pub fn track(&self) -> Option<TextTrack> {
        #[cfg(all(feature = "HtmlTrackElement", feature = "TextTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_track_HTMLTrackElement(
                self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_track_HTMLTrackElement(
            self_: <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTrackElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_track_HTMLTrackElement(self_)
            };
            <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl HtmlTrackElement {
    pub const NONE: u16 = 0i64 as u16;
}
impl HtmlTrackElement {
    pub const LOADING: u16 = 1u64 as u16;
}
impl HtmlTrackElement {
    pub const LOADED: u16 = 2u64 as u16;
}
impl HtmlTrackElement {
    pub const ERROR: u16 = 3u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_0fc97a4ebf2d900b: [u8; 1182usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\\\x04\0\0\0\0\r\0\0\x02\x10HTMLTrackElement\"__widl_instanceof_HTMLTrackElement\0\0\0\0\x1E__widl_f_kind_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x01\x04kind\x01\x01\x05self_\x04kind\0\0\0\"__widl_f_set_kind_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x02\x04kind\x01\x02\x05self_\x04kind\x04kind\0\0\0\x1D__widl_f_src_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x01\x03src\x01\x01\x05self_\x03src\0\0\0!__widl_f_set_src_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x02\x03src\x01\x02\x05self_\x03src\x03src\0\0\0!__widl_f_srclang_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x01\x07srclang\x01\x01\x05self_\x07srclang\0\0\0%__widl_f_set_srclang_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x02\x07srclang\x01\x02\x05self_\x07srclang\x07srclang\0\0\0\x1F__widl_f_label_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x01\x05label\x01\x01\x05self_\x05label\0\0\0#__widl_f_set_label_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x02\x05label\x01\x02\x05self_\x05label\x05label\0\0\0!__widl_f_default_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x01\x07default\x01\x01\x05self_\x07default\0\0\0%__widl_f_set_default_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x02\x07default\x01\x02\x05self_\x07default\x07default\0\0\0%__widl_f_ready_state_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0\x1F__widl_f_track_HTMLTrackElement\0\0\0\x01\x10HTMLTrackElement\x01\0\x01\x05track\x01\x01\x05self_\x05track\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
