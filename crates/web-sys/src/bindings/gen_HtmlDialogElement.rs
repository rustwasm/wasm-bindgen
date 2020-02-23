use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLDialogElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlDialogElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlDialogElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlDialogElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(68u32);
            inform(105u32);
            inform(97u32);
            inform(108u32);
            inform(111u32);
            inform(103u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlDialogElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlDialogElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlDialogElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlDialogElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlDialogElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlDialogElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlDialogElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlDialogElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlDialogElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlDialogElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlDialogElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlDialogElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlDialogElement {
            HtmlDialogElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlDialogElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlDialogElement> for HtmlDialogElement {
        #[inline]
        fn as_ref(&self) -> &HtmlDialogElement {
            self
        }
    }
    impl From<HtmlDialogElement> for JsValue {
        #[inline]
        fn from(obj: HtmlDialogElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlDialogElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLDialogElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLDialogElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLDialogElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlDialogElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlDialogElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlDialogElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlDialogElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlDialogElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlDialogElement> for Element {
    #[inline]
    fn from(obj: HtmlDialogElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlDialogElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlDialogElement> for Node {
    #[inline]
    fn from(obj: HtmlDialogElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlDialogElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlDialogElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlDialogElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlDialogElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlDialogElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlDialogElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlDialogElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlDialogElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_HTMLDialogElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDialogElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDialogElement {
    #[cfg(all(feature = "HtmlDialogElement",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "HtmlDialogElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_HTMLDialogElement(
                self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_HTMLDialogElement(
            self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_HTMLDialogElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDialogElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_with_return_value_HTMLDialogElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDialogElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDialogElement {
    #[cfg(all(feature = "HtmlDialogElement",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    #[allow(clippy::all)]
    pub fn close_with_return_value(&self, return_value: &str) {
        #[cfg(all(feature = "HtmlDialogElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_with_return_value_HTMLDialogElement(
                self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                return_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_with_return_value_HTMLDialogElement(
            self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            return_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(return_value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let return_value =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(return_value);
                __widl_f_close_with_return_value_HTMLDialogElement(self_, return_value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDialogElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_show_HTMLDialogElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDialogElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDialogElement {
    #[cfg(all(feature = "HtmlDialogElement",))]
    #[allow(bad_style)]
    #[doc = "The `show()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/show)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    #[allow(clippy::all)]
    pub fn show(&self) {
        #[cfg(all(feature = "HtmlDialogElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_show_HTMLDialogElement(
                self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_show_HTMLDialogElement(
            self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_show_HTMLDialogElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDialogElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_show_modal_HTMLDialogElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDialogElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDialogElement {
    #[cfg(all(feature = "HtmlDialogElement",))]
    #[allow(bad_style)]
    #[doc = "The `showModal()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/showModal)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    #[allow(clippy::all)]
    pub fn show_modal(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlDialogElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_show_modal_HTMLDialogElement(
                self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_show_modal_HTMLDialogElement(
            self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_show_modal_HTMLDialogElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlDialogElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_HTMLDialogElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDialogElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlDialogElement {
    #[cfg(all(feature = "HtmlDialogElement",))]
    #[allow(bad_style)]
    #[doc = "The `open` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    #[allow(clippy::all)]
    pub fn open(&self) -> bool {
        #[cfg(all(feature = "HtmlDialogElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_HTMLDialogElement(
                self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_HTMLDialogElement(
            self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_open_HTMLDialogElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlDialogElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_open_HTMLDialogElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDialogElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDialogElement {
    #[cfg(all(feature = "HtmlDialogElement",))]
    #[allow(bad_style)]
    #[doc = "The `open` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    #[allow(clippy::all)]
    pub fn set_open(&self, open: bool) {
        #[cfg(all(feature = "HtmlDialogElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_open_HTMLDialogElement(
                self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_open_HTMLDialogElement(
            self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            open: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(open);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let open = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(open);
                __widl_f_set_open_HTMLDialogElement(self_, open)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlDialogElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_return_value_HTMLDialogElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlDialogElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlDialogElement {
    #[cfg(all(feature = "HtmlDialogElement",))]
    #[allow(bad_style)]
    #[doc = "The `returnValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    #[allow(clippy::all)]
    pub fn return_value(&self) -> String {
        #[cfg(all(feature = "HtmlDialogElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_return_value_HTMLDialogElement(
                self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_return_value_HTMLDialogElement(
            self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_return_value_HTMLDialogElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlDialogElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_return_value_HTMLDialogElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlDialogElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlDialogElement {
    #[cfg(all(feature = "HtmlDialogElement",))]
    #[allow(bad_style)]
    #[doc = "The `returnValue` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)\n\n*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
    #[allow(clippy::all)]
    pub fn set_return_value(&self, return_value: &str) {
        #[cfg(all(feature = "HtmlDialogElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_return_value_HTMLDialogElement(
                self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                return_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_return_value_HTMLDialogElement(
            self_: <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            return_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(return_value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlDialogElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let return_value =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(return_value);
                __widl_f_set_return_value_HTMLDialogElement(self_, return_value)
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
pub static __WASM_BINDGEN_GENERATED_58cb82b2794518bf: [u8; 883usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}1\x03\0\0\0\0\t\0\0\x02\x11HTMLDialogElement#__widl_instanceof_HTMLDialogElement\0\0\0\0 __widl_f_close_HTMLDialogElement\0\0\0\x01\x11HTMLDialogElement\x01\0\0\x01\x01\x05self_\x05close\0\0\02__widl_f_close_with_return_value_HTMLDialogElement\0\0\0\x01\x11HTMLDialogElement\x01\0\0\x01\x02\x05self_\x0Creturn_value\x05close\0\0\0\x1F__widl_f_show_HTMLDialogElement\0\0\0\x01\x11HTMLDialogElement\x01\0\0\x01\x01\x05self_\x04show\0\0\0%__widl_f_show_modal_HTMLDialogElement\x01\0\0\x01\x11HTMLDialogElement\x01\0\0\x01\x01\x05self_\tshowModal\0\0\0\x1F__widl_f_open_HTMLDialogElement\0\0\0\x01\x11HTMLDialogElement\x01\0\x01\x04open\x01\x01\x05self_\x04open\0\0\0#__widl_f_set_open_HTMLDialogElement\0\0\0\x01\x11HTMLDialogElement\x01\0\x02\x04open\x01\x02\x05self_\x04open\x04open\0\0\0'__widl_f_return_value_HTMLDialogElement\0\0\0\x01\x11HTMLDialogElement\x01\0\x01\x0BreturnValue\x01\x01\x05self_\x0BreturnValue\0\0\0+__widl_f_set_return_value_HTMLDialogElement\0\0\0\x01\x11HTMLDialogElement\x01\0\x02\x0BreturnValue\x01\x02\x05self_\x0Creturn_value\x0BreturnValue\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
