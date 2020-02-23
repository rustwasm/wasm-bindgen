use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLScriptElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlScriptElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlScriptElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlScriptElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(83u32);
            inform(99u32);
            inform(114u32);
            inform(105u32);
            inform(112u32);
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
    impl core::ops::Deref for HtmlScriptElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlScriptElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlScriptElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlScriptElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlScriptElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlScriptElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlScriptElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlScriptElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlScriptElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlScriptElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlScriptElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlScriptElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlScriptElement {
            HtmlScriptElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlScriptElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlScriptElement> for HtmlScriptElement {
        #[inline]
        fn as_ref(&self) -> &HtmlScriptElement {
            self
        }
    }
    impl From<HtmlScriptElement> for JsValue {
        #[inline]
        fn from(obj: HtmlScriptElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlScriptElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLScriptElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLScriptElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLScriptElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlScriptElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlScriptElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlScriptElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlScriptElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlScriptElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlScriptElement> for Element {
    #[inline]
    fn from(obj: HtmlScriptElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlScriptElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlScriptElement> for Node {
    #[inline]
    fn from(obj: HtmlScriptElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlScriptElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlScriptElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlScriptElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlScriptElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlScriptElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlScriptElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlScriptElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_src_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/src)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn src(&self) -> String {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_src_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_src_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_src_HTMLScriptElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_src_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/src)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn set_src(&self, src: &str) {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_src_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_src_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_set_src_HTMLScriptElement(self_, src)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/type)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLScriptElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/type)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_HTMLScriptElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_no_module_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `noModule` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/noModule)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn no_module(&self) -> bool {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_no_module_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_no_module_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_no_module_HTMLScriptElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_no_module_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `noModule` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/noModule)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn set_no_module(&self, no_module: bool) {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_no_module_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                no_module: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_no_module_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            no_module: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(no_module);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let no_module = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(no_module);
                __widl_f_set_no_module_HTMLScriptElement(self_, no_module)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_charset_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `charset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/charset)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn charset(&self) -> String {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_charset_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_charset_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_charset_HTMLScriptElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_charset_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `charset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/charset)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn set_charset(&self, charset: &str) {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_charset_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                charset: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_charset_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            charset: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(charset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let charset = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(charset);
                __widl_f_set_charset_HTMLScriptElement(self_, charset)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_async_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `async` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/async)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn r#async(&self) -> bool {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_async_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_async_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_async_HTMLScriptElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_async_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `async` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/async)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn set_async(&self, r#async: bool) {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_async_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                r#async: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_async_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            r#async: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(r#async);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let r#async = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r#async);
                __widl_f_set_async_HTMLScriptElement(self_, r#async)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_defer_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `defer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/defer)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn defer(&self) -> bool {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_defer_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_defer_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_defer_HTMLScriptElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_defer_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `defer` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/defer)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn set_defer(&self, defer: bool) {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_defer_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                defer: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_defer_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            defer: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(defer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let defer = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(defer);
                __widl_f_set_defer_HTMLScriptElement(self_, defer)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cross_origin_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `crossOrigin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn cross_origin(&self) -> Option<String> {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cross_origin_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cross_origin_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cross_origin_HTMLScriptElement(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cross_origin_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `crossOrigin` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn set_cross_origin(&self, cross_origin: Option<&str>) {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cross_origin_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cross_origin: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cross_origin_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cross_origin =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cross_origin);
                __widl_f_set_cross_origin_HTMLScriptElement(self_, cross_origin)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `text` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/text)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn text(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_HTMLScriptElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `text` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/text)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn set_text(&self, text: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_set_text_HTMLScriptElement(self_, text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_event_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `event` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/event)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn event(&self) -> String {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_event_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_event_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_event_HTMLScriptElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_event_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `event` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/event)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn set_event(&self, event: &str) {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_event_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_event_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(event);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let event = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(event);
                __widl_f_set_event_HTMLScriptElement(self_, event)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_html_for_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `htmlFor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/htmlFor)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn html_for(&self) -> String {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_html_for_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_html_for_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_html_for_HTMLScriptElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_html_for_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `htmlFor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/htmlFor)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn set_html_for(&self, html_for: &str) {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_html_for_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                html_for: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_html_for_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            html_for: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(html_for);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let html_for = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(html_for);
                __widl_f_set_html_for_HTMLScriptElement(self_, html_for)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_integrity_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `integrity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/integrity)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn integrity(&self) -> String {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_integrity_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_integrity_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_integrity_HTMLScriptElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlScriptElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_integrity_HTMLScriptElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlScriptElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlScriptElement {
    #[cfg(all(feature = "HtmlScriptElement",))]
    #[allow(bad_style)]
    #[doc = "The `integrity` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/integrity)\n\n*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    #[allow(clippy::all)]
    pub fn set_integrity(&self, integrity: &str) {
        #[cfg(all(feature = "HtmlScriptElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_integrity_HTMLScriptElement(
                self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                integrity: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_integrity_HTMLScriptElement(
            self_: <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            integrity: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(integrity);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlScriptElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let integrity = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(integrity);
                __widl_f_set_integrity_HTMLScriptElement(self_, integrity)
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
pub static __WASM_BINDGEN_GENERATED_6d18b4fed32bf6e1: [u8; 2158usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"},\x08\0\0\0\0\x17\0\0\x02\x11HTMLScriptElement#__widl_instanceof_HTMLScriptElement\0\0\0\0\x1E__widl_f_src_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x01\x03src\x01\x01\x05self_\x03src\0\0\0\"__widl_f_set_src_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x02\x03src\x01\x02\x05self_\x03src\x03src\0\0\0\x1F__widl_f_type_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0#__widl_f_set_type_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0$__widl_f_no_module_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x01\x08noModule\x01\x01\x05self_\x08noModule\0\0\0(__widl_f_set_no_module_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x02\x08noModule\x01\x02\x05self_\tno_module\x08noModule\0\0\0\"__widl_f_charset_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x01\x07charset\x01\x01\x05self_\x07charset\0\0\0&__widl_f_set_charset_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x02\x07charset\x01\x02\x05self_\x07charset\x07charset\0\0\0 __widl_f_async_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x01\x05async\x01\x01\x05self_\x05async\0\0\0$__widl_f_set_async_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x02\x05async\x01\x02\x05self_\x07r#async\x05async\0\0\0 __widl_f_defer_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x01\x05defer\x01\x01\x05self_\x05defer\0\0\0$__widl_f_set_defer_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x02\x05defer\x01\x02\x05self_\x05defer\x05defer\0\0\0'__widl_f_cross_origin_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x01\x0BcrossOrigin\x01\x01\x05self_\x0BcrossOrigin\0\0\0+__widl_f_set_cross_origin_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x02\x0BcrossOrigin\x01\x02\x05self_\x0Ccross_origin\x0BcrossOrigin\0\0\0\x1F__widl_f_text_HTMLScriptElement\x01\0\0\x01\x11HTMLScriptElement\x01\0\x01\x04text\x01\x01\x05self_\x04text\0\0\0#__widl_f_set_text_HTMLScriptElement\x01\0\0\x01\x11HTMLScriptElement\x01\0\x02\x04text\x01\x02\x05self_\x04text\x04text\0\0\0 __widl_f_event_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x01\x05event\x01\x01\x05self_\x05event\0\0\0$__widl_f_set_event_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x02\x05event\x01\x02\x05self_\x05event\x05event\0\0\0#__widl_f_html_for_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x01\x07htmlFor\x01\x01\x05self_\x07htmlFor\0\0\0'__widl_f_set_html_for_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x02\x07htmlFor\x01\x02\x05self_\x08html_for\x07htmlFor\0\0\0$__widl_f_integrity_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x01\tintegrity\x01\x01\x05self_\tintegrity\0\0\0(__widl_f_set_integrity_HTMLScriptElement\0\0\0\x01\x11HTMLScriptElement\x01\0\x02\tintegrity\x01\x02\x05self_\tintegrity\tintegrity\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
