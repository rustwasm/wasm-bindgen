use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLSourceElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlSourceElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlSourceElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlSourceElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(83u32);
            inform(111u32);
            inform(117u32);
            inform(114u32);
            inform(99u32);
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
    impl core::ops::Deref for HtmlSourceElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlSourceElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlSourceElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlSourceElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlSourceElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlSourceElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlSourceElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlSourceElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlSourceElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlSourceElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlSourceElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlSourceElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlSourceElement {
            HtmlSourceElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlSourceElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlSourceElement> for HtmlSourceElement {
        #[inline]
        fn as_ref(&self) -> &HtmlSourceElement {
            self
        }
    }
    impl From<HtmlSourceElement> for JsValue {
        #[inline]
        fn from(obj: HtmlSourceElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlSourceElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLSourceElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLSourceElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLSourceElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlSourceElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlSourceElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlSourceElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlSourceElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlSourceElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlSourceElement> for Element {
    #[inline]
    fn from(obj: HtmlSourceElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlSourceElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlSourceElement> for Node {
    #[inline]
    fn from(obj: HtmlSourceElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlSourceElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlSourceElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlSourceElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlSourceElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlSourceElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlSourceElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlSourceElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlSourceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_src_HTMLSourceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSourceElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlSourceElement {
    #[cfg(all(feature = "HtmlSourceElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    #[allow(clippy::all)]
    pub fn src(&self) -> String {
        #[cfg(all(feature = "HtmlSourceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_src_HTMLSourceElement(
                self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_src_HTMLSourceElement(
            self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_src_HTMLSourceElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSourceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_src_HTMLSourceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSourceElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSourceElement {
    #[cfg(all(feature = "HtmlSourceElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    #[allow(clippy::all)]
    pub fn set_src(&self, src: &str) {
        #[cfg(all(feature = "HtmlSourceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_src_HTMLSourceElement(
                self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_src_HTMLSourceElement(
            self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_set_src_HTMLSourceElement(self_, src)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSourceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLSourceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSourceElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlSourceElement {
    #[cfg(all(feature = "HtmlSourceElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlSourceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLSourceElement(
                self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLSourceElement(
            self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLSourceElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSourceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_HTMLSourceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSourceElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSourceElement {
    #[cfg(all(feature = "HtmlSourceElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "HtmlSourceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_HTMLSourceElement(
                self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_HTMLSourceElement(
            self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_HTMLSourceElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSourceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_srcset_HTMLSourceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSourceElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlSourceElement {
    #[cfg(all(feature = "HtmlSourceElement",))]
    #[allow(bad_style)]
    #[doc = "The `srcset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    #[allow(clippy::all)]
    pub fn srcset(&self) -> String {
        #[cfg(all(feature = "HtmlSourceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_srcset_HTMLSourceElement(
                self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_srcset_HTMLSourceElement(
            self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_srcset_HTMLSourceElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSourceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_srcset_HTMLSourceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSourceElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSourceElement {
    #[cfg(all(feature = "HtmlSourceElement",))]
    #[allow(bad_style)]
    #[doc = "The `srcset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    #[allow(clippy::all)]
    pub fn set_srcset(&self, srcset: &str) {
        #[cfg(all(feature = "HtmlSourceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_srcset_HTMLSourceElement(
                self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                srcset: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_srcset_HTMLSourceElement(
            self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let srcset = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(srcset);
                __widl_f_set_srcset_HTMLSourceElement(self_, srcset)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSourceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sizes_HTMLSourceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSourceElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlSourceElement {
    #[cfg(all(feature = "HtmlSourceElement",))]
    #[allow(bad_style)]
    #[doc = "The `sizes` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    #[allow(clippy::all)]
    pub fn sizes(&self) -> String {
        #[cfg(all(feature = "HtmlSourceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sizes_HTMLSourceElement(
                self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sizes_HTMLSourceElement(
            self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sizes_HTMLSourceElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSourceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_sizes_HTMLSourceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSourceElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSourceElement {
    #[cfg(all(feature = "HtmlSourceElement",))]
    #[allow(bad_style)]
    #[doc = "The `sizes` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    #[allow(clippy::all)]
    pub fn set_sizes(&self, sizes: &str) {
        #[cfg(all(feature = "HtmlSourceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_sizes_HTMLSourceElement(
                self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sizes: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_sizes_HTMLSourceElement(
            self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sizes = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sizes);
                __widl_f_set_sizes_HTMLSourceElement(self_, sizes)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSourceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_HTMLSourceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSourceElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlSourceElement {
    #[cfg(all(feature = "HtmlSourceElement",))]
    #[allow(bad_style)]
    #[doc = "The `media` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    #[allow(clippy::all)]
    pub fn media(&self) -> String {
        #[cfg(all(feature = "HtmlSourceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_HTMLSourceElement(
                self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_HTMLSourceElement(
            self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_HTMLSourceElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSourceElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_media_HTMLSourceElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSourceElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSourceElement {
    #[cfg(all(feature = "HtmlSourceElement",))]
    #[allow(bad_style)]
    #[doc = "The `media` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    #[allow(clippy::all)]
    pub fn set_media(&self, media: &str) {
        #[cfg(all(feature = "HtmlSourceElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_media_HTMLSourceElement(
                self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                media: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_media_HTMLSourceElement(
            self_: <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            media: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(media);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlSourceElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let media = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(media);
                __widl_f_set_media_HTMLSourceElement(self_, media)
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
pub static __WASM_BINDGEN_GENERATED_7f6bd90be3313459: [u8; 1010usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB0\x03\0\0\0\0\x0B\0\0\x02\x11HTMLSourceElement#__widl_instanceof_HTMLSourceElement\0\0\0\0\x1E__widl_f_src_HTMLSourceElement\0\0\0\x01\x11HTMLSourceElement\x01\0\x01\x03src\x01\x01\x05self_\x03src\0\0\0\"__widl_f_set_src_HTMLSourceElement\0\0\0\x01\x11HTMLSourceElement\x01\0\x02\x03src\x01\x02\x05self_\x03src\x03src\0\0\0\x1F__widl_f_type_HTMLSourceElement\0\0\0\x01\x11HTMLSourceElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0#__widl_f_set_type_HTMLSourceElement\0\0\0\x01\x11HTMLSourceElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0!__widl_f_srcset_HTMLSourceElement\0\0\0\x01\x11HTMLSourceElement\x01\0\x01\x06srcset\x01\x01\x05self_\x06srcset\0\0\0%__widl_f_set_srcset_HTMLSourceElement\0\0\0\x01\x11HTMLSourceElement\x01\0\x02\x06srcset\x01\x02\x05self_\x06srcset\x06srcset\0\0\0 __widl_f_sizes_HTMLSourceElement\0\0\0\x01\x11HTMLSourceElement\x01\0\x01\x05sizes\x01\x01\x05self_\x05sizes\0\0\0$__widl_f_set_sizes_HTMLSourceElement\0\0\0\x01\x11HTMLSourceElement\x01\0\x02\x05sizes\x01\x02\x05self_\x05sizes\x05sizes\0\0\0 __widl_f_media_HTMLSourceElement\0\0\0\x01\x11HTMLSourceElement\x01\0\x01\x05media\x01\x01\x05self_\x05media\0\0\0$__widl_f_set_media_HTMLSourceElement\0\0\0\x01\x11HTMLSourceElement\x01\0\x02\x05media\x01\x02\x05self_\x05media\x05media\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
